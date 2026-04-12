# RLVR Environments Wiki — Master Index

**Total domains: 212** | **Verification types: 9** | **Target: AGI-scale verifiable training**

## Overview & Synthesis
- [Overview](overview.md) — High-level thesis, taxonomy, and path to AGI
- [Scaling Roadmap](synthesis/scaling-roadmap.md) — Phased plan from current RLVR to AGI
- [Dataset Sources](synthesis/dataset-sources.md) — Every internet dataset mapped to RLVR domains

## Concepts
- [Verification Types](concepts/verification-types.md) — Taxonomy of 9 verification mechanisms
- [Reward Shaping](concepts/reward-shaping.md) — Designing effective rewards across domains
- [Dataset Scaling](concepts/dataset-scaling.md) — From internet mining to unlimited procedural generation

---

## Domain Index by Category

### Mathematics (29 domains)
- [math-competition](domains/math-competition.md) — AMC/AIME/Olympiad problems [exact_match, verified]
- [math-numerical](domains/math-numerical.md) — Numerical computation, GSM8K, AMPS [exact_match, verified]
- [math-formal-proofs](domains/math-formal-proofs.md) — Lean4/Coq/Isabelle theorem proving [execution, verified]
- [math-symbolic](domains/math-symbolic.md) — Symbolic algebra/calculus via CAS [execution, strong]
- [math-theorem-conjecturing](domains/math-theorem-conjecturing.md) — Generating true conjectures [execution, needs_validation]
- [math-word-problems-visual](domains/math-word-problems-visual.md) — Visual math (GeoQA, MathVista) [exact_match, verified]
- [math-olympiad-inequalities](domains/math-olympiad-inequalities.md) — Inequality proofs & bounds [execution, strong]
- [number-theory-computation](domains/number-theory-computation.md) — Primality, factoring, modular arithmetic [execution, strong]
- [geometry-computation](domains/geometry-computation.md) — Computational geometry [execution, strong]
- [geometric-construction](domains/geometric-construction.md) — Compass-and-straightedge [simulation, strong]
- [probability-statistics](domains/probability-statistics.md) — Probability/stats problems [exact_match, strong]
- [abstract-algebra](domains/abstract-algebra.md) — Group/ring theory via GAP/SageMath [execution, needs_validation]
- [graph-theory](domains/graph-theory.md) — Graph properties & algorithms [execution, strong]
- [combinatorics-optimization](domains/combinatorics-optimization.md) — TSP, bin packing, scheduling [execution, strong]
- [linear-algebra-computation](domains/linear-algebra-computation.md) — Matrix operations, eigenvalues [execution, verified]
- [differential-equations](domains/differential-equations.md) — ODE & PDE solving [execution, verified]
- [topology-computation](domains/topology-computation.md) — Betti numbers, persistent homology [execution, strong]
- [category-theory](domains/category-theory.md) — Categorical constructs [execution, needs_validation]
- [knot-theory](domains/knot-theory.md) — Knot invariants [execution, strong]
- [information-theory](domains/information-theory.md) — Entropy, channel capacity, codes [execution, verified]
- [symbolic-regression](domains/symbolic-regression.md) — Find expressions from data [execution, verified]
- [mathematical-modeling](domains/mathematical-modeling.md) — Applied modeling problems [execution, strong]
- [mathematical-programming](domains/mathematical-programming.md) — LP/IP/QP/SDP [execution, verified]
- [mathematical-conjecture-testing](domains/mathematical-conjecture-testing.md) — Counterexample finding [execution, strong]
- [theorem-proving-natural-language](domains/theorem-proving-natural-language.md) — NL proofs [execution, strong]
- [theorem-application](domains/theorem-application.md) — Strategy selection [execution, strong]
- [theorem-discovery](domains/theorem-discovery.md) — Novel theorem generation [rule, needs_validation]
- [tensor-computation](domains/tensor-computation.md) — Tensor ops & einsum [execution, verified]
- [satisfiability-modular-arithmetic](domains/satisfiability-modular-arithmetic.md) — Finite field computation [execution, verified]

