---
title: RLVR Training Harness Architecture
description: Unified architecture for multi-domain verifiable reward training with pre-trained policy loading
---

# RLVR Training Harness Architecture

## Design Goals

1. **Unified interface** across all 243 domains — one architecture, one training loop
2. **Load pre-trained vision/text policy** (e.g., a base VLM) and fine-tune with RL
3. **Rust verifiers** for speed, called from Python training loop via FFI or subprocess
4. **TensorBoard/W&B reporting** per-domain and aggregate
5. **Claude (or any LLM) can interact** to monitor progress, adjust curriculum, debug failures
6. **Sandboxed execution** for code/agent domains
7. **Curriculum controller** that manages difficulty across domains

## Architecture Overview

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
           │   (Rust — HTTP/gRPC)  │
           │                       │
           │  ┌─────────────────┐  │
           │  │  Domain Router  │  │
           │  └────────┬────────┘  │
           │           │           │
           │  ┌────────▼────────┐  │
           │  │ Verifier Pool   │  │
           │  │                 │  │
           │  │ math_numerical  │  │
           │  │ sudoku          │  │
           │  │ regex_synthesis │  │
           │  │ code_execution  │  │
           │  │ ...243 domains  │  │
           │  └─────────────────┘  │
           │                       │
           │  ┌─────────────────┐  │
           │  │ Sandbox Pool    │  │
           │  │ (for code exec) │  │
           │  └─────────────────┘  │
           └───────────────────────┘
```

## Core Data Types

```rust
/// A task to present to the policy
struct Task {
    id: String,
    domain: String,           // "math_numerical", "sudoku", etc.
    stage: Stage,             // Pre/Mid/Post
    difficulty: u8,           // 1-10
    prompt: String,           // Text prompt for the policy
    images: Vec<Image>,       // Optional visual context
    metadata: serde_json::Value,  // Domain-specific data (puzzle grid, test cases, etc.)
}

/// What the policy produces
struct Response {
    task_id: String,
    text: String,             // The model's response
    tokens_used: usize,
}

/// What the verifier returns
struct Reward {
    task_id: String,
    domain: String,
    score: f64,               // 0.0 to 1.0
    reason: String,           // Human-readable explanation
    metadata: serde_json::Value,  // Domain-specific details
}

enum Stage {
    Pre,   // Stage 1: Rule recognition
    Mid,   // Stage 2: General systems
    Post,  // Stage 3: Specific climbing
}
```

## Verifier Server (Rust)

The verifier runs as a separate HTTP service for several reasons:
1. **Isolation**: Code execution sandboxing is easier in a separate process
2. **Performance**: Rust verifiers are fast; can handle batch verification
3. **Language independence**: Training loop can be Python, Swift, whatever
4. **Scaling**: Can run multiple verifier instances for throughput

### API

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
    ...
]
→ [{ "score": 1.0, ... }, { "score": 0.0, ... }, ...]

POST /sample
{
    "domain": "math_numerical",    // or "random" for curriculum sampling
    "difficulty": 5,
    "count": 32
}
→ [{ task1 }, { task2 }, ...]

GET /health
→ { "domains": 243, "status": "ok" }

GET /stats
→ { per-domain accuracy, throughput, difficulty distribution }
```

### Verifier Registry

```rust
/// Trait that every domain verifier implements
trait Verifier: Send + Sync {
    /// Domain name (e.g., "math_numerical")
    fn domain(&self) -> &str;
    
    /// Verify a response against a task
    fn verify(&self, task: &serde_json::Value, response: &str) -> VerifyResult;
    
    /// Sample a new task at the given difficulty
    fn sample_task(&self, difficulty: u8, rng: &mut impl Rng) -> Task;
    
    /// Which training stage this domain belongs to
    fn stage(&self) -> Stage;
}

/// Registry holds all domain verifiers
struct VerifierRegistry {
    verifiers: HashMap<String, Box<dyn Verifier>>,
}
```

