# Three-Stage Theory of Intelligence

## The Universal Pattern

Intelligence, in any domain, follows a universal pattern:

1. **Recognize the rules of the game** — observe patterns, infer latent structure, model hidden dynamics
2. **Strengthen visibility on high-leverage areas** — recognize the meta-game, understand how systems work as wholes
3. **Maximize curves** — hill-climb performance in specific areas, discover new methods to accelerate progress
4. **Climb curves** — apply the discovered methods to push toward mastery
5. **Repeat** — transfer the meta-skills to new domains

This maps directly to three training stages:

| Stage | Training Phase | What It Develops | Analogy |
|-------|---------------|-----------------|---------|
| **Pre-training** | Rule Recognition | Pattern inference, structural reasoning, formal logic | Learning what games are and how games work |
| **Mid-training** | System Mastery | Language, code, math, science, vision, agency | Becoming competent at civilization's major games |
| **Post-training** | Capability Climbing | Expert performance in specific applications | Becoming world-class at games that matter |

## Empirical Validation: The NCA Paper

Lee et al. (2025) demonstrated that 164M tokens of purely synthetic Neural Cellular Automata data — containing zero linguistic content — outperformed 1.6B tokens of Common Crawl at preparing models for language, code, and math.

The transfer occurred entirely through attention layers learning **pure latent rule inference**. The content was irrelevant; the structure was everything.

Key findings:
- **Complexity matching**: Code benefits from lower-complexity rules (gzip ratio 30-40%), while math and natural language benefit from higher complexity (50%+)
- **Transfer is multiplicative**: NCA pre-training provided a 1.4-1.6x convergence acceleration across downstream tasks
- **No semantic shortcuts**: Without natural language, the model is forced to develop pure computational primitives

## Stage 1: Rule Recognition (~55 domains)

**Goal**: Develop the raw computational machinery for pattern recognition, latent structure inference, and abstract reasoning.

### Synthetic Rule Systems (purest Stage 1)
Zero domain content, pure structure:
- **Cellular automata rule inference**: Observe CA trajectories → infer transition rules → predict future states
- **Synthetic grammar induction**: Dyck languages, PCFGs, HMMs — learn hidden grammatical structure
- **Abstract pattern completion (ARC)**: Visual grid transformations — infer the rule from examples
- **Latent concept learning**: Few-shot identification of hidden concepts
- **Dynamical system identification**: Infer governing equations from trajectories
- **Boolean function learning**: Learn Boolean functions from truth tables

### Formal Reasoning Primitives (~12 domains)
Logic, proofs, automata — pure structure:
- SAT/SMT solving, first-order theorem proving, type theory, formal language theory, automated theorem proving

### Mathematical Primitives (~9 domains)
Pure computation:
- Numerical, linear algebra, tensor operations, modular arithmetic, number theory, differential equations

### Code Primitives (~7 domains)
Programming as reasoning:
- Code generation, repair, regex, functional programming, logic programming

### Perceptual Primitives (~6 domains)
Raw pattern recognition:
- Image classification, OCR, speech recognition, spatial reasoning

**Data**: Entirely procedurally generated (unlimited) or from formal benchmarks (TPTP, SMT-LIB, SAT Competition). No internet text needed.

## Stage 2: System Mastery (~110 domains)

**Goal**: Apply rule-recognition primitives to civilization's major systems.

### Language Systems (19 domains)
NLI, reading comprehension, information extraction, semantic parsing, fact verification, entity linking, QA, commonsense reasoning, multilingual tasks

### Code Systems (19 domains)
SQL, code translation/optimization/refactoring, compiler construction, type inference, database design, CI/CD, testing, spreadsheet formulas

### Mathematics Systems (16 domains)
Competition math, formal proofs, symbolic manipulation, olympiad problems, theorem formalization, mathematical programming

### Science & Engineering (14 domains)
Physics simulation, chemistry, biology sequences, materials science, fluid dynamics, engineering, control systems, climate

### Visual & Multimodal (10 domains)
VQA, chart understanding, document extraction, segmentation, 3D scene understanding, data visualization

### Agent & Interaction (8 domains)
Web navigation, computer use, GUI navigation, instruction following, API orchestration, planning

### Games & Strategy (9 domains)
Chess, Go, multi-agent coordination, negotiation, puzzles, social choice

**Data**: Internet-scale datasets (SQuAD, TriviaQA, FEVER, SNLI, HumanEval, MBPP, WikiSQL, GSM8K, etc.)

**Cross-domain transfer**: Math + code → mathematical programming. Science + code → computational science. Language + vision → visual QA. The total capability exceeds the sum.

## Stage 3: Capability Climbing (~80 domains)

**Goal**: Hill-climb to expert and superhuman performance on specific high-value applications.

### Specialized Science
Molecular generation (drug design), protein design, DNA sequence design, retrosynthesis, chip design (RTL), analog circuits, PCB layout, quantum computing, robotics

### Specialized Code
Compiler optimization, database query optimization, memory management, concurrency, formal verification, distributed systems

### Specialized Security
CTF challenges, defense/hardening, reverse engineering, cryptanalysis, protocol verification

### Specialized Professional
Medical diagnosis (USMLE), legal reasoning (Bar exam), actuarial computation, economic modeling, scientific experiment design

### Specialized Creative
Constrained writing, formal poetry, music generation, shader programming, typographic layout

**Data**: Domain-specific datasets (MedQA, CryptoHack, Lichess puzzles, PDBbind) plus procedural generation.

## The Feynman Principle

Richard Feynman's genius wasn't in memorizing physics — it was in seeing the underlying rules and applying them creatively:

- **Stage 1**: Learn to see rules (Feynman's ability to find the pattern in any system)
- **Stage 2**: Apply to physics, math, engineering (Feynman's broad competence)
- **Stage 3**: Push the frontier in specific areas (Feynman's Nobel Prize work in QED)

## The Inverse Dynamics Connection

The ultimate form of this framework is an **inverse dynamics model** — a system that observes any environment, reverse-engineers its rules, and makes progress toward specified goals:

- **Stage 1** develops the inverse dynamics capability (infer rules from observations)
- **Stage 2** applies it to the major environments of civilization
- **Stage 3** optimizes for specific objectives within those environments

The 272 domains are 272 different "games," and the three-stage training teaches the agent to play them all.

## Why Breadth Matters for AGI

A model trained only on math and code develops narrow reasoning. The jump to general intelligence requires exposure to the full diversity of human knowledge:

- The **physical world** (science, engineering)
- The **social world** (language, law, medicine)
- The **abstract world** (mathematics, logic)
- The **creative world** (music, art, writing)
- The **interactive world** (web, computers, games)

Training across all domains simultaneously develops cognitive capabilities that no single domain can produce alone — just as human intelligence emerges from the interaction of perception, language, reasoning, memory, and action.