### Logic & Formal Methods (12 domains)
- [logic-propositional](domains/logic-propositional.md) — SAT solving [execution, strong]
- [logic-first-order](domains/logic-first-order.md) — FOL / ATP [execution, strong]
- [logic-puzzles](domains/logic-puzzles.md) — Sudoku, constraint puzzles [constraint, strong]
- [logic-formal-specification](domains/logic-formal-specification.md) — TLA+, Alloy [execution, needs_validation]
- [smt-solving](domains/smt-solving.md) — SMT-LIB benchmarks [execution, verified]
- [automated-reasoning](domains/automated-reasoning.md) — General reasoning [rule, strong]
- [automated-theorem-proving](domains/automated-theorem-proving.md) — TPTP first-order proofs [rule, verified]
- [theorem-formalization](domains/theorem-formalization.md) — NL → Lean4/Coq [rule, strong]
- [proof-repair](domains/proof-repair.md) — Fix broken formal proofs [rule, strong]
- [formal-language-theory](domains/formal-language-theory.md) — DFA/NFA/grammar [execution, strong]
- [boolean-function-learning](domains/boolean-function-learning.md) — Learn Boolean functions [execution, verified]
- [type-theory-puzzles](domains/type-theory-puzzles.md) — Curry-Howard [rule, strong]

### Code & Software (26 domains)
- [code-generation](domains/code-generation.md) — HumanEval/APPS/CodeContests [execution, verified]
- [code-repair](domains/code-repair.md) — Bug fixing (SWE-bench) [execution, verified]
- [code-translation](domains/code-translation.md) — Cross-language [execution, strong]
- [code-optimization](domains/code-optimization.md) — Performance optimization [execution, strong]
- [code-refactoring](domains/code-refactoring.md) — Behavior-preserving refactoring [execution, strong]
- [code-documentation](domains/code-documentation.md) — Doctest verification [execution, strong]
- [sql-generation](domains/sql-generation.md) — Text-to-SQL (Spider, BIRD) [execution, verified]
- [regex-synthesis](domains/regex-synthesis.md) — Regex from examples [execution, strong]
- [shell-commands](domains/shell-commands.md) — Bash generation [execution, strong]
- [type-inference](domains/type-inference.md) — Type annotation [execution, strong]
- [compiler-tasks](domains/compiler-tasks.md) — Decompilation, optimization [execution, strong]
- [compiler-construction](domains/compiler-construction.md) — Build interpreters/compilers [execution, strong]
- [compiler-optimization-passes](domains/compiler-optimization-passes.md) — Pass ordering [execution, strong]
- [api-usage](domains/api-usage.md) — API call generation [execution, strong]
- [natural-language-to-api](domains/natural-language-to-api.md) — NL → API [execution, verified]
- [infrastructure-as-code](domains/infrastructure-as-code.md) — Terraform/Docker [execution, strong]
- [data-wrangling](domains/data-wrangling.md) — Pandas transformations [execution, strong]
- [web-scraping](domains/web-scraping.md) — Scraping code generation [execution, strong]
- [test-generation](domains/test-generation.md) — Test creation [execution, strong]
- [database-migrations](domains/database-migrations.md) — Schema migrations [execution, strong]
- [build-configuration](domains/build-configuration.md) — Build system configs [execution, strong]
- [database-query-optimization](domains/database-query-optimization.md) — Query optimization [execution, strong]
- [competitive-coding-interactive](domains/competitive-coding-interactive.md) — CP with judges [execution, verified]
- [competitive-programming-interactive](domains/competitive-programming-interactive.md) — Interactive problems [execution, verified]
- [program-synthesis-from-io](domains/program-synthesis-from-io.md) — ARC, FlashFill [execution, verified]
- [program-equivalence](domains/program-equivalence.md) — Semantic equivalence [execution, verified]
- [functional-programming](domains/functional-programming.md) — Haskell/OCaml challenges [execution, verified]
- [text-to-code-nl](domains/text-to-code-nl.md) — NL to code beyond CP [execution, strong]
- [memory-management](domains/memory-management.md) — Low-level systems [execution, strong]
- [dependency-resolution](domains/dependency-resolution.md) — Package management [constraint, strong]
- [spreadsheet-formulas](domains/spreadsheet-formulas.md) — Excel/Sheets formulas [execution, verified]

