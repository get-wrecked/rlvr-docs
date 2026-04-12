# RLVR Environments: A Comprehensive Taxonomy for AGI Training

## The Core Thesis

Reinforcement Learning with Verifiable Rewards (RLVR) is the most scalable path to AGI because it eliminates the bottleneck of human preference labels. If you can programmatically verify whether an output is correct, you can generate unlimited training signal. The key insight: **the internet already contains millions of problem-solution pairs across hundreds of domains, and for many of these domains, verification is fully automatable.**

This wiki catalogs every domain where RLVR is feasible, with the goal of building a maximally diverse training curriculum that develops general intelligence through breadth of verifiable challenge.

## Why Breadth Matters

A model trained only on math and code develops narrow reasoning. AGI requires:

- **Formal reasoning**: proofs, logic, constraint satisfaction
- **Empirical reasoning**: science, engineering, data analysis  
- **Perceptual reasoning**: vision, spatial, multimodal
- **Agentic capability**: web navigation, computer use, tool use
- **Language mastery**: extraction, parsing, translation, generation
- **Creative problem-solving**: puzzles, games, novel challenges
- **Instruction following**: precise adherence to complex specifications

RLVR across all these axes simultaneously develops the full spectrum of cognitive capability.

## Taxonomy

### Tier 1: Battle-Tested (Proven in RLVR papers, massive datasets)
These domains have been used in published RLVR systems and have well-understood verification:

- **Mathematics**: Competition math, numerical computation, formal theorem proving
- **Code Generation**: From unit-test-verified coding problems (HumanEval → CodeContests → SWE-bench)
- **SQL**: Text-to-SQL with execution-based verification
- **Logic Puzzles**: Constraint satisfaction with programmatic verification
- **Games**: Chess, Go, board games with clear win conditions
- **Factual QA**: Closed-form question answering with exact match

### Tier 2: Strong Infrastructure (Clear verification, large datasets, not yet widely used in RLVR)
These have all the ingredients but haven't been widely exploited for RLVR training:

- **Formal Logic**: SAT/SMT solving, first-order logic, model checking
- **Science Computation**: Physics, chemistry, biology with simulation-based verification
- **Information Extraction**: NER, relation extraction verified against knowledge bases
- **Web Navigation**: Task completion verified by DOM state checking
- **Visual QA**: Questions about images with exact-match answers
- **Regex/Shell/Config**: Domain-specific code generation with execution verification
- **Data Wrangling**: Pandas/SQL transformations with output comparison
- **Structured Output**: JSON/XML generation with schema validation

### Tier 3: Emerging (Verification possible but requires care)
These require more sophisticated verification or have partial coverage:

- **Computer Use**: General OS interaction verified by system state
- **GUI Navigation**: Mobile/desktop app interaction
- **Circuit/Control Design**: Hardware simulation verification
- **Retrosynthesis**: Chemical synthesis planning against reaction databases
- **Constrained Writing**: Text satisfying formal constraints
- **HTML/CSS Generation**: Visual diff against reference renders
- **Instruction Following**: Multi-constraint instruction adherence

### Tier 4: Approximate Verification (Useful but imperfect signal)
These have automated metrics that correlate with correctness but aren't binary/exact:

- **Translation**: BLEU/chrF against references (useful, imperfect)
- **Summarization**: ROUGE scores (very imperfect, but usable as partial signal)
- **Visual Grounding**: IoU thresholds (reasonable approximation)
- **Chart/Document Understanding**: Partial match metrics
- **Music Composition**: Rule-based theory checking (partial)
- **ASCII Art**: Structural pattern matching (limited)

## The Verification Spectrum

From strongest to weakest:

1. **Formal proof checking** — Lean/Coq type-checker. Zero false positives.
2. **Execution + test suite** — Code runs and all tests pass. Near-zero false positives if tests are good.
3. **Exact match** — Output matches known answer exactly. Zero false positives, possible false negatives (alternative correct answers).
4. **Constraint satisfaction** — All constraints verified programmatically. Zero false positives.
5. **Simulation-based** — Physics/circuit/control simulation confirms specs. Very reliable within simulation fidelity.
6. **Schema validation** — Output conforms to formal schema. True for structure, doesn't verify content.
7. **Metric-based** — BLEU, ROUGE, IoU above threshold. Noisy signal, correlates with quality.

For AGI training, we should **maximize time in categories 1-5** and use 6-7 as supplementary signal.

## Scale Estimates

| Category | Domains | Estimated Total Problems | Verification Quality |
|----------|---------|------------------------|---------------------|
| Math & Logic | ~16 | 10M+ (generatable to ∞) | Excellent |
| Code & Software | ~17 | 5M+ (generatable to ∞) | Excellent |
| Science & Engineering | ~16 | 1M+ (generatable) | Good-Excellent |
| Language & Knowledge | ~22 | 100M+ (from internet) | Good |
| Games & Agents | ~25 | ∞ (self-play, procedural) | Excellent |
| Format & Misc | ~25 | ∞ (generatable) | Good |
| **Total** | **~121** | **Effectively unlimited** | **Mostly excellent** |

## Path to AGI

The roadmap is:
1. **Phase 1**: Train on Tier 1 domains (math + code + QA + games). This is where most labs are today.
2. **Phase 2**: Expand to Tier 2 domains. This dramatically broadens capability.
3. **Phase 3**: Add Tier 3 domains with careful verification. This develops agentic capability.
4. **Phase 4**: Curriculum learning across ALL domains simultaneously, with difficulty scaling from trivial to superhuman within each domain.

The key realization: **we don't need new research to do Phase 2 and 3 — we just need to build the environments and data pipelines.** The verification mechanisms already exist. The datasets are on the internet. The training infrastructure (PPO/GRPO/reinforce) is proven. The bottleneck is systematic environment construction, which is what this wiki specifies.

## Vision/Text Steerable Agent Integration

Every domain in this wiki is designed to be addressable by a unified agent that:
- Receives a **task description** (text) and optionally **visual context** (images, screenshots, diagrams)
- Produces a **text output** (answer, code, structured data, action sequence)
- Receives a **binary or numeric reward** from the verification function

This simple interface (observation → action → reward) is universal across all 121+ domains. The agent doesn't need domain-specific architecture — it needs domain-specific **tasks and verifiers**.
