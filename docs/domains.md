# Domain Taxonomy

**272 domains** organized across 11 categories, mapped to the three-stage training framework.

## Summary Statistics

| Category | Domains | Primary Verification | Stage |
|----------|---------|---------------------|-------|
| Mathematics | 29 | exact_match, execution | 1-3 |
| Logic & Formal Methods | 12 | execution, rule | 1 |
| Code & Software | 26 | execution | 1-3 |
| Science & Engineering | 27 | execution, simulation | 2-3 |
| Language & Knowledge | 27 | exact_match | 2 |
| Games & Interactive | 14 | rule, outcome | 1-2 |
| Agent & Tool Use | 12 | outcome, constraint | 2-3 |
| Vision & Multimodal | 14 | exact_match, execution | 2 |
| Format-Constrained & Creative | 20 | constraint, rule | 2-3 |
| Security | 6 | exact_match, execution | 3 |
| Domain Expert | 5 | exact_match, execution | 3 |
| Machine Learning | 2 | execution | 3 |
| Systems | 4 | simulation, constraint | 2-3 |

### By Verification Type

| Type | Count | Reliability |
|------|-------|-------------|
| exact_match | ~55 | Excellent |
| execution | ~85 | Excellent |
| constraint | ~20 | Excellent |
| rule | ~15 | Excellent |
| simulation | ~15 | Very Good |
| diff | ~10 | Good |
| outcome | ~8 | Good |
| approximate | ~4 | Acceptable |

### By Status

| Status | Count | Meaning |
|--------|-------|---------|
| verified | ~100 | Verification confirmed working |
| strong_hypothesis | ~100 | High confidence, untested |
| needs_validation | ~12 | Speculative |

---

## Mathematics (29 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Math competition (AMC/AIME/Olympiad) | exact_match | verified | 2 |
| Math numerical (GSM8K, AMPS) | exact_match | verified | 1 |
| Formal proofs (Lean4/Coq/Isabelle) | execution | verified | 2 |
| Symbolic algebra/calculus | execution | strong | 2 |
| Theorem conjecturing | execution | needs_validation | 3 |
| Visual math (GeoQA, MathVista) | exact_match | verified | 2 |
| Olympiad inequalities | execution | strong | 3 |
| Number theory computation | execution | strong | 1 |
| Geometry computation | execution | strong | 1 |
| Geometric construction | simulation | strong | 2 |
| Probability & statistics | exact_match | strong | 2 |
| Abstract algebra (GAP/SageMath) | execution | needs_validation | 3 |
| Graph theory | execution | strong | 1 |
| Combinatorics & optimization | execution | strong | 1 |
| Linear algebra computation | execution | verified | 1 |
| Differential equations | execution | verified | 1 |
| Topology computation | execution | strong | 3 |
| Category theory | execution | needs_validation | 3 |
| Knot theory | execution | strong | 3 |
| Information theory | execution | verified | 1 |
| Symbolic regression | execution | verified | 2 |
| Mathematical modeling | execution | strong | 2 |
| Mathematical programming (LP/IP) | execution | verified | 2 |
| Conjecture testing | execution | strong | 3 |
| NL theorem proving | execution | strong | 2 |
| Theorem application | execution | strong | 2 |
| Theorem discovery | rule | needs_validation | 3 |
| Tensor computation | execution | verified | 1 |
| Satisfiability modular arithmetic | execution | verified | 1 |

## Logic & Formal Methods (12 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Propositional logic (SAT) | execution | strong | 1 |
| First-order logic (ATP) | execution | strong | 1 |
| Logic puzzles (Sudoku, etc.) | constraint | strong | 1 |
| Formal specification (TLA+, Alloy) | execution | needs_validation | 2 |
| SMT solving | execution | verified | 1 |
| Automated reasoning | rule | strong | 1 |
| Automated theorem proving (TPTP) | rule | verified | 1 |
| Theorem formalization (NL→Lean4) | rule | strong | 2 |
| Proof repair | rule | strong | 3 |
| Formal language theory | execution | strong | 1 |
| Boolean function learning | execution | verified | 1 |
| Type theory puzzles | rule | strong | 1 |

