---
title: Intelligence Hierarchy — Pre/Mid/Post Training Stages
description: All RLVR domains organized by the 3-stage theory of intelligence
---

# Intelligence Hierarchy: Pre/Mid/Post Training

## The Theory

Intelligence is:
1. **Recognize rules of the game**
2. **Strengthen visibility on high-leverage areas** (recognize the meta / game theory)
3. **Maximize curves in those areas**, discover new methods to climb
4. **Climb curves**
5. **Repeat**

This maps to three training stages:

| Stage | What It Develops | Analogy |
|-------|-----------------|---------|
| **Pre-training** | Diverse rule recognition — learning to see patterns, infer latent structure, model hidden dynamics | Learning what games exist and how games work in general |
| **Mid-training** | General systems — applying rule recognition to broadly useful domains: language, code, math, physics | Becoming competent at the major "games" of civilization |
| **Post-training** | Specific ability climbing — hill-climbing performance on narrow, specialized capabilities | Becoming world-class at specific games that matter |

The NCA paper validates this: CA pre-pre-training develops **pure rule inference in attention layers** (Stage 1), which then accelerates learning of language/code/math (Stage 2), which then transfers to specific benchmarks like GSM8K and HumanEval (Stage 3).

---

## Stage 1: PRE-TRAINING — Rule Recognition

**Purpose**: Develop the raw cognitive machinery for pattern recognition, latent structure inference, sequential prediction, and abstract reasoning. These are the "attention mechanisms" that the NCA paper showed transfer across all domains.

**Key insight from NCA paper**: "With no semantic fallback, the model is forced to develop pure latent rule inference capabilities." Stage 1 environments should have NO semantic shortcuts — only structure.

**Properties of Stage 1 environments**:
- Procedurally generated (unlimited data)
- No natural language semantics — pure structure
- Reward from exact match or execution
- Develop domain-agnostic computational primitives
- Transfer to everything in Stage 2 and 3

### 1.1 Synthetic Rule Systems (NEW — inspired by NCA paper)
These are the purest Stage 1 environments. They develop rule inference without any domain content.

| Domain | What It Develops | Verification |
|--------|-----------------|-------------|
| **cellular-automata-rule-inference** | Spatiotemporal rule inference from observation | Simulate CA, compare prediction |
| **synthetic-grammar-induction** | Hierarchical structure tracking, bracket matching | Grammar acceptance check |
| **abstract-pattern-completion** | Visual abstract reasoning, transformation inference (ARC) | Exact grid match |
| **latent-concept-learning** | In-context concept identification from few examples | Apply concept to test instances |
| **procedural-game-inference** | Strategic rule inference from gameplay traces | Legal move/outcome prediction |
| **dynamical-system-identification** | Continuous dynamics inference from trajectories | Simulate identified system |
| **boolean-function-learning** | Boolean logic learning from truth tables | Evaluate on all inputs |
| **program-synthesis-from-io** | Program induction from input-output examples | Execute on held-out tests |
| **cellular-automata** | Predicting evolution of known CA | Simulate and compare |

### 1.2 Formal Reasoning Primitives
Pure logic and mathematical structure — no applied domain knowledge needed.

| Domain | What It Develops | Verification |
|--------|-----------------|-------------|
| **logic-propositional** | Boolean reasoning, SAT solving | SAT solver verification |
| **logic-first-order** | Quantified reasoning, model theory | ATP verification |
| **smt-solving** | Theory combination, constraint solving | SMT solver check |
| **automated-theorem-proving** | Proof construction | Proof checker |
| **formal-language-theory** | Automata, grammars, computability | DFA/grammar equivalence |
| **boolean-function-learning** | Logic circuit learning | Exhaustive/random evaluation |
| **type-theory-puzzles** | Curry-Howard (proofs = programs) | Type checker |
| **graph-algorithm-execution** | Algorithmic reasoning on graphs | NetworkX computation |
| **information-theory** | Entropy, coding, channel capacity | Direct computation |
| **combinatorics-optimization** | Discrete optimization | Objective value computation |

### 1.3 Mathematical Primitives
Pure mathematical computation — develops the machinery used across all quantitative domains.

| Domain | What It Develops | Verification |
|--------|-----------------|-------------|
| **math-numerical** | Arithmetic, numerical reasoning | Exact match |
| **linear-algebra-computation** | Matrix operations, eigenvalues | NumPy computation |
| **tensor-computation** | Higher-order tensor operations | NumPy/einsum |
| **satisfiability-modular-arithmetic** | Finite field arithmetic | Direct computation |
| **number-theory-computation** | Prime, factoring, modular | Direct computation |
| **probability-statistics** | Probability computation | Exact/simulation |
| **differential-equations** | ODE/PDE solving | Substitution check |
| **geometry-computation** | Computational geometry | Geometric computation |
| **abstract-algebra** | Group/ring theory | CAS verification |