### Science & Engineering (27 domains)
- [physics-simulation](domains/physics-simulation.md) — Physics engine verification [simulation, strong]
- [chemistry-computation](domains/chemistry-computation.md) — RDKit/OpenBabel [execution, strong]
- [chemistry-retrosynthesis](domains/chemistry-retrosynthesis.md) — Synthesis planning [execution, strong]
- [biology-sequence](domains/biology-sequence.md) — Protein/DNA tasks [execution, strong]
- [molecular-generation](domains/molecular-generation.md) — Drug design [execution, strong]
- [protein-design](domains/protein-design.md) — Protein engineering [simulation, strong]
- [dna-sequence-design](domains/dna-sequence-design.md) — DNA/RNA primer design [execution, strong]
- [genomics-bioinformatics](domains/genomics-bioinformatics.md) — Variant calling, alignment [execution, strong]
- [phylogenetics](domains/phylogenetics.md) — Evolutionary trees [execution, strong]
- [circuit-design](domains/circuit-design.md) — SPICE simulation [simulation, strong]
- [analog-circuit-design](domains/analog-circuit-design.md) — Analog circuits [simulation, strong]
- [chip-design-rtl](domains/chip-design-rtl.md) — Verilog/VHDL [simulation, verified]
- [control-systems](domains/control-systems.md) — Controller design [simulation, strong]
- [signal-processing](domains/signal-processing.md) — DSP tasks [execution, strong]
- [materials-science](domains/materials-science.md) — Material properties [execution, strong]
- [engineering-optimization](domains/engineering-optimization.md) — Structural optimization [simulation, strong]
- [fluid-dynamics](domains/fluid-dynamics.md) — CFD problems [simulation, strong]
- [electrical-engineering](domains/electrical-engineering.md) — EE problems [execution, strong]
- [astronomy-computation](domains/astronomy-computation.md) — Orbital mechanics [execution, strong]
- [quantum-computing](domains/quantum-computing.md) — Quantum circuits [simulation, strong]
- [robotics-planning](domains/robotics-planning.md) — Motion planning [simulation, strong]
- [climate-science](domains/climate-science.md) — Climate computation [execution, strong]
- [climate-weather](domains/climate-weather.md) — Weather prediction [execution, strong]
- [epidemiology-modeling](domains/epidemiology-modeling.md) — SIR/SEIR models [execution, strong]
- [pde-numerical](domains/pde-numerical.md) — Numerical PDE solving [execution, strong]
- [scientific-experiment-design](domains/scientific-experiment-design.md) — Experiment design [constraint, needs_validation]
- [scientific-paper-extraction](domains/scientific-paper-extraction.md) — Paper IE [exact_match, verified]