## Code & Software (26+ domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Code generation (HumanEval/APPS) | execution | verified | 1 |
| Code repair (SWE-bench) | execution | verified | 3 |
| Code translation | execution | strong | 2 |
| Code optimization | execution | strong | 3 |
| Code refactoring | execution | strong | 2 |
| Code documentation (doctest) | execution | strong | 2 |
| SQL generation (Spider, BIRD) | execution | verified | 2 |
| Regex synthesis | execution | strong | 1 |
| Shell commands | execution | strong | 2 |
| Type inference | execution | strong | 2 |
| Compiler tasks | execution | strong | 3 |
| Compiler construction | execution | strong | 3 |
| Compiler optimization passes | execution | strong | 3 |
| API usage | execution | strong | 2 |
| NL to API | execution | verified | 2 |
| Infrastructure as code | execution | strong | 2 |
| Data wrangling | execution | strong | 2 |
| Test generation | execution | strong | 2 |
| Database migrations | execution | strong | 2 |
| Build configuration | execution | strong | 2 |
| DB query optimization | execution | strong | 3 |
| Competitive programming | execution | verified | 2 |
| Program synthesis from I/O | execution | verified | 1 |
| Program equivalence | execution | verified | 2 |
| Functional programming | execution | verified | 1 |
| Spreadsheet formulas | execution | verified | 2 |

## Science & Engineering (27 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Physics simulation | simulation | strong | 2 |
| Chemistry computation (RDKit) | execution | strong | 2 |
| Retrosynthesis | execution | strong | 3 |
| Biology sequences | execution | strong | 2 |
| Molecular generation (drug design) | execution | strong | 3 |
| Protein design | simulation | strong | 3 |
| DNA sequence design | execution | strong | 3 |
| Genomics/bioinformatics | execution | strong | 3 |
| Phylogenetics | execution | strong | 3 |
| Circuit design (SPICE) | simulation | strong | 3 |
| Analog circuit design | simulation | strong | 3 |
| Chip design RTL (Verilog) | simulation | verified | 3 |
| Control systems | simulation | strong | 2 |
| Signal processing | execution | strong | 2 |
| Materials science | execution | strong | 3 |
| Fluid dynamics | simulation | strong | 3 |
| Electrical engineering | execution | strong | 2 |
| Astronomy computation | execution | strong | 2 |
| Quantum computing | simulation | strong | 3 |
| Robotics planning | simulation | strong | 3 |
| Climate science | execution | strong | 2 |
| Climate/weather | execution | strong | 2 |
| Epidemiology modeling | execution | strong | 3 |
| PDE numerical | execution | strong | 2 |
| Experiment design | constraint | needs_validation | 3 |
| Scientific paper extraction | exact_match | verified | 2 |

## Language & Knowledge (27 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Information extraction (NER) | exact_match | verified | 2 |
| Fact verification (FEVER) | exact_match | verified | 2 |
| Extractive QA (SQuAD) | exact_match | verified | 2 |
| Closed QA (trivia) | exact_match | verified | 2 |
| Translation (reference-based) | approximate | verified | 2 |
| Text classification | exact_match | verified | 2 |
| Structured output (JSON/XML) | exact_match | verified | 2 |
| Document parsing | exact_match | verified | 2 |
| Knowledge graph completion | exact_match | verified | 2 |
| Citation verification | exact_match | verified | 2 |
| Data entry correction | exact_match | verified | 2 |
| Table understanding | exact_match | verified | 2 |
| Extractive summarization | approximate | verified | 2 |
| Entity linking | exact_match | verified | 2 |
| Temporal reasoning | exact_match | verified | 2 |
| Reading comprehension | exact_match | verified | 2 |
| Semantic parsing | exact_match | verified | 2 |
| Spelling/grammar correction | exact_match | verified | 2 |
| Cloze completion | exact_match | verified | 2 |
| Anagram/wordplay | exact_match | verified | 2 |
| Natural language inference | exact_match | verified | 2 |
| Knowledge base querying | execution | verified | 2 |
| Formal grammar | execution | verified | 2 |
| Ontology construction | rule | strong | 3 |
| Commonsense reasoning | exact_match | verified | 2 |
| Multi-hop web research | exact_match | strong | 2 |

