---
title: Scaling Roadmap
description: Path from current RLVR to AGI-scale training across all verifiable domains
---

# Scaling Roadmap: RLVR to AGI

## Current State of the Art (2024-2025)

### What Labs Are Doing Now
- **DeepSeek-R1**: RLVR on math (MATH, AIME) + code (competitive programming). Demonstrated that RLVR alone (no SFT) can produce strong reasoning.
- **OpenAI o1/o3**: Rumored RLVR on math + code + science. Chain-of-thought RL.
- **Qwen2.5-Math**: RLVR specifically for mathematics.
- **AlphaProof/AlphaGeometry**: Formal math (Lean) + geometry with RLVR (AlphaGo-style).
- **StarCoder2/CodeLlama**: Code-focused but mostly SFT, limited RLVR.

### Current Domain Coverage
Most systems train on **2-5 domains**: math, code, sometimes science QA.

### The Gap
The gap between current practice and the spec in this wiki is enormous. Current systems use <5% of the verifiable reward surface area available. This is not a research gap — it's an engineering gap.

## Phase 1: Consolidate the Foundation (Months 1-3)

### Domains: ~10 core domains
- Math competition (MATH, AMC/AIME, Olympiad)
- Math numerical (GSM8K, AMPS)
- Formal proofs (Lean4/mathlib)
- Code generation (HumanEval, APPS, CodeContests)
- Code repair (SWE-bench)
- SQL generation (Spider, BIRD)
- Logic puzzles (Sudoku, constraint satisfaction)
- Factual QA (TriviaQA, Natural Questions)
- Chess puzzles
- Instruction following (IFEval)

### Infrastructure Needed
- Multi-domain training harness with per-domain reward normalization
- Sandboxed code execution (already standard)
- Lean4 proof checker integration
- Constraint satisfaction checker library
- Chess engine integration (Stockfish)
- Difficulty-based curriculum controller

### Expected Capability
Strong reasoning + coding + factual recall + instruction following. Competitive with current frontier models on standard benchmarks.

## Phase 2: Broaden the Reward Surface (Months 3-8)

### Add ~40 domains from Tier 2
**Formal & Logic**:
- Propositional logic / SAT
- First-order logic / ATP
- Combinatorial optimization
- Formal specification (TLA+)
- Graph theory

**Code Expansion**:
- Regex synthesis
- Shell commands
- Type inference
- Data wrangling
- API usage
- Infrastructure as code

**Science**:
- Physics simulation
- Chemistry computation
- Circuit design
- Bioinformatics

**Language**:
- Information extraction
- Entity linking
- Table understanding
- Semantic parsing
- Reading comprehension

**Vision**:
- Visual QA (VQAv2, GQA)
- Document OCR
- Chart understanding
- Math with diagrams

### Infrastructure Needed
- Physics simulation integration (PyBullet)
- Chemistry toolkit integration (RDKit)
- Circuit simulator integration (SPICE)
- Headless browser for web rendering
- Image processing pipeline for vision tasks
- 30+ domain-specific verifier implementations

### Expected Capability
Dramatically broader reasoning. Science capability. Vision understanding. Multi-hop reasoning. The model can now do things that math+code alone cannot teach.

## Phase 3: Agentic and Interactive (Months 8-14)

### Add ~30 domains from Tier 3
**Agent domains**:
- Web navigation (WebArena)
- Computer use (OSWorld)
- GUI navigation (Android/iOS)
- File system manipulation
- Spreadsheet tasks
- Interactive fiction

**Complex reasoning**:
- Multi-step chemistry (retrosynthesis)
- Control system design
- Engineering optimization
- Competitive programming (interactive)
- Map navigation

**Creative-constrained**:
- Constrained writing
- HTML/CSS from visual spec
- Music theory
- Cryptography challenges
- Crossword construction

### Infrastructure Needed
- Full VM sandboxes for computer use
- Browser automation framework
- Mobile device emulators
- Multi-step environment wrappers (reset, step, reward)
- Long-horizon reward aggregation