## Curriculum Controller

The curriculum controller decides WHAT tasks to sample and at WHAT difficulty, based on the policy's current performance.

```python
class CurriculumController:
    def __init__(self, domains: list[str], initial_difficulty: int = 1):
        self.domain_stats = {d: DomainStats() for d in domains}
        self.stage_weights = {Stage.PRE: 0.4, Stage.MID: 0.4, Stage.POST: 0.2}
    
    def sample_batch(self, batch_size: int) -> list[TaskRequest]:
        """Sample a batch of tasks across domains and difficulties."""
        tasks = []
        for _ in range(batch_size):
            # 1. Sample stage according to weights
            stage = self._sample_stage()
            
            # 2. Within stage, sample domain (weighted by capability gap)
            domain = self._sample_domain(stage)
            
            # 3. Within domain, sample difficulty (target ~40-60% success rate)
            difficulty = self._target_difficulty(domain)
            
            tasks.append(TaskRequest(domain=domain, difficulty=difficulty))
        return tasks
    
    def update(self, results: list[Reward]):
        """Update stats after a batch of verifications."""
        for result in results:
            stats = self.domain_stats[result.domain]
            stats.add(result.score, result.difficulty)
            
            # Adjust difficulty to maintain target success rate
            if stats.recent_success_rate() > 0.7:
                stats.target_difficulty = min(10, stats.target_difficulty + 1)
            elif stats.recent_success_rate() < 0.3:
                stats.target_difficulty = max(1, stats.target_difficulty - 1)
    
    def _target_difficulty(self, domain: str) -> int:
        return self.domain_stats[domain].target_difficulty
```

## Reporting & Monitoring

### TensorBoard Metrics (logged every N steps)

**Aggregate:**
- `reward/mean` — mean reward across all domains
- `reward/by_stage/{pre,mid,post}` — mean reward per stage
- `curriculum/difficulty_distribution` — histogram of sampled difficulties
- `curriculum/domain_distribution` — which domains are being sampled

**Per-domain (top-level group per domain):**
- `{domain}/reward_mean` — running mean reward
- `{domain}/reward_std` — reward variance
- `{domain}/success_rate` — fraction scoring > 0.5
- `{domain}/difficulty` — current target difficulty
- `{domain}/examples_seen` — total tasks in this domain

**Diagnostics:**
- `verifier/latency_ms` — verification latency
- `verifier/batch_throughput` — tasks verified per second
- `policy/tokens_per_task` — response length distribution

### Claude Interaction Points

Claude (me) can interact with this system at several points:

1. **Review failure cases**: After each eval, dump the worst-performing examples. I can read these and diagnose whether the verifier has a bug or the model genuinely got it wrong.

2. **Adjust curriculum**: If a domain is stuck (success rate flat for many steps), I can propose: change difficulty, adjust sampling weight, or examine what specific sub-skills are failing.

3. **Verify verifier correctness**: I can periodically run test suites and spot-check edge cases in the verifiers against known correct answers.

4. **Analyze reward distributions**: Look at score histograms per domain to detect reward hacking or degenerate behavior.

The practical interface: a CLI or webhook that dumps stats + example failures, which I review and provide recommendations.

## Pre-trained Policy Loading

The system is designed to load a pre-trained VLM (vision-language model) as the initial policy:

```python
# Example: load a pre-trained policy and begin RLVR
from transformers import AutoModelForCausalLM, AutoProcessor

# Load pre-trained base (e.g., Qwen2-VL, LLaVA, or a custom model)
model = AutoModelForCausalLM.from_pretrained("base-vlm-checkpoint")
processor = AutoProcessor.from_pretrained("base-vlm-checkpoint")

# Wrap in RL trainer
from trl import GRPOTrainer, GRPOConfig

config = GRPOConfig(
    num_generations=4,           # Generate 4 responses per task
    max_new_tokens=2048,
    learning_rate=1e-6,
    per_device_train_batch_size=4,
    gradient_accumulation_steps=8,
)

# The reward function calls our Rust verifier server
def reward_fn(prompts, responses, tasks):
    results = verifier_client.verify_batch(tasks, responses)
    return [r.score for r in results]

trainer = GRPOTrainer(
    model=model,
    config=config,
    reward_funcs=[reward_fn],
    train_dataset=curriculum_dataset,  # Dynamically sampled by curriculum controller
)

trainer.train()
```