## Games & Interactive (14 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Chess | rule/outcome | verified | 2 |
| Go | rule/outcome | verified | 2 |
| Board games (Othello, Connect4) | rule/outcome | verified | 2 |
| Card games (poker, bridge) | outcome | strong | 2 |
| Puzzle games (Sokoban, Rush Hour) | simulation | verified | 1 |
| Atari games | outcome | verified | 2 |
| Interactive fiction | outcome | verified | 2 |
| Level generation | constraint | strong | 3 |
| Multi-agent coordination | simulation | strong | 3 |
| Negotiation/bargaining | outcome | strong | 3 |
| Cellular automata | execution | verified | 1 |
| Graph algorithm execution | execution | verified | 1 |
| Proof-of-work puzzles | exact_match | verified | 1 |
| Regex crossword | constraint | verified | 1 |

## Agent & Tool Use (12 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Web navigation (WebArena) | outcome | verified | 2 |
| Computer use (OSWorld) | outcome | strong | 3 |
| GUI navigation | outcome | verified | 2 |
| Map navigation | constraint | strong | 2 |
| File system tasks | constraint | verified | 2 |
| Spreadsheet tasks | diff | strong | 2 |
| Calendar/scheduling | constraint | strong | 2 |
| Email tasks | constraint | strong | 2 |
| Workflow automation | simulation | strong | 3 |
| NL to API | execution | verified | 2 |
| Process mining | execution | strong | 3 |
| Distributed systems | simulation | needs_validation | 3 |

## Vision & Multimodal (14 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Visual QA | exact_match | verified | 2 |
| Document OCR extraction | exact_match | verified | 2 |
| Chart understanding | exact_match | verified | 2 |
| Visual grounding | constraint | verified | 2 |
| Image classification | exact_match | verified | 1 |
| Spatial reasoning | exact_match | verified | 2 |
| Video understanding | exact_match | verified | 2 |
| Multimodal reasoning | exact_match | verified | 2 |
| Image segmentation | execution | verified | 2 |
| Advanced OCR | exact_match | verified | 2 |
| 3D scene understanding | execution | strong | 3 |
| Constrained image generation | execution | strong | 3 |
| CAD modeling | diff | strong | 3 |
| Data visualization | execution | strong | 2 |

## Security (6 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| CTF challenges | exact_match | verified | 3 |
| Defense/hardening | execution | strong | 3 |
| Reverse engineering | exact_match | verified | 3 |
| Mathematical cryptanalysis | execution | verified | 3 |
| Cryptographic protocol design | rule | strong | 3 |
| Cryptography challenges | exact_match | verified | 3 |

## Domain Expert (5 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Medical diagnosis (USMLE) | exact_match | verified | 3 |
| Legal reasoning (Bar exam) | exact_match | verified | 3 |
| Economic modeling | execution | strong | 3 |
| Causal reasoning | exact_match | strong | 3 |
| Bayesian network reasoning | execution | verified | 3 |

## Format-Constrained & Creative (20 domains)

| Domain | Verification | Status | Stage |
|--------|-------------|--------|-------|
| Constrained writing | constraint | verified | 2 |
| Formal poetry (haiku, sonnet) | constraint | strong | 2 |
| Music theory | rule | strong | 3 |
| MIDI generation | rule | strong | 3 |
| Crossword construction | constraint | strong | 3 |
| Sudoku generation | constraint | verified | 2 |
| Data formatting | diff | verified | 2 |
| LaTeX typesetting | execution | strong | 2 |
| HTML/CSS generation | diff | strong | 2 |
| SVG generation | diff | strong | 2 |
| Markdown formatting | diff | verified | 2 |
| Calendar iCal | execution | strong | 2 |
| Config generation | execution | strong | 2 |
| Protocol compliance | execution | strong | 2 |
| Accessibility (WCAG) | execution | strong | 2 |
| ASCII art | constraint | needs_validation | 2 |
| Instruction following (IFEval) | constraint | verified | 2 |
| Educational content | constraint | strong | 2 |
| Error correction codes | execution | verified | 3 |
| Network configuration | simulation | strong | 3 |