### 1.4 Core Code Primitives
The fundamental programming capability — develops code as a reasoning medium.

| Domain | What It Develops | Verification |
|--------|-----------------|-------------|
| **code-generation** | Writing code from specs | Test suite execution |
| **code-repair** | Understanding and fixing bugs | Test suite passes |
| **regex-synthesis** | Pattern language | Match/reject examples |
| **functional-programming** | Pure functional reasoning | Test execution |
| **logic-programming** | Declarative reasoning | Query result check |
| **constraint-programming** | Constraint modeling | Solution feasibility |
| **shell-commands** | System interaction primitives | Output/state check |

### 1.5 Perceptual Primitives
Raw pattern recognition in visual/audio modalities.

| Domain | What It Develops | Verification |
|--------|-----------------|-------------|
| **image-classification** | Visual category recognition | Label match |
| **optical-character-recognition** | Text extraction from images | CER/WER |
| **audio-speech-recognition** | Speech pattern recognition | WER |
| **spatial-reasoning** | Spatial relationship understanding | Exact match |
| **text-classification** | Text categorization | Label match |
| **cloze-completion** | Contextual prediction | Exact match |

**Total Stage 1 domains: ~55**

---

## Stage 2: MID-TRAINING — General Systems

**Purpose**: Apply rule recognition to the major "games" of civilization — language, code, mathematics, science, engineering, law, medicine. These are the general-purpose systems where competence unlocks vast capability.

**Key insight**: Stage 2 is where you "recognize the meta" — not just individual rules, but how entire systems work. You learn language not as individual words but as a system of communication. You learn code not as individual functions but as software engineering. You learn physics not as individual formulas but as a way of modeling reality.

**Properties of Stage 2 environments**:
- Mix of internet datasets and procedural generation
- Require combining Stage 1 primitives with domain knowledge
- Develop broadly transferable expertise
- Each domain is a "general system" — mastering it helps with many downstream tasks

### 2.1 Language Systems
Understanding and generating human language as a general system.

| Domain | What It Develops |
|--------|-----------------|
| **natural-language-inference** | Logical reasoning in natural language |
| **reading-comprehension** | Multi-step text comprehension |
| **information-extraction** | Structured understanding from text |
| **semantic-parsing** | NL → formal representation |
| **fact-verification** | Evidence-based reasoning |
| **entity-linking** | Grounding language to knowledge |
| **question-answering-extractive** | Finding answers in text |
| **question-answering-closed** | Factual knowledge recall |
| **table-understanding** | Structured data comprehension |
| **formal-grammar** | Syntactic structure |
| **commonsense-reasoning** | World model through language |
| **temporal-reasoning** | Time-based reasoning |
| **multilingual-tasks** | Cross-lingual transfer |
| **knowledge-graph-completion** | Relational knowledge |
| **knowledge-base-querying** | Structured knowledge access |
| **spelling-grammar** | Language mechanics |
| **argument-analysis** | Logical argumentation |
| **natural-language-inference** | Textual entailment |
| **multi-hop-web-research** | Multi-source synthesis |

### 2.2 Code Systems
Software engineering as a general system for building and reasoning.

| Domain | What It Develops |
|--------|-----------------|
| **sql-generation** | Structured data querying |
| **code-translation** | Cross-language understanding |
| **code-optimization** | Performance reasoning |
| **code-refactoring** | Code quality reasoning |
| **compiler-construction** | Language implementation |
| **compiler-tasks** | Compilation pipeline |
| **type-inference** | Type system reasoning |
| **test-generation** | Specification reasoning |
| **code-documentation** | Code-language bridge |
| **data-wrangling** | Data transformation |
| **infrastructure-as-code** | System configuration |
| **build-configuration** | Build systems |
| **cicd-pipeline** | DevOps workflows |
| **database-design** | Data modeling |
| **program-equivalence** | Semantic equivalence |
| **competitive-coding-interactive** | Algorithmic problem solving |
| **text-to-code-nl** | NL→code bridge |
| **graph-query-languages** | Graph data querying |
| **spreadsheet-formulas** | Tabular computation |

### 2.3 Mathematics Systems
Mathematics as a general system for precise reasoning.