### Language & Knowledge (27 domains)
- [information-extraction](domains/information-extraction.md) — NER, relation extraction [exact_match, verified]
- [fact-verification](domains/fact-verification.md) — FEVER, TabFact [exact_match, verified]
- [question-answering-extractive](domains/question-answering-extractive.md) — SQuAD, NQ [exact_match, verified]
- [question-answering-closed](domains/question-answering-closed.md) — Trivia, Jeopardy [exact_match, verified]
- [translation-reference](domains/translation-reference.md) — MT with references [approximate, verified]
- [text-classification](domains/text-classification.md) — Sentiment, topic [exact_match, verified]
- [structured-output-generation](domains/structured-output-generation.md) — JSON/XML schema [exact_match, verified]
- [document-parsing](domains/document-parsing.md) — Document structure [exact_match, verified]
- [knowledge-graph-completion](domains/knowledge-graph-completion.md) — KG link prediction [exact_match, verified]
- [citation-verification](domains/citation-verification.md) — Source verification [exact_match, verified]
- [data-entry-correction](domains/data-entry-correction.md) — OCR/data correction [exact_match, verified]
- [table-understanding](domains/table-understanding.md) — Table QA [exact_match, verified]
- [summarization-extractive](domains/summarization-extractive.md) — ROUGE scores [approximate, verified]
- [entity-linking](domains/entity-linking.md) — Entity disambiguation [exact_match, verified]
- [temporal-reasoning](domains/temporal-reasoning.md) — Time-based questions [exact_match, verified]
- [multilingual-tasks](domains/multilingual-tasks.md) — Cross-lingual NLU [exact_match, verified]
- [reading-comprehension](domains/reading-comprehension.md) — Multi-hop RC [exact_match, verified]
- [semantic-parsing](domains/semantic-parsing.md) — NL → formal [exact_match, verified]
- [spelling-grammar](domains/spelling-grammar.md) — Error correction [exact_match, verified]
- [cloze-completion](domains/cloze-completion.md) — Fill-in-blanks [exact_match, verified]
- [anagram-wordplay](domains/anagram-wordplay.md) — Word puzzles [exact_match, verified]
- [natural-language-inference](domains/natural-language-inference.md) — NLI (SNLI, ANLI) [exact_match, verified]
- [knowledge-base-querying](domains/knowledge-base-querying.md) — SPARQL/Cypher [execution, verified]
- [formal-grammar](domains/formal-grammar.md) — Parsing & grammar induction [execution, verified]
- [ontology-construction](domains/ontology-construction.md) — OWL reasoning [rule, strong]
- [commonsense-reasoning](domains/commonsense-reasoning.md) — Physical/social commonsense [exact_match, verified]
- [multi-hop-web-research](domains/multi-hop-web-research.md) — Multi-hop search [exact_match, strong]

### Games & Interactive (14 domains)
- [chess](domains/chess.md) — Chess puzzles & play [rule/outcome, verified]
- [go-game](domains/go-game.md) — Go playing [rule/outcome, verified]
- [board-games](domains/board-games.md) — Othello, Connect4, etc. [rule/outcome, verified]
- [card-games](domains/card-games.md) — Poker, bridge [outcome, strong]
- [puzzle-games](domains/puzzle-games.md) — Sokoban, Rush Hour [simulation, verified]
- [game-playing-atari](domains/game-playing-atari.md) — Atari games [outcome, verified]
- [interactive-fiction](domains/interactive-fiction.md) — Text adventures [outcome, verified]
- [game-design-level-generation](domains/game-design-level-generation.md) — Level creation [constraint, strong]
- [multi-agent-coordination](domains/multi-agent-coordination.md) — Nash equilibria [simulation, strong]
- [negotiation-bargaining](domains/negotiation-bargaining.md) — Bargaining games [outcome, strong]
- [cellular-automata](domains/cellular-automata.md) — Game of Life, CA prediction [execution, verified]
- [graph-algorithm-execution](domains/graph-algorithm-execution.md) — Manual graph algorithms [execution, verified]
- [proof-of-work-puzzles](domains/proof-of-work-puzzles.md) — Hash puzzles [exact_match, verified]
- [regex-crossword](domains/regex-crossword.md) — Regex puzzles [constraint, verified]

