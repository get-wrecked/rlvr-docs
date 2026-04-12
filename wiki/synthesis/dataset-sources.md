---
title: Internet Dataset Sources for RLVR
description: Comprehensive catalog of internet-scale datasets that provide verifiable rewards
---

# Internet Dataset Sources for RLVR

Master list of every known internet dataset that can generate verifiable rewards for RL training. Organized by dataset → domains it supports.

## Mega-Scale Sources (>10M problem instances extractable)

### GitHub (100M+ repositories)
- **Function-test pairs**: Extract functions with their tests. Each is a code generation task with execution-based verification.
- **Commit-diff pairs**: Code repair tasks.
- **Cross-language implementations**: Code translation tasks.
- **Build files**: Infrastructure-as-code tasks.
- **SQL files**: SQL generation tasks.
- **Regex in code**: Regex synthesis tasks.
- **Config files**: Config generation tasks.
- **Jupyter notebooks**: Data science tasks.
- **Domains**: code-generation, code-repair, code-translation, sql-generation, regex-synthesis, infrastructure-as-code, data-wrangling, test-generation

### Wikipedia / Wikidata (6.7M articles / 100B+ triples)
- **Wikidata structured data**: QA pairs (extract entity → property → value).
- **Wikipedia infoboxes**: Structured information extraction tasks.
- **Wikipedia revision history**: Edit task generation, fact verification.
- **Inter-article links**: Entity linking training data.
- **Wikidata SPARQL**: Knowledge base querying tasks.
- **Domains**: question-answering-closed, information-extraction, entity-linking, fact-verification, knowledge-base-querying, knowledge-graph-completion

### StackOverflow / StackExchange (23M+ questions)
- **Accepted answers with code**: Code generation tasks (extract question + test from answer).
- **Math StackExchange**: Mathematical problems with verified answers.
- **GIS StackExchange**: Geospatial analysis tasks.
- **Electronics StackExchange**: Circuit design tasks.
- **CS Theory StackExchange**: Theory problems.
- **Domains**: code-generation, sql-generation, regex-synthesis, shell-commands, math-numerical, geospatial-analysis

### Common Crawl (petabytes of web data)
- **Structured data (Schema.org markup)**: Structured output generation.
- **Tables**: Table understanding tasks.
- **Multilingual text**: Translation and multilingual tasks.
- **Forms**: Structured extraction tasks.
- **Domains**: structured-output-generation, table-understanding, translation-reference, information-extraction

### arXiv (2.5M+ papers)
- **LaTeX source**: LaTeX generation/compilation verification.
- **Theorem-proof pairs**: Math reasoning tasks.
- **Figures + captions**: Visual understanding tasks.
- **Citation networks**: Citation verification.
- **Domains**: math-formal-proofs, latex-typesetting, scientific-paper-extraction, citation-verification

### PubMed / PMC (36M+ abstracts / 8M+ full-text)
- **Medical QA**: Clinical reasoning tasks.
- **Biomedical NER**: Entity extraction.
- **Drug-gene-disease relations**: Relation extraction.
- **Domains**: medical-diagnosis, information-extraction, scientific-paper-extraction, genomics-bioinformatics

## Large-Scale Curated Datasets (1M-10M instances)

### Competitive Programming Archives
| Source | Size | Domains |
|--------|------|---------|
| Codeforces | 10K+ problems, 50M+ submissions | code-generation, competitive-coding-interactive |
| LeetCode | 3K+ problems | code-generation, code-optimization |
| AtCoder | 5K+ problems | code-generation, competitive-coding-interactive |
| HackerRank | 2K+ problems | code-generation |
| Project Euler | 800+ problems | math-numerical, code-generation |
| USACO | 1K+ problems | code-generation |
| Google Code Jam archives | 500+ problems | code-generation |

### Mathematics
| Source | Size | Domains |
|--------|------|---------|
| MATH dataset | 12.5K | math-competition, math-numerical |
| GSM8K | 8.8K | math-numerical |
| AMPS | 100K+ | math-numerical, math-symbolic |
| AoPS problems | 50K+ | math-competition |
| Mathlib4 (Lean) | 100K+ theorems | math-formal-proofs |
| miniF2F | 488 | math-formal-proofs |
| OEIS | 360K+ sequences | number-theory-computation |
| Omni-MATH | 4K+ | math-competition |

### Natural Language Understanding
| Source | Size | Domains |
|--------|------|---------|
| SQuAD 2.0 | 150K | question-answering-extractive |
| Natural Questions | 300K | question-answering-extractive |
| TriviaQA | 95K | question-answering-closed |
| HotpotQA | 113K | reading-comprehension |
| SNLI | 570K | natural-language-inference |
| MultiNLI | 433K | natural-language-inference |
| FEVER | 185K | fact-verification |
| CoNLL 2003 | 20K sentences | information-extraction |
| DocRED | 5K documents | information-extraction |
| Universal Dependencies | 200+ treebanks | formal-grammar |
| WMT (all years) | 10M+ sentence pairs | translation-reference |