| Domain | What It Develops |
|--------|-----------------|
| **math-competition** | Competition problem solving |
| **math-formal-proofs** | Rigorous proof construction |
| **math-symbolic** | Symbolic manipulation |
| **math-olympiad-geometry** | Geometric reasoning |
| **math-olympiad-inequalities** | Inequality reasoning |
| **symbolic-regression** | Pattern→equation discovery |
| **mathematical-modeling** | Reality→math translation |
| **mathematical-programming** | Optimization formulation |
| **topology-computation** | Topological reasoning |
| **knot-theory** | Invariant computation |
| **category-theory** | Abstract structure |
| **mathematical-conjecture-testing** | Hypothesis testing |
| **theorem-proving-natural-language** | NL proof construction |
| **theorem-formalization** | NL→formal bridge |
| **theorem-application** | Strategy selection |
| **proof-repair** | Proof maintenance |

### 2.4 Science & Engineering Systems
The physical world as a system to model, predict, and design within.

| Domain | What It Develops |
|--------|-----------------|
| **physics-simulation** | Physical reasoning |
| **chemistry-computation** | Chemical reasoning |
| **biology-sequence** | Biological sequence understanding |
| **materials-science** | Material property reasoning |
| **fluid-dynamics** | Continuum mechanics |
| **electrical-engineering** | Circuit reasoning |
| **mechanical-engineering** | Structural/thermal reasoning |
| **control-systems** | Feedback system design |
| **signal-processing** | Signal analysis |
| **climate-science** | Earth system reasoning |
| **astronomy-computation** | Celestial mechanics |
| **pde-numerical** | Numerical methods |
| **simulation-modeling** | Simulation construction |
| **epidemiology-modeling** | Epidemic reasoning |

### 2.5 Visual & Multimodal Systems
Seeing and reasoning about the visual world as a general system.

| Domain | What It Develops |
|--------|-----------------|
| **visual-question-answering** | Visual reasoning |
| **chart-understanding** | Data visualization comprehension |
| **document-ocr-extraction** | Document understanding |
| **image-segmentation** | Visual parsing |
| **visual-grounding** | Language→vision grounding |
| **video-understanding** | Temporal visual reasoning |
| **multimodal-reasoning** | Cross-modal reasoning |
| **3d-scene-understanding** | 3D perception |
| **math-word-problems-visual** | Visual math reasoning |
| **data-visualization** | Visual communication |

### 2.6 Agent & Interaction Systems
Interacting with environments as a general system.

| Domain | What It Develops |
|--------|-----------------|
| **web-navigation** | Web interaction |
| **computer-use** | General computer operation |
| **gui-navigation** | App interaction |
| **instruction-following** | Precise instruction adherence |
| **natural-language-to-api** | API orchestration |
| **planning-classical** | Plan construction |
| **workflow-automation** | Process automation |
| **file-system-tasks** | File management |

### 2.7 Game & Strategy Systems
Strategic reasoning as a general system.

| Domain | What It Develops |
|--------|-----------------|
| **chess** | Perfect-info strategy |
| **go-game** | Territorial strategy |
| **multi-agent-coordination** | Game-theoretic reasoning |
| **negotiation-bargaining** | Strategic interaction |
| **puzzle-games** | Constraint-based search |
| **interactive-fiction** | Narrative strategy |
| **board-games** | Abstract strategy |
| **game-design-level-generation** | Creative constraint satisfaction |
| **voting-social-choice** | Social aggregation |

**Total Stage 2 domains: ~110**

---

## Stage 3: POST-TRAINING — Specific Ability Climbing

**Purpose**: Hill-climb performance on specific, specialized capabilities. This is where you go from "competent" to "expert" in targeted domains. The agent has already learned to recognize rules (Stage 1) and operate general systems (Stage 2). Now it optimizes for specific high-value applications.

**Key insight**: Stage 3 is where you "maximize curves." You've identified the game, you understand the meta — now you push to be the best at specific things.

**Properties of Stage 3 environments**:
- Often require deep domain expertise
- Reward is highly task-specific
- May require specialized tools/simulators
- Performance is measured against human experts
- This is where you get "superhuman" capabilities in specific domains

### 3.1 Specialized Science & Engineering
Deep domain expertise in specific scientific/engineering fields.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **molecular-generation** | Drug design (QED, binding affinity) |
| **protein-design** | Protein engineering (TM-score, Rosetta energy) |
| **dna-sequence-design** | Genetic engineering (Tm, GC content, off-target) |
| **chemistry-retrosynthesis** | Synthesis route planning |
| **chip-design-rtl** | Hardware design (Verilog verification) |
| **analog-circuit-design** | Analog circuit optimization (SPICE specs) |
| **pcb-layout** | Physical circuit layout (DRC) |
| **circuit-design** | Circuit analysis |
| **quantum-computing** | Quantum circuit design |
| **robotics-planning** | Robot motion planning |
| **engineering-optimization** | Structural optimization (FEA) |
| **genomics-bioinformatics** | Bioinformatics analysis |
| **phylogenetics** | Evolutionary tree reconstruction |
| **3d-mesh-processing** | 3D geometry processing |
| **cad-modeling** | 3D model creation |

