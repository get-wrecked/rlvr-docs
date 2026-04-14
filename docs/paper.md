# Paper Summary

**Title**: Scaling Intelligence Through Verifiable Rewards: A Three-Stage Framework for Training General Agents

## Abstract

A framework for training general-purpose AI agents using Reinforcement Learning with Verifiable Rewards (RLVR) across 2,697 diverse domains. Organized around a three-stage theory of intelligence:

1. **Rule Recognition** — learning to infer latent structure from synthetic and formal environments
2. **System Mastery** — applying rule recognition to language, code, mathematics, science, and strategic reasoning
3. **Capability Climbing** — hill-climbing performance on specific, high-value applications

Each stage uses programmatically verifiable rewards requiring no human judgment. The framework includes 2,697 environments, 13 Rust-based verifiers (227 tests, validated against 6M real problems), and a curriculum controller managing difficulty across all domains.

## Key Contributions

1. **272-domain taxonomy** of verifiable reward environments spanning every branch of human knowledge where programmatic verification is feasible
2. **13 Rust verifiers** with 227 tests validated against 6 million real problems
3. **Dataset registry** of 33 downloaded datasets (2.3GB, ~6M problems) plus 7 procedural generators producing unlimited data
4. **Training harness architecture** with Rust verifier server, Python GRPO loop, curriculum controller, and TensorBoard reporting
5. **Concrete training protocol** with phase-by-phase instructions and compute estimates

## The Scaling Argument

### Why RLVR Scales to AGI

1. **Data is unlimited**: ~6M internet problems + unlimited procedural generation
2. **Verification is cheap**: Nanoseconds (exact match) to seconds (code execution) — 1000x cheaper than response generation
3. **Difficulty is controllable**: Each domain scales from trivial to superhuman
4. **Domains are independent**: Adding a domain = writing a verifier
5. **Transfer is multiplicative**: Cross-domain training creates compounding effects (validated by NCA paper's 1.4-1.6x multiplier)

### Compute to AGI

~250,000 H100-hours (~$750K at cloud pricing) for initial AGI-level breadth across 2,697 domains.

## Key References

- **DeepSeek-R1** (2024): RLVR on math + code produces strong reasoning via GRPO with binary rewards
- **NCA Pre-Pre-Training** (Lee et al., 2025): 164M tokens of synthetic cellular automata data outperformed 1.6B tokens of Common Crawl — pure rule inference transfers everywhere
- **AlphaProof / AlphaGeometry** (DeepMind, 2024): RLVR for formal mathematics
- **veRL, OpenRLHF, TRL**: Open-source RL frameworks; all use inline Python verifiers vs. our Rust verifier server

## The Theory in Detail

Intelligence follows a universal pattern:
1. Recognize the rules of the game
2. Strengthen visibility on high-leverage areas
3. Maximize curves in those areas
4. Climb curves toward mastery
5. Repeat across new domains

The NCA paper's key insight: with no semantic shortcuts (zero linguistic content), models are forced to develop **pure rule inference in attention layers**. This is the foundational capability that transfers to everything.

### Stage Mapping

| NCA Finding | Our Framework | Implication |
|-------------|--------------|-------------|
| Synthetic data develops attention-layer rule inference | Stage 1: ~55 synthetic/formal domains | Train rule recognition first |
| Transfer to language, code, math | Stage 2: ~110 applied domains | Rule recognition accelerates system mastery |
| Complexity matching varies by target | Difficulty curriculum per domain | Different domains benefit from different rule complexities |
| 1.4-1.6x convergence acceleration | Multiplied across 2,697 domains | Compounding returns from breadth |