### Code & Software
| Source | Size | Domains |
|--------|------|---------|
| HumanEval | 164 | code-generation |
| MBPP | 1K | code-generation |
| APPS | 10K | code-generation |
| CodeContests | 10K | code-generation |
| Spider | 10K | sql-generation |
| BIRD | 12K | sql-generation |
| SWE-bench | 2.3K | code-repair |
| DS-1000 | 1K | data-wrangling |
| LiveCodeBench | continuous | code-generation |

### Vision
| Source | Size | Domains |
|--------|------|---------|
| COCO | 330K images | image-segmentation, visual-grounding |
| ImageNet | 14M images | image-classification |
| VQAv2 | 1.1M QA pairs | visual-question-answering |
| GQA | 22M QA pairs | visual-question-answering |
| ChartQA | 33K | chart-understanding |
| DocVQA | 50K | document-ocr-extraction |
| RefCOCO | 142K | visual-grounding |
| ScienceQA | 21K | multimodal-reasoning |
| SA-1B | 1B masks | image-segmentation |

### Games
| Source | Size | Domains |
|--------|------|---------|
| Lichess database | 4B+ games | chess |
| Lichess puzzles | 3M+ | chess |
| KGS Go database | 200K+ games | go-game |
| ALE (Atari) | 57 games | game-playing-atari |
| WebArena | 812 tasks | web-navigation |
| MiniWoB++ | 100 tasks | web-navigation |
| OSWorld | 369 tasks | computer-use |

### Science
| Source | Size | Domains |
|--------|------|---------|
| PDB | 200K+ structures | protein-design, biology-sequence |
| UniProt | 250M+ sequences | biology-sequence |
| PubChem | 110M+ compounds | molecular-generation, chemistry-computation |
| ChEMBL | 2M+ bioactive molecules | molecular-generation |
| Materials Project | 150K+ materials | materials-science |
| NCBI GenBank | 250M+ sequences | dna-sequence-design, genomics-bioinformatics |
| ERA5 | Global reanalysis | climate-science |

### Audio
| Source | Size | Domains |
|--------|------|---------|
| LibriSpeech | 960 hours | audio-speech-recognition |
| Common Voice | 20K+ hours | audio-speech-recognition |
| AudioSet | 2M+ clips | audio-speech-recognition |
| MAESTRO | 200+ hours | music-audio-processing |
| Lakh MIDI | 170K files | music-generation-midi |

## Medium-Scale Curated (10K-1M instances)

### CTF / Security
- CTFtime.org: 10K+ challenges | cybersecurity-ctf
- CryptoHack: 200+ crypto challenges | mathematical-cryptanalysis
- CrackMes.one: 5K+ RE challenges | reverse-engineering
- picoCTF: 500+ challenges | cybersecurity-ctf

### Logic & Formal Methods
- SMT-LIB: 100K+ benchmarks | smt-solving
- TPTP: 25K+ FOL problems | logic-first-order
- SAT Competition: 1K+ instances/year | logic-propositional
- IPC Planning: 100s of instances/domain | planning-classical

### Operations Research
- MIPLIB: 1K+ instances | mathematical-programming
- TSPLIB: 100+ instances | combinatorics-optimization
- OR-Library: 1K+ instances | scheduling, combinatorics-optimization
- PSPLIB: 2K+ instances | scheduling

### Hardware
- HDLBits: 170+ Verilog exercises | chip-design-rtl
- ISCAS benchmarks: 100s circuits | circuit-design, analog-circuit-design
- OpenCores: 1K+ designs | chip-design-rtl

### Miscellaneous
- Sudoku puzzles: millions available | logic-puzzles
- Crossword databases: 100K+ clues | anagram-wordplay, crossword-construction
- Recipe databases: 2M+ recipes | recipe-scaling
- IFTTT recipes: 300K+ | workflow-automation
- Legal benchmarks (LegalBench): 162 tasks | legal-reasoning
- BPI Challenge: annual process logs | process-mining

## Procedurally Generatable (Unlimited)

These domains can generate unlimited training data procedurally:

| Domain | Generation Method |
|--------|-------------------|
| math-numerical | Sample random equations, compute solutions |
| logic-puzzles | Constraint-based puzzle generation |
| logic-propositional | Random SAT/CSP generation |
| code-generation | Extract from repos + mutate |
| regex-synthesis | Generate from random patterns |
| combinatorics-optimization | Random instance generation |
| unit-conversion | Infinite from conversion tables |
| date-time-computation | Random date problems |
| constrained-writing | Template-based constraint generation |
| data-formatting | Convert between any formats |
| graph-algorithm-execution | Random graph generation |
| boolean-function-learning | Random Boolean functions |
| cellular-automata | Random initial states |
| linear-algebra-computation | Random matrices |
| information-theory | Random distributions |
| puzzle-games | Procedural level generation |
| scheduling | Random task/resource generation |
| bayesian-network-reasoning | Random BN generation |
| tensor-computation | Random tensor shapes |
| differential-equations | Work backwards from solutions |
| symbolic-regression | Generate expression → evaluate → use as task |

## Key Insight

The total volume of verifiable training data on the internet exceeds **1 billion** extractable problem-solution pairs. With procedural generation, it is **effectively unlimited**. The bottleneck is not data — it is building the verification infrastructure and managing the curriculum across 200+ domains.