### 3.2 Specialized Code & Systems
Deep expertise in specific software domains.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **compiler-optimization-passes** | Compiler pass ordering |
| **database-query-optimization** | Query plan optimization |
| **database-migrations** | Schema evolution |
| **memory-management** | Low-level systems (sanitizer-clean) |
| **concurrency-puzzles** | Thread-safe code (TSan-clean) |
| **formal-verification-software** | Verified software (Dafny, Frama-C) |
| **distributed-systems** | Protocol design (TLA+ verified) |
| **network-configuration** | Network config (simulation-verified) |
| **dependency-resolution** | Package resolution |
| **web-scraping** | Data extraction |
| **api-usage** | API integration |
| **ml-pipeline-optimization** | AutoML performance |

### 3.3 Specialized Security
Deep expertise in cybersecurity.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **cybersecurity-ctf** | CTF flag capture |
| **cybersecurity-defense** | Hardening & detection |
| **reverse-engineering** | Binary analysis |
| **mathematical-cryptanalysis** | Cipher breaking |
| **cryptographic-protocol-design** | Protocol verification |
| **cryptography-challenges** | Crypto problem solving |

### 3.4 Specialized Professional Domains
Expert-level professional knowledge.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **medical-diagnosis** | Clinical reasoning (USMLE scores) |
| **legal-reasoning** | Legal analysis (Bar exam scores) |
| **accounting-bookkeeping** | Financial computation |
| **actuarial-computation** | Actuarial science |
| **economic-modeling** | Economic analysis |
| **financial-calculation** | Financial math |
| **scientific-experiment-design** | Research methodology |
| **scientific-paper-extraction** | Literature mining |
| **educational-content** | Pedagogical content creation |

### 3.5 Specialized Format & Creative
Mastery of specific output formats and constrained creativity.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **constrained-writing** | Writing under constraints |
| **poetry-formal** | Formal poetry (meter, rhyme) |
| **music-generation-midi** | Music with theory constraints |
| **music-theory** | Music theory application |
| **music-audio-processing** | Audio transcription |
| **crossword-construction** | Puzzle construction |
| **html-css-generation** | Web UI reproduction |
| **image-to-code** | Screenshot→code |
| **shader-programming** | GPU visual programming |
| **svg-generation** | Vector graphics |
| **latex-typesetting** | Document typesetting |
| **image-generation-constrained** | Code-based image generation |
| **cartography** | Map generation |
| **ascii-art** | ASCII art |

### 3.6 Specialized Data & Analytics
Deep expertise in specific data analysis tasks.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **data-science-eda** | Statistical analysis |
| **time-series-forecasting** | Temporal prediction |
| **supply-chain-optimization** | Logistics optimization |
| **scheduling** | Job scheduling |
| **process-mining** | Workflow discovery |
| **network-analysis** | Social network metrics |
| **geospatial-analysis** | Geographic computation |
| **bayesian-network-reasoning** | Probabilistic inference |
| **causal-reasoning** | Causal inference |
| **survival-analysis** | Survival/reliability |

### 3.7 Specialized Math
Deep mathematical expertise in specific branches.

| Domain | The Specific Curve Being Climbed |
|--------|--------------------------------|
| **theorem-discovery** | Novel theorem generation |
| **math-theorem-conjecturing** | Conjecture generation |
| **mathematical-logic-metalogic** | Logic metatheory |
| **proof-complexity** | Complexity reductions |
| **error-correction-codes** | Coding theory design |

### 3.8 Specialized Miscellaneous
Narrow but verifiable domains.

