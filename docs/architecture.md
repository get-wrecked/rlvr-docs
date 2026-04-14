# System Architecture

## Overview

The RLVR training system has four major components:

1. **Policy (VLM)** — The vision-language model being trained
2. **GRPO Trainer** — The reinforcement learning training loop (Python/TRL)
3. **Verifier Server** — Rust HTTP service that scores responses (13 verifiers, 2,697 domains)
4. **Curriculum Controller** — Manages difficulty and domain mixing across training

```
┌─────────────────────────────────────────────────────────────┐
│                    TRAINING ORCHESTRATOR                     │
│              (Python — manages the RL loop)                  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌────────────┐  ┌──────────┐ │
│  │  Policy   │  │   GRPO   │  │  Curriculum │  │ Reporter │ │
│  │  (VLM)   │  │  Trainer │  │  Controller │  │ (TB/W&B) │ │
│  └────┬─────┘  └────┬─────┘  └─────┬──────┘  └────┬─────┘ │
│       │              │              │               │       │
│       └──────────────┼──────────────┘               │       │
│                      │                              │       │
│              ┌───────▼────────┐                     │       │
│              │   Task Sampler │◄────────────────────┘       │
│              │ (domain mixer) │                              │
│              └───────┬────────┘                              │
└──────────────────────┼──────────────────────────────────────┘
                       │
           ┌───────────▼───────────┐
           │    VERIFIER SERVER    │
           │   (Rust — HTTP)       │
           │                       │
           │  ┌─────────────────┐  │
           │  │  Domain Router  │  │
           │  └────────┬────────┘  │
           │           │           │
           │  ┌────────▼────────┐  │
           │  │ Verifier Pool   │  │
           │  │ 13 verifiers    │  │
           │  │ 2,697 domains     │  │
           │  └─────────────────┘  │
           │                       │
           │  ┌─────────────────┐  │
           │  │ Sandbox Pool    │  │
           │  │ (for code exec) │  │
           │  └─────────────────┘  │
           └───────────────────────┘
```

## Why a Separate Rust Verifier Server?

All existing RLVR frameworks (DeepSeek-R1, veRL, OpenRLHF, TRL) use inline Python functions with regex-based extraction and no service boundary. Our Rust server approach provides:

1. **Isolation**: Code execution sandboxing is easier in a separate process
2. **Performance**: Rust verifiers handle batch verification at high throughput
3. **Language independence**: Training loop can be Python, Swift, anything
4. **Scaling**: Can run multiple verifier instances for throughput
5. **Reliability**: 227 tests ensure verifier correctness

## Core Data Types

```rust
/// A task to present to the policy
struct Task {
    id: String,
    domain: String,               // "math_numerical", "sudoku", etc.
    stage: Stage,                 // Pre/Mid/Post
    difficulty: u8,               // 1-10
    prompt: String,               // Text prompt for the policy
    images: Vec<Image>,           // Optional visual context
    metadata: serde_json::Value,  // Domain-specific data
}

/// What the policy produces
struct Response {
    task_id: String,
    text: String,
    tokens_used: usize,
}

/// What the verifier returns
struct Reward {
    task_id: String,
    domain: String,
    score: f64,          // 0.0 to 1.0
    reason: String,      // Human-readable explanation
    metadata: serde_json::Value,
}

enum Stage {
    Pre,   // Stage 1: Rule recognition
    Mid,   // Stage 2: General systems
    Post,  // Stage 3: Specific climbing
}
```

## Verifier Server API

```
POST /verify
{
    "domain": "math_numerical",
    "task": { ... task-specific data ... },
    "response": "The answer is 42"
}
→ { "score": 1.0, "reason": "correct" }

POST /verify/batch
[
    { "domain": "math_numerical", "task": ..., "response": ... },
    { "domain": "sudoku", "task": ..., "response": ... },
]
→ [{ "score": 1.0, ... }, { "score": 0.0, ... }]

POST /sample
{
    "domain": "math_numerical",
    "difficulty": 5,
    "count": 32
}
→ [task1, task2, ...]

GET /health
→ { "domains": 272, "status": "ok" }

GET /stats
→ { per-domain accuracy, throughput, difficulty distribution }
```

