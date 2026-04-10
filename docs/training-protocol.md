# Training Protocol

A concrete, phase-by-phase guide from policy initialization to AGI-level breadth.

## Phase 0: Policy Initialization

Load a pre-trained vision-language model (VLM) as the initial policy.

### Base Model Requirements
- Multimodal capability (vision + text)
- Long context (32K+ tokens)
- Tool use / function calling
- Pre-existing language and code capability

### Candidate Base Models
| Model | Parameters | Strengths |
|-------|-----------|-----------|
| Qwen2-VL | 7B-72B | Strong vision-language, tool use |
| LLaVA-OneVision | 7B-34B | Open multimodal model |
| InternVL | 8B-76B | Competitive open VLM |
| Llama 3.1 + vision adapter | 8B-405B | Meta's base + added vision |

### Optional NCA Pre-Pre-Training
Before RL, train on 164M+ tokens of Neural Cellular Automata data to bootstrap attention-layer rule inference. This accelerates all subsequent stages by 1.4-1.6x (Lee et al., 2025).

### Optional Custom Pre-Training Mix
If training from scratch instead of using an existing model:
- 40% web text (FineWeb-Edu, 1.3T tokens)
- 25% code (The Stack v2, 67TB)
- 15% science/math (ProofPile-2 + peS2o)
- 10% books/encyclopedias (Dolma)
- 10% multilingual (ROOTS)

### SFT Bridge
After pre-training, supervised fine-tuning on 2-3M examples:
- General: Tulu 2 Mix (326K) + OpenHermes 2.5 (1M) + FLAN (1,836 tasks)
- Math reasoning: NuminaMath-CoT (860K) + MAmmoTH (260K)
- Code: Magicoder-OSS (75K) + CodeFeedback (66K)

## Phase 1: Stage 1 RLVR (Months 1-2)

### Domains (~55 active)
- Synthetic rule inference (NCA, grammar, ARC-like)
- Formal logic (SAT, SMT, Boolean functions)
- Basic math (numerical, linear algebra)
- Basic code (function generation, regex)
- Basic perception (classification, OCR)

### Hyperparameters
| Parameter | Value |
|-----------|-------|
| Algorithm | GRPO |
| Generations per prompt | 4-8 |
| Reward type | Binary (0 or 1) |
| Learning rate | 1e-6 → 5e-7 (cosine decay) |
| Batch size | 64-256 |
| Max new tokens | 2048 |
| Gradient accumulation | 8 |

### Curriculum
- Start at difficulty 1 across all domains
- Target 40-60% success rate
- Auto-scale difficulty as agent improves
- Equal time across domains initially

### Expected Outcome
Strong foundational reasoning. The model can infer rules from examples, track state, do basic logic and arithmetic, write simple code. All Stage 2 learning will be faster because the computational primitives are in place.

### Compute
~5,000 H100-hours over 1-2 months.

## Phase 2: Stage 2 RLVR (Months 2-6)

### Domains (~110 new, ~165 total)
- All language understanding (NLI, QA, IE, summarization)
- Full code capability (SQL, competitive programming, testing)
- Full mathematics (competition, formal proofs, modeling)
- Science and engineering (physics, chemistry, biology)
- Vision and multimodal (VQA, charts, documents)
- Agent tasks (web navigation, computer use)
- Strategic reasoning (games, planning)

### Curriculum
| Source | Compute Share |
|--------|-------------|
| Stage 1 (maintenance) | 10% |
| Stage 2 (active learning) | 80% |
| Stage 3 (early exposure) | 10% |

### Domain Mixing
- Start with equal time across domains
- Shift to capability-weighted sampling (more time on weaker domains)
- Every domain maintains ≥1% minimum sampling rate

### Expected Outcome
Broad, general competence. The model can understand language, write code, solve math, reason about science, navigate websites, follow instructions, answer questions — all at a competent level.

### Compute
~50,000 H100-hours over 3-4 months.

## Phase 3: Stage 3 RLVR (Months 6-12)

### Domains (~80 new, ~245 total)
- Molecular generation, protein design
- CTF challenges, security hardening
- Medical diagnosis, legal reasoning
- Advanced compiler optimization
- Time series forecasting, supply chain optimization

### Curriculum
| Source | Compute Share |
|--------|-------------|
| Stage 1 (maintenance) | 5% |
| Stage 2 (maintenance + deepening) | 30% |
| Stage 3 (pushing frontiers) | 65% |

### Difficulty Scaling
Within each Stage 3 domain, push difficulty from 5 → 8 → 10 (approaching superhuman on benchmark evaluations).

### Expected Outcome
Expert-level capability in dozens of specialized domains while maintaining broad Stage 2 competence. The model can diagnose diseases (USMLE level), solve CTF challenges, design molecules, write Lean4 proofs, optimize supply chains.

### Compute
~200,000 H100-hours over 6 months.

## Phase 4: Unified AGI Training (Months 12+)

All 272 environments simultaneously at all difficulty levels:
- Dynamic curriculum across all three stages
- Difficulty within each domain scales from trivial to superhuman
- Procedural generation ensures no data exhaustion
- Cross-domain transfer emerges from simultaneous training

The curriculum controller maintains ~40-60% success rate in every domain by adjusting difficulty, ensuring the agent is always learning at the frontier of its capability in every domain simultaneously.

### Compute
~500,000+ H100-hours (ongoing).

## Total Compute Estimate

| Phase | Duration | Compute | Cost (cloud) |
|-------|----------|---------|-------------|
| Phase 0 (NCA) | 1 week | ~500 H100-hours | ~$1,500 |
| Phase 1 | 1-2 months | ~5K H100-hours | ~$15,000 |
| Phase 2 | 3-4 months | ~50K H100-hours | ~$150,000 |
| Phase 3 | 6 months | ~200K H100-hours | ~$600,000 |
| Phase 4 | Ongoing | ~500K+ H100-hours | ~$1.5M+ |

**Total to initial AGI-level breadth: ~250K H100-hours (~$750K).**

## Expected Performance Trajectory

| Benchmark | Baseline (SFT) | Phase 1 | Phase 2 | Phase 3 |
|-----------|----------------|---------|---------|---------|
| GSM8K | 50% | 65% | 80% | 90%+ |
| MATH | 30% | 45% | 65% | 80%+ |
| HumanEval | 40% | 55% | 70% | 85%+ |
| MMLU | 60% | 65% | 75% | 85%+ |
| IFEval | 40% | 60% | 80% | 90%+ |
| WebArena | 5% | 10% | 25% | 40%+ |
| SWE-bench | 10% | 15% | 30% | 45%+ |

These are conservative — they don't account for cross-domain transfer effects, which the NCA paper suggests could be substantial.

## Key Principles

1. **Never run out of data**: Between internet datasets (~6M problems) and procedural generation (unlimited), there is no data wall
2. **Verification is cheap**: Running a verifier (nanoseconds for exact match, seconds for code execution) is 1000x cheaper than generating a response
3. **Difficulty is controllable**: Within each domain, difficulty scales from trivial to superhuman
4. **Domains are independent**: Adding a new domain requires only writing a new verifier
5. **Transfer is multiplicative**: Training on 272 diverse domains creates compounding transfer effects

## Limitations

1. **Verification gap**: Some domains (translation, summarization) have imperfect verification — used as supplementary signal only
2. **Reward hacking**: More domains = more attack surface. Per-domain anti-hacking mitigations needed
3. **Infrastructure complexity**: 272 verifiers, each with its own pipeline
4. **Compute requirements**: Full protocol requires substantial GPU resources