| Domain | The Specific Curve Being Climbed |
|--------|-----------------|
| **unit-conversion** | Unit conversion accuracy |
| **date-time-computation** | Date/time computation |
| **recipe-scaling** | Proportional scaling |
| **encryption-decryption** | Crypto operations |
| **compression-encoding** | Encoding/decoding |
| **data-formatting** | Format conversion |
| **config-generation** | Config file generation |
| **protocol-compliance** | Protocol message construction |
| **accessibility-compliance** | WCAG compliance |
| **calendar-ical** | Calendar file generation |
| **markdown-formatting** | Markup conversion |
| **regex-crossword** | Regex puzzle solving |
| **sudoku-generation** | Puzzle generation |
| **anagram-wordplay** | Word puzzles |
| **proof-of-work-puzzles** | Hash puzzles |
| **legal-logic** | Legal rule application |
| **geographic-trivia** | Factual knowledge |
| **citation-verification** | Source verification |
| **data-entry-correction** | Error correction |
| **operating-system-concepts** | OS computation |
| **document-parsing** | Document structure |
| **structured-output-generation** | Schema-valid output |
| **summarization-extractive** | Summary extraction |
| **translation-reference** | Machine translation |
| **ontology-construction** | Ontology engineering |
| **email-tasks** | Email composition |
| **calendar-scheduling** | Scheduling constraints |
| **spreadsheet-tasks** | Spreadsheet operations |
| **map-navigation** | Route finding |
| **card-games** | Card game strategy |
| **game-playing-atari** | Video game scores |
| **climate-weather** | Weather prediction |

**Total Stage 3 domains: ~80**

---

## The NCA Paper's Contribution to This Framework

The NCA paper proves Stage 1 works:

1. **164M NCA tokens** (pure Stage 1 synthetic data with zero semantic content)
2. → **Develops attention-layer rule inference** (the primitive computational capability)
3. → **Beats 1.6B tokens of C4** (natural Stage 2 data) at preparing for Stage 2
4. → **Transfers to GSM8K (+16%), HumanEval (+10%), BigBench (+41%)** (Stage 3 performance)

The implication: **you should invest heavily in Stage 1 environments** because they provide the foundation that makes Stage 2 and Stage 3 learning faster and better. The paper's key finding — that attention layers transfer while MLPs don't — tells us that Stage 1 builds the **general computational machinery** while Stage 2 and 3 fill in the **domain-specific knowledge**.

### NCA Complexity Matching as Curriculum Design

The paper's finding that optimal NCA complexity varies by domain gives us a curriculum principle:

| Target Domain | Optimal Stage 1 Complexity | Why |
|---------------|---------------------------|-----|
| Code | Lower complexity (gzip 30-40%) | Code has rigid, deterministic structure |
| Math/Web text | Higher complexity (gzip 50%+) | Rich, unpredictable dependencies |
| Logic | Lower complexity | Deterministic rule systems |
| Science | Higher complexity | Complex interacting systems |
| Games | Medium complexity | Structured but with strategic depth |

This suggests: **tune Stage 1 environments' complexity to match the Stage 2/3 targets you're aiming for.**

---

## Proposed Training Schedule

```
Phase 0 (Pre-pre-training): Stage 1 synthetic environments
├── NCA rule inference (complexity-matched per target)
├── Synthetic grammar induction (Dyck, PCFG)
├── Abstract pattern completion (ARC-like)
├── Boolean function learning
├── Basic logic (SAT, graph algorithms)
├── Basic math (arithmetic, linear algebra)
├── Basic code (simple function generation)
└── Basic perception (image/text classification)

Phase 1 (Foundation): Stage 1 → Stage 2 transition
├── All Stage 1 environments at increasing difficulty
├── Begin Stage 2 general systems (language, code, math)
├── Curriculum: start with Stage 2 domains closest to Stage 1
│   (formal proofs → math competition → language reasoning)
└── Agent success rate maintained at 40-60% via difficulty scaling

Phase 2 (Broadening): Full Stage 2
├── All Stage 2 environments simultaneously
├── Continue Stage 1 at maintenance level
├── Cross-domain transfer (math+code, science+code, language+vision)
└── Difficulty scaling within each domain

Phase 3 (Specialization): Stage 2 → Stage 3
├── Select Stage 3 domains based on capability gaps
├── Deep hill-climbing on specific applications
├── Stage 3 difficulty pushes toward superhuman
└── Stage 1+2 continue as regularization

Phase 4 (AGI): All stages simultaneously
├── Stage 1: Continuous synthetic data generation (unlimited)
├── Stage 2: All general systems at expert level
├── Stage 3: Superhuman in 50+ specialized domains
└── The agent can recognize any game, understand any system,
    and climb any curve.
```

---

## The Feynman Connection

As you noted, this is Feynman-like: "understanding the rules is the hardest part." Feynman's genius wasn't memorizing physics facts — it was his ability to **see the underlying rules** and then apply them creatively. Stage 1 is Feynman's "learning to see." Stage 2 is Feynman's "applying to physics." Stage 3 is Feynman's "cracking specific problems nobody else could."

The NCA paper shows you can develop Feynman's "learning to see" from pure synthetic data that has nothing to do with physics. The rules are the rules — whether they govern cellular automata, chess, chemical reactions, or social dynamics. The same attention mechanisms infer them all.