### Expected Capability
Genuine agentic capability. Can navigate websites, use computers, solve multi-step problems that require planning. Can operate in interactive environments, not just single-turn Q&A.

## Phase 4: Curriculum Unification (Months 14-20)

### All ~121+ domains simultaneously
- Dynamic difficulty curriculum across all domains
- Agent trains on a mixture of all domains every day
- Difficulty scales from trivial to frontier-research-level within each domain
- Procedural generation ensures no data exhaustion
- Multi-modal mixing: text-only, vision+text, interactive

### Architecture
```
                    ┌──────────────────┐
                    │  Unified Agent    │
                    │  (Vision + Text)  │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │  Task Router     │
                    │  (domain mixing) │
                    └────────┬─────────┘
                             │
          ┌──────────────────┼──────────────────┐
          │                  │                  │
   ┌──────▼──────┐  ┌───────▼──────┐  ┌───────▼──────┐
   │ Math/Logic  │  │ Code/Agent   │  │ Science/     │
   │ Verifiers   │  │ Sandboxes    │  │ Simulators   │  ...x121
   └─────────────┘  └──────────────┘  └──────────────┘
```

### Difficulty Scaling Strategy
Within each domain, maintain 10 difficulty levels:
- **Level 1**: Trivial (grade school math, FizzBuzz)
- **Level 2-3**: Easy (basic textbook)
- **Level 4-5**: Medium (undergraduate)
- **Level 6-7**: Hard (graduate/professional)
- **Level 8-9**: Expert (competition-level, research-level)
- **Level 10**: Superhuman (open problems, unsolved challenges)

The curriculum controller keeps the agent at ~40-60% success rate by adjusting the difficulty mix.

### Expected Capability
AGI-level breadth. The model can reason formally, write code, do science, navigate computers, solve puzzles, extract information, follow instructions — all at expert level. The combination of 121+ domains with superhuman difficulty scaling creates capability far beyond what any single domain could develop.

## Phase 5: Self-Improving (Months 20+)

### New Capabilities
- Agent generates new RLVR tasks for itself (meta-learning)
- Agent proposes new verification functions
- Agent discovers new domains
- Agent improves its own training curriculum

This is where RLVR meets recursive self-improvement. The agent's ability to generate verifiable challenges for itself is the key to open-ended capability growth.

## Key Technical Decisions

### Training Algorithm
- **GRPO** (Group Relative Policy Optimization): Current best for RLVR. Used by DeepSeek.
- **PPO**: Standard but requires critic model.
- **REINFORCE with baseline**: Simplest, surprisingly effective.
- **STaR/ReST**: Rejection sampling + SFT on correct answers. Simpler alternative to RL.

Recommendation: Start with GRPO, fall back to STaR if RL is unstable.

### Base Model
- Needs to be multimodal (vision + text) from the start
- Needs long context for complex tasks (32K+)
- Needs tool use capability for agentic domains
- Open-weight preferred for rapid iteration

### Compute Estimate
- Phase 1: ~1000 H100-hours (small by modern standards)
- Phase 2: ~10,000 H100-hours
- Phase 3: ~50,000 H100-hours (agent training is expensive)
- Phase 4: ~500,000 H100-hours
- Phase 5: Open-ended

The verification compute (running tests, simulations, etc.) is 10-100x cheaper than model training compute, so it's not the bottleneck.

## Risk Factors

1. **Reward hacking at scale**: More domains = more attack surface. Need robust per-domain hacking mitigations.
2. **Catastrophic forgetting**: Training on 121 domains may cause forgetting. Need careful mixing.
3. **Verification fidelity**: Some domains (translation, summarization) have imperfect verification. Don't over-weight these.
4. **Infrastructure complexity**: 121 different verifiers, sandboxes, simulators. Significant engineering.
5. **Difficulty calibration**: Getting the curriculum right across 121 domains is hard. Need automated difficulty estimation.
