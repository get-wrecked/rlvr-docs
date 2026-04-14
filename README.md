# RLVR: Reinforcement Learning with Verifiable Rewards

A comprehensive framework for training general-purpose AI agents using programmatically verifiable rewards across **2,697 diverse domains**. This project provides the taxonomy, verification infrastructure, dataset registry, and training protocol for scaling from current frontier models toward AGI-level breadth and depth.

## The Core Idea

Current AI training hits a bottleneck: **human feedback doesn't scale**. RLVR eliminates this by using programmatic verification — if you can automatically check whether an answer is correct, you can generate unlimited training signal with zero human involvement.

DeepSeek-R1 showed this works for math and code. We extend it to **every verifiable domain on the internet**: science, engineering, games, language understanding, agent tasks, security, medicine, law, and more.

## Three-Stage Theory of Intelligence

We organize training around a universal pattern of skill acquisition:

| Stage | Phase | What It Develops | Domains |
|-------|-------|-----------------|---------|
| **Stage 1** | Rule Recognition | Pattern inference, structural reasoning, formal logic | ~200 synthetic & formal domains |
| **Stage 2** | System Mastery | Language, code, math, science, vision, agency | ~1,500 applied domains |
| **Stage 3** | Capability Climbing | Expert performance in specialized applications | ~1,000 expert domains |

This is validated by research on Neural Cellular Automata (NCA) pre-training (Lee et al., 2025), which showed that purely synthetic data containing zero linguistic content outperformed 10x more Common Crawl data at preparing models for language, code, and math.

## What's Included

### 13 Rust Verifiers (227 tests)

Production-quality verifiers that are deterministic, fast, and validated against real data:

| Verifier | Type | Tests | Technique |
|----------|------|-------|-----------|
| `math_numerical` | Number extraction + comparison | 26 | Regex extraction, GSM8K `####` format, `\boxed{}`, tolerance |
| `math_equivalence` | Symbolic equivalence | 18 | LaTeX normalization, fraction→decimal, numerical evaluation |
| `exact_match` | Normalized string matching | 27 | Article removal, punctuation stripping, F1 scoring |
| `instruction_following` | Constraint satisfaction | 21 | 15 constraint types (word count, format, include/exclude) |
| `json_schema` | Schema validation | 20 | Type checking, required fields, ranges, patterns, nesting |
| `code_execution` | Subprocess sandbox | 16 | Python execution, stdin/stdout, function-call harness, timeout |
| `sudoku` | Grid constraints | 16 | Row/column/box uniqueness, given-respect, partial credit |
| `chemical_equation` | Atom balance | 15 | Formula parsing with parentheses, coefficient handling |
| `regex_synthesis` | Compile + test | 13 | Full-string anchoring, positive/negative example checking |
| `date_time` | Chrono computation | 13 | Days-between, day-of-week, leap year, date arithmetic |
| `unit_conversion` | Lookup + compute | 12 | 60+ unit pairs, temperature special cases |
| `sql_execution` | SQLite execution | 11 | In-memory DB, result set comparison, order-insensitive |
| `graph_properties` | Graph algorithms | 10 | Dijkstra, connected components, topological sort, coloring |

### 2,697 Domain Environments

Every domain has a concrete verification mechanism, dataset sources, and reconstruction notes. Organized across major categories:

- **Code & Software** (~680 domains): Code generation/repair/translation in 50+ languages, SQL (window functions through DuckDB), every major algorithm (Dijkstra through FFT), design patterns, 54 Exercism tracks, 15 Rosetta Code tasks, SWE-bench variants, HumanEval/MBPP/APPS/CodeContests, DevOps configs (Terraform/K8s/Docker/Helm/ArgoCD/Jenkins/GitLab CI), web development (React/Vue components, OAuth, WebSockets), security (CSRF/XSS/input validation), data structures (BST through Bloom filters), all 10 Advent of Code years
- **Mathematics** (~180 domains): Competition math (AIME/AMC/Putnam/IMO), formal proofs (Lean 4/Coq/Isabelle), symbolic algebra, every calculus topic (limits through surface integrals), number theory, combinatorics, probability, statistics (hypothesis testing through ANOVA), game theory (Nash equilibria), Markov chains, SDEs, 57 MMLU subjects
- **Logic & Formal Methods** (~50 domains): SAT/SMT/ATP/ITP, modal/temporal logic, BDDs, Petri nets, automata construction, Datalog, resolution refutation, 8 formal grammar tasks (CFG through LALR)
- **Science & Engineering** (~350 domains): Physics (projectile motion through Maxwell's equations), chemistry (all organic reaction types, TDC drug discovery, 11 MoleculeNet tasks), biology (protein fitness/stability/solubility, genomics, CAFA), materials science (band gap through defect formation), engineering (HVAC through lightning protection), astronomy (Kepler through eclipse prediction)
- **Language & Knowledge** (~450 domains): GLUE/SuperGLUE/XTREME individual tasks, 152 BIG-Bench tasks, 50 FLORES translation pairs, 20 WMT language pairs, QA (DROP/CosmosQA/StrategyQA), NLI, NER, relation extraction, clinical NLP (15 n2c2/BioCreative tasks), retrieval (MS MARCO/BEIR/MTEB), financial QA (FinQA/TAT-QA), legal (CUAD/LexGLUE)
- **Games & Interactive** (~500 domains): 80+ Atari games, 16 Procgen games, 49 OpenSpiel games, 49 Meta-World tasks, 28 DM Control tasks, 50 RLBench tasks, 16 PettingZoo environments, chess/Go/Shogi, 30+ logic puzzles (Sudoku through Masyu), board games (Mancala through Terraforming Mars), card games (Poker through Skat)
- **Agent & Tool Use** (~100 domains): ALFRED/BEHAVIOR household tasks, WebArena/VisualWebArena, OSWorld, BabyAI levels, MiniHack levels, GAIA multi-tool, autonomous driving (nuPlan/Waymo), drone navigation, dexterous manipulation, D4RL offline RL tasks, Safety-Gymnasium constrained RL
- **Vision & Multimodal** (~200 domains): MMMU/MM-Vet/SEED-Bench, ImageNet variants (V2/R/A/Sketch), COCO detection/panoptic, fine-grained recognition (birds/flowers/cars/aircraft), medical imaging (CheXpert/ISIC/retinal), 3D (ScanNet/ShapeNet/ModelNet), video understanding (Kinetics/Something-Something/DAVIS)
- **Audio & Speech** (~80 domains): LibriSpeech/CommonVoice in 50 languages, SUPERB benchmark, MusicCaps, beat tracking, chord recognition, speaker identification, source separation (MUSDB18)
- **ML & Data Science** (~100 domains): 25+ Kaggle competitions, M4/M5 forecasting, NAS-Bench, fairness/bias (BBQ/WinoBias/CrowS), adversarial robustness, calibration, 30+ UCI datasets, safety benchmarks (HarmBench/XSTest/ETHICS)
- **Expert & Professional** (~150 domains): Medical (MedQA/PubMedQA/PathVQA/CheXpert), legal (CUAD/CaseHOLD/LexGLUE), financial (FinQA/options pricing/portfolio optimization), engineering calculations (pipe sizing through sprinkler design), sports scoring (14 sports), nutrition, construction, clinical trials
- **Miscellaneous** (~50 domains): Geographic knowledge, unit conversions, calendar systems (Hebrew/Islamic/Mayan), encoding (Braille/Morse/semaphore), checksums, number theory curiosities (Collatz/happy numbers)

### Dataset Registry (~6M problems + unlimited procedural generation)

| Category | Datasets | Problems | Size |
|----------|----------|----------|------|
| Math | GSM8K, MMLU | ~24K | 162MB |
| Code | HumanEval, MBPP, WikiSQL | ~82K | 25MB |
| QA | SQuAD, TriviaQA, HotpotQA, CommonsenseQA, COPA, WikiTQ | ~265K | 700MB |
| Fact Verification | FEVER, TabFact | ~263K | 31MB |
| NLI/Classification | SNLI, MultiNLI, ANLI, SST-2, AG News | ~1.25M | 331MB |
| Commonsense/Science | HellaSwag, PIQA, Winogrande, ARC, OpenBookQA, SciQ | ~83K | 666MB |
| Medical | MedQA (USMLE) | ~10K | 15MB |
| Logic/Games | Lichess puzzles, Sudoku | ~4M | 348MB |
| Instruction | IFEval | 541 | <1MB |

**7 procedural generators** produce unlimited training data at 10K-100K problems/second for: unit conversion, date/time, chemical equations, regex, JSON schema, instruction constraints, and graph problems.

## Architecture

```
                    ┌──────────────────┐
                    │  Policy (VLM)    │
                    └────────┬─────────┘
                             │ generates response
                    ┌────────▼─────────┐
                    │  GRPO Trainer    │
                    │  (Python/TRL)   │
                    └────────┬─────────┘
                             │ sends (task, response)
                    ┌────────▼─────────┐
                    │ Verifier Server  │
                    │ (Rust, HTTP)     │
                    │ 13 verifiers     │
                    │ 2,697 domains      │
                    └────────┬─────────┘
                             │ returns score ∈ [0,1]
                    ┌────────▼─────────┐
                    │ Curriculum Ctrl  │
                    │ difficulty ∈[1,10]│
                    │ domain mixing    │
                    └──────────────────┘
```

The Rust verifier runs as a separate HTTP service, enabling language-agnostic integration. This is a departure from existing RLVR frameworks (DeepSeek-R1, veRL, OpenRLHF, TRL) which all use inline Python functions with regex-based extraction.

## Project Structure

```
src/
  main.rs                    # Entry point
  verifiers/
    mod.rs                   # VerifyResult type + Verifier trait
    math_numerical.rs        # 26 tests — numeric extraction & comparison
    math_equivalence.rs      # 18 tests — symbolic math equivalence
    exact_match.rs           # 27 tests — normalized string matching
    instruction_following.rs # 21 tests — constraint satisfaction
    json_schema.rs           # 20 tests — schema validation
    code_execution.rs        # 16 tests — sandboxed code execution
    sudoku.rs                # 16 tests — sudoku grid verification
    chemical_equation.rs     # 15 tests — chemical equation balancing
    regex_synthesis.rs       # 13 tests — regex synthesis verification
    date_time.rs             # 13 tests — date/time computation
    unit_conversion.rs       # 12 tests — physical unit conversion
    sql_execution.rs         # 11 tests — SQL query verification
    graph_properties.rs      # 10 tests — graph algorithm verification
  datasets/
    mod.rs                   # Dataset loading traits
    gsm8k.rs                 # GSM8K parser
    registry.rs              # Dataset registry
wiki/
  overview.md                # High-level thesis and taxonomy
  index.md                   # Master index of all 2,697 domains
  domains/                   # One page per domain (2,697 files)
  concepts/                  # Cross-cutting concepts
    verification-types.md    # Taxonomy of verification mechanisms
    reward-shaping.md        # Reward design principles
    dataset-scaling.md       # Scaling strategies
  synthesis/                 # Analysis pages
    intelligence-hierarchy.md # Three-stage theory deep dive
    harness-architecture.md  # Training harness design
    domain-matrix.md         # Feature matrix across domains
    scaling-roadmap.md       # Path to AGI
    dataset-sources.md       # Internet dataset catalog
    pretraining-sft-sources.md # Upstream pipeline data
    exhaustive-audit.md      # Domain coverage audit
raw/
  datasets/                  # 33 downloaded datasets (2.3GB)
  papers/                    # Source papers
```

## Quick Start

```bash
# Run all verifier tests
cargo test

# Run a specific verifier's tests
cargo test math_numerical
cargo test sudoku
```

## Verification Principles

Every verifier satisfies four properties:

1. **Deterministic**: Same input always produces the same score
2. **Zero false positives**: A score of 1.0 means the answer is definitively correct
3. **Tested against real data**: Every verifier has anti-hardcoding tests validated on real benchmark problems
4. **Fast**: Verification in milliseconds (except code execution, which uses subprocess timeout)

## Verification Types

| Type | Mechanism | Example Domains |
|------|-----------|----------------|
| **Exact match** | Output matches known answer | Math, QA, classification |
| **Execution-based** | Output is tested by running it | Code, SQL, regex |
| **Simulation-based** | Physics/logic simulator verifies | Circuit design, robotics |
| **Constraint satisfaction** | Hard constraints must be met | Sudoku, scheduling |
| **Diff-based** | Structural comparison to reference | AST, DOM, image similarity |
| **Rule-based** | Formal rules check correctness | Chess legality, grammar |
| **Outcome-based** | Downstream result determines correctness | Game win, task completion |

Hypothetical or "LLM-as-judge" rewards are NOT used. Every domain has a concrete, implementable verification function.

## Related Work

- **DeepSeek-R1** (2024): RLVR on math + code → strong reasoning. We extend from 2-5 domains to 272.
- **NCA Pre-Pre-Training** (Lee et al., 2025): Synthetic CA data develops transferable attention mechanisms. Our Stage 1 formalizes this.
- **AlphaProof / AlphaGeometry** (DeepMind, 2024): RLVR for formal mathematics. We extend to the full reasoning spectrum.
- **veRL, OpenRLHF, TRL**: Open-source RL frameworks with inline Python verifiers. Our Rust verifier server with 227 validated tests is a significant infrastructure improvement.

## License

MIT