## Verification-on-the-Way Strategy

How I (Claude) verify the system is working correctly during training:

### Before Training (Setup Validation)
1. Run full test suite on all Rust verifiers (`cargo test`)
2. For each verifier, run 100 procedurally generated tasks and check:
   - Correct answers always score 1.0
   - Random garbage always scores 0.0  
   - Known wrong answers score 0.0
   - Partial credit is between 0 and 1

### During Training (Continuous Monitoring)
1. **Spot-check rewards**: Every 1000 steps, sample 10 tasks per domain, manually verify the reward makes sense
2. **Reward distribution health**: If any domain shows >90% success rate at max difficulty, suspect reward hacking
3. **Loss monitoring**: If policy loss stops decreasing but reward increases, suspect degenerate behavior
4. **Diversity check**: Verify the policy isn't collapsing to a single answer pattern

### After Training (Evaluation)
1. Run standard benchmarks (GSM8K, HumanEval, MMLU) to compare to baselines
2. Run per-domain held-out evaluation sets
3. Compute correlation between verifier reward and human judgment on a small sample

## File Structure

```
rlvr/
├── Cargo.toml                    # Rust project
├── src/
│   ├── main.rs                   # HTTP server entry point
│   ├── server.rs                 # Actix-web/Axum HTTP server
│   ├── registry.rs               # Verifier registry
│   ├── verifiers/
│   │   ├── mod.rs                # VerifyResult + Verifier trait
│   │   ├── math_numerical.rs     # ✅ Built (26 tests)
│   │   ├── sudoku.rs             # ✅ Built (16 tests)
│   │   ├── regex_synthesis.rs    # ✅ Built (13 tests)
│   │   ├── code_execution.rs     # TODO: sandbox + test runner
│   │   ├── json_schema.rs        # TODO: schema validation
│   │   ├── constraint_sat.rs     # TODO: general constraint checker
│   │   └── ... (243 total)
│   └── datasets/
│       ├── mod.rs
│       ├── gsm8k.rs              # GSM8K loader
│       └── ...
├── python/
│   ├── client.py                 # Python client for verifier server
│   ├── curriculum.py             # Curriculum controller
│   ├── train.py                  # GRPO training loop (uses TRL)
│   └── eval.py                   # Evaluation script
├── wiki/                         # Knowledge base (existing)
├── raw/                          # Datasets (existing)
└── configs/
    ├── domains.toml              # Per-domain config (difficulty, weight, stage)
    └── training.toml             # Training hyperparameters
```

## Build Order

Priority order for building verifiers (maximizing capability gain per effort):

### Phase 1: Core (do first — enables basic RLVR loop)
1. ✅ math_numerical (exact match extraction)
2. ✅ sudoku (constraint satisfaction)
3. ✅ regex_synthesis (compilation + example testing)
4. json_schema (structured output validation)
5. instruction_following (multi-constraint checking)
6. code_execution (sandbox + test runner — hardest infra)

### Phase 2: Expand
7. unit_conversion, date_time (simple computation)
8. spelling_grammar (diff-based)
9. text_classification (label matching)
10. fact_verification (label matching)
11. question_answering (exact/F1 matching)

### Phase 3: Agent & Vision
12. web_navigation (headless browser state checking)
13. visual_qa (exact match on multimodal input)
14. chart_understanding
15. code_repair (test suite execution variant)

### Phase 4: Science & Engineering
16. physics_simulation (numerical comparison)
17. chemistry_computation (RDKit wrappers)
18. circuit_design (SPICE integration)