### Agent & Tool Use (12 domains)
- [web-navigation](domains/web-navigation.md) — WebArena, MiniWoB++ [outcome, verified]
- [computer-use](domains/computer-use.md) — OS interaction (OSWorld) [outcome, strong]
- [gui-navigation](domains/gui-navigation.md) — Mobile app navigation [outcome, verified]
- [map-navigation](domains/map-navigation.md) — Route finding [constraint, strong]
- [file-system-tasks](domains/file-system-tasks.md) — File manipulation [constraint, verified]
- [spreadsheet-tasks](domains/spreadsheet-tasks.md) — Spreadsheet operations [diff, strong]
- [calendar-scheduling](domains/calendar-scheduling.md) — Constraint scheduling [constraint, strong]
- [email-tasks](domains/email-tasks.md) — Email requirements [constraint, strong]
- [workflow-automation](domains/workflow-automation.md) — IFTTT/Zapier style [simulation, strong]
- [natural-language-to-api](domains/natural-language-to-api.md) — API orchestration [execution, verified]
- [process-mining](domains/process-mining.md) — Workflow discovery [execution, strong]
- [distributed-systems](domains/distributed-systems.md) — Protocol design [simulation, needs_validation]

### Vision & Multimodal (14 domains)
- [visual-question-answering](domains/visual-question-answering.md) — VQA [exact_match, verified]
- [document-ocr-extraction](domains/document-ocr-extraction.md) — Document extraction [exact_match, verified]
- [chart-understanding](domains/chart-understanding.md) — Chart QA [exact_match, verified]
- [visual-grounding](domains/visual-grounding.md) — Object localization [constraint, verified]
- [image-classification](domains/image-classification.md) — Image labels [exact_match, verified]
- [spatial-reasoning](domains/spatial-reasoning.md) — Spatial relations [exact_match, verified]
- [video-understanding](domains/video-understanding.md) — Video QA [exact_match, verified]
- [multimodal-reasoning](domains/multimodal-reasoning.md) — Multi-modal reasoning [exact_match, verified]
- [image-segmentation](domains/image-segmentation.md) — Detection & segmentation [execution, verified]
- [optical-character-recognition](domains/optical-character-recognition.md) — Advanced OCR [exact_match, verified]
- [3d-scene-understanding](domains/3d-scene-understanding.md) — 3D perception [execution, strong]
- [image-generation-constrained](domains/image-generation-constrained.md) — Code-based image gen [execution, strong]
- [cad-modeling](domains/cad-modeling.md) — 3D CAD generation [diff, strong]
- [data-visualization](domains/data-visualization.md) — Chart/viz code generation [execution, strong]

### Audio (2 domains)
- [audio-speech-recognition](domains/audio-speech-recognition.md) — ASR [exact_match, verified]
- [music-audio-processing](domains/music-audio-processing.md) — Music transcription [execution, strong]

### Format-Constrained & Creative (20 domains)
- [constrained-writing](domains/constrained-writing.md) — Constraint satisfaction writing [constraint, verified]
- [poetry-formal](domains/poetry-formal.md) — Formal poetry (haiku, sonnet) [constraint, strong]
- [music-theory](domains/music-theory.md) — Music theory rules [rule, strong]
- [music-generation-midi](domains/music-generation-midi.md) — MIDI with theory constraints [rule, strong]
- [crossword-construction](domains/crossword-construction.md) — Valid crosswords [constraint, strong]
- [sudoku-generation](domains/sudoku-generation.md) — Unique-solution sudoku [constraint, verified]
- [data-formatting](domains/data-formatting.md) — Format conversion [diff, verified]
- [latex-typesetting](domains/latex-typesetting.md) — LaTeX generation [execution, strong]
- [html-css-generation](domains/html-css-generation.md) — Visual web generation [diff, strong]
- [svg-generation](domains/svg-generation.md) — SVG images [diff, strong]
- [markdown-formatting](domains/markdown-formatting.md) — Markdown conversion [diff, verified]
- [calendar-ical](domains/calendar-ical.md) — iCal file generation [execution, strong]
- [config-generation](domains/config-generation.md) — Config files [execution, strong]
- [protocol-compliance](domains/protocol-compliance.md) — Protocol messages [execution, strong]
- [accessibility-compliance](domains/accessibility-compliance.md) — WCAG checking [execution, strong]
- [ascii-art](domains/ascii-art.md) — ASCII art [constraint, needs_validation]
- [instruction-following](domains/instruction-following.md) — IFEval [constraint, verified]
- [educational-content](domains/educational-content.md) — Quiz/exercise generation [constraint, strong]
- [error-correction-codes](domains/error-correction-codes.md) — ECC design [execution, verified]
- [network-configuration](domains/network-configuration.md) — Network config [simulation, strong]