## Curriculum Controller

The curriculum controller decides what tasks to sample and at what difficulty, targeting ~40-60% success rate in each domain.

```python
class CurriculumController:
    def __init__(self, domains: list[str], initial_difficulty: int = 1):
        self.domain_stats = {d: DomainStats() for d in domains}
        self.stage_weights = {Stage.PRE: 0.4, Stage.MID: 0.4, Stage.POST: 0.2}

    def sample_batch(self, batch_size: int) -> list[TaskRequest]:
        """Sample a batch of tasks across domains and difficulties."""
        tasks = []
        for _ in range(batch_size):
            stage = self._sample_stage()
            domain = self._sample_domain(stage)  # weighted by capability gap
            difficulty = self._target_difficulty(domain)
            tasks.append(TaskRequest(domain=domain, difficulty=difficulty))
        return tasks

    def update(self, results: list[Reward]):
        """Update stats after a batch of verifications."""
        for result in results:
            stats = self.domain_stats[result.domain]
            stats.add(result.score, result.difficulty)
            if stats.recent_success_rate() > 0.7:
                stats.target_difficulty = min(10, stats.target_difficulty + 1)
            elif stats.recent_success_rate() < 0.3:
                stats.target_difficulty = max(1, stats.target_difficulty - 1)
```

### Stage Weight Progression

| Training Phase | Stage 1 | Stage 2 | Stage 3 |
|---------------|---------|---------|---------|
| Phase 1 | 80% | 15% | 5% |
| Phase 2 | 10% | 80% | 10% |
| Phase 3 | 5% | 30% | 65% |
| Phase 4 | 5% | 30% | 65% |

## Training Loop (GRPO)

```python
from transformers import AutoModelForCausalLM, AutoProcessor
from trl import GRPOTrainer, GRPOConfig

# Load pre-trained VLM
model = AutoModelForCausalLM.from_pretrained("base-vlm-checkpoint")
processor = AutoProcessor.from_pretrained("base-vlm-checkpoint")

config = GRPOConfig(
    num_generations=4,            # 4 responses per task
    max_new_tokens=2048,
    learning_rate=1e-6,
    per_device_train_batch_size=4,
    gradient_accumulation_steps=8,
)

# Reward function calls the Rust verifier server
def reward_fn(prompts, responses, tasks):
    results = verifier_client.verify_batch(tasks, responses)
    return [r.score for r in results]

trainer = GRPOTrainer(
    model=model,
    config=config,
    reward_funcs=[reward_fn],
    train_dataset=curriculum_dataset,
)
trainer.train()
```

## Monitoring & Reporting

### TensorBoard Metrics

**Aggregate:**
- `reward/mean` — Mean reward across all domains
- `reward/by_stage/{pre,mid,post}` — Mean reward per stage
- `curriculum/difficulty_distribution` — Histogram of sampled difficulties
- `curriculum/domain_distribution` — Which domains are being sampled

**Per-domain:**
- `{domain}/reward_mean` — Running mean reward
- `{domain}/success_rate` — Fraction scoring > 0.5
- `{domain}/difficulty` — Current target difficulty
- `{domain}/examples_seen` — Total tasks seen

**Diagnostics:**
- `verifier/latency_ms` — Verification latency
- `verifier/batch_throughput` — Tasks verified per second
- `policy/tokens_per_task` — Response length distribution

## Validation Protocol

### Before Training
1. `cargo test` — all 227 verifier tests pass
2. For each verifier, run 100 procedurally generated tasks:
   - Correct answers always score 1.0
   - Random garbage always scores 0.0
   - Known wrong answers score 0.0

### During Training
1. Spot-check rewards every 1000 steps
2. Monitor for reward hacking (>90% success at max difficulty)
3. Check policy diversity (not collapsing to single answer patterns)

### After Training
1. Standard benchmarks (GSM8K, HumanEval, MMLU)
2. Per-domain held-out evaluation
3. Cross-domain transfer evaluation