### Miscellaneous Computation (12 domains)
- [unit-conversion](domains/unit-conversion.md) — Unit conversion [exact_match, verified]
- [date-time-computation](domains/date-time-computation.md) — Date/time calculations [exact_match, verified]
- [financial-calculation](domains/financial-calculation.md) — Financial math [exact_match, verified]
- [recipe-scaling](domains/recipe-scaling.md) — Recipe scaling [exact_match, verified]
- [encryption-decryption](domains/encryption-decryption.md) — Crypto operations [exact_match, verified]
- [compression-encoding](domains/compression-encoding.md) — Encoding/decoding [exact_match, verified]
- [legal-logic](domains/legal-logic.md) — Legal rule application [rule, strong]

### Domain Expert Knowledge (5 domains)
- [medical-diagnosis](domains/medical-diagnosis.md) — Medical QA (USMLE) [exact_match, verified]
- [legal-reasoning](domains/legal-reasoning.md) — Legal reasoning (Bar exam) [exact_match, verified]
- [economic-modeling](domains/economic-modeling.md) — Economics problems [execution, strong]
- [causal-reasoning](domains/causal-reasoning.md) — Causal inference [exact_match, strong]
- [bayesian-network-reasoning](domains/bayesian-network-reasoning.md) — Probabilistic reasoning [execution, verified]

### Security (5 domains)
- [cybersecurity-ctf](domains/cybersecurity-ctf.md) — CTF challenges [exact_match, verified]
- [cybersecurity-defense](domains/cybersecurity-defense.md) — Defense & hardening [execution, strong]
- [reverse-engineering](domains/reverse-engineering.md) — Binary analysis [exact_match, verified]
- [mathematical-cryptanalysis](domains/mathematical-cryptanalysis.md) — Cryptanalysis [execution, verified]
- [cryptographic-protocol-design](domains/cryptographic-protocol-design.md) — Protocol verification [rule, strong]
- [cryptography-challenges](domains/cryptography-challenges.md) — General crypto [exact_match, verified]

### Machine Learning (2 domains)
- [ml-pipeline-optimization](domains/ml-pipeline-optimization.md) — AutoML pipelines [execution, strong]
- [data-science-eda](domains/data-science-eda.md) — Statistical analysis [execution, strong]

### Systems (4 domains)
- [compiler-construction](domains/compiler-construction.md) — Build compilers [execution, strong]
- [distributed-systems](domains/distributed-systems.md) — Protocol design [simulation, needs_validation]
- [network-configuration](domains/network-configuration.md) — Network config [simulation, strong]
- [scheduling](domains/scheduling.md) — Job scheduling [constraint, verified]
- [planning-classical](domains/planning-classical.md) — PDDL planning [simulation, verified]

---

## Statistics

| Verification Type | Count | Reliability |
|-------------------|-------|-------------|
| exact_match | ~55 | Excellent |
| execution | ~85 | Excellent |
| constraint | ~20 | Excellent |
| rule | ~15 | Excellent |
| simulation | ~15 | Very Good |
| diff | ~10 | Good |
| outcome | ~8 | Good |
| approximate | ~4 | Acceptable |
| **Total** | **~212** | |

| Status | Count |
|--------|-------|
| verified | ~100 |
| strong_hypothesis | ~100 |
| needs_validation | ~12 |
