# Dataset Registry

**33 downloaded datasets (2.3GB, ~6M problems)** plus **7 procedural generators** producing unlimited training data.

## Downloaded Datasets

### Math & Numerical Reasoning

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **GSM8K** (train) | 7,473 | math_numerical | Grade-school math word problems |
| **GSM8K** (test) | 1,319 | math_numerical | Held-out evaluation set |
| **MMLU** (57 subjects) | 15,908 | exact_match | Multi-domain multiple choice |

### Code & Programming

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **HumanEval** | 164 | code_execution | OpenAI function-completion benchmark |
| **MBPP** | 974 | code_execution | Mostly Basic Python Programming |
| **WikiSQL** | 80,654 | sql_execution | Text-to-SQL with table contexts |

### Question Answering

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **SQuAD 2.0** (train) | 130K+ | exact_match | Extractive QA with unanswerable questions |
| **SQuAD 2.0** (dev) | 11K+ | exact_match | Evaluation split |
| **TriviaQA** | 95K | exact_match | Trivia with evidence documents |
| **HotpotQA** (val) | 7,405 | exact_match | Multi-hop reasoning QA |
| **CommonsenseQA** | 9,741 | exact_match | Commonsense knowledge questions |
| **WikiTableQuestions** | 22K | exact_match | Questions over Wikipedia tables |
| **COPA** | 1,000 | exact_match | Choice of Plausible Alternatives |

### Fact Verification

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **FEVER** (train) | 145,449 | exact_match | Fact Extraction and Verification |
| **TabFact** | 118K | exact_match | Table-based fact verification |

### NLI & Classification

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **SNLI** | 570K | exact_match | Stanford Natural Language Inference |
| **MultiNLI** | 433K | exact_match | Multi-Genre NLI |
| **ANLI** (all rounds) | 170K | exact_match | Adversarial NLI |
| **SST-2** | 70K | exact_match | Stanford Sentiment Treebank |
| **AG News** (test) | 7,600 | exact_match | News topic classification |

### Commonsense & Science

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **HellaSwag** (val) | 10,042 | exact_match | Commonsense NLI |
| **PIQA** (val) | 1,838 | exact_match | Physical Intuition QA |
| **Winogrande** | 44K | exact_match | Winograd schema challenges |
| **ARC** (Challenge+Easy) | 7,787 | exact_match | AI2 Reasoning Challenge |
| **OpenBookQA** | 5,957 | exact_match | Open-book science QA |
| **SciQ** | 13,679 | exact_match | Science exam questions |

### Medical

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **MedQA** (USMLE) | 10K+ | exact_match | US Medical Licensing Exam questions |

### Logic & Games

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **Lichess puzzles** | 3M+ | rule (chess engine) | Chess tactical puzzles |
| **Sudoku puzzles** | 1M+ | sudoku | Constraint satisfaction puzzles |

### Instruction Following

| Dataset | Problems | Verifier | Notes |
|---------|----------|----------|-------|
| **IFEval** | 541 | instruction_following | Multi-constraint instruction adherence |

## Procedural Generators (Unlimited)

These generate fresh training data at runtime — no download needed, no data exhaustion:

| Generator | Method | Rate | Example Tasks |
|-----------|--------|------|---------------|
| **unit_conversion** | Random value + random unit pair | 100K/sec | "Convert 3.5 miles to kilometers" |
| **date_time** | Random dates + random operations | 100K/sec | "What day of the week is 2024-03-15?" |
| **chemical_equation** | Random formulas, compute balance | 10K/sec | "Balance: Fe + O2 → Fe2O3" |
| **sudoku** | Constraint-based with uniqueness | 1K/sec | Complete 9x9 Sudoku grids |
| **regex_synthesis** | Sample patterns, generate examples | 50K/sec | "Write regex matching these: ..." |
| **instruction_following** | Compose constraint templates | 50K/sec | "Write 200 words about X, include Y" |
| **json_schema** | Random schema + valid/invalid instances | 50K/sec | "Generate JSON matching this schema" |
| **graph_properties** | Erdos-Renyi graphs, compute properties | 10K/sec | "Find shortest path from A to B" |

## Datasets Requiring Authentication

These need HuggingFace login (`huggingface-cli login`):

| Dataset | Problems | Domain | Priority |
|---------|----------|--------|----------|
| **MATH** (Hendrycks) | 12.5K | Competition math | Critical |
| **APPS** | 10K | Competitive programming | Critical |
| **SWE-bench** | 2,294 | Real-world code repair | Critical |
| **Natural Questions** | 307K | Open-domain QA | High |
| **Spider** | 10K | Complex text-to-SQL | High |
| **BIRD** | 12.7K | Text-to-SQL | High |
| **MedMCQA** | 194K | Medical QA | Medium |
| **LegalBench** | varies | Legal reasoning | Medium |
| **DS-1000** | 1K | Data science code | Medium |

```bash
# Download authenticated datasets
huggingface-cli login
huggingface-cli download hendrycks/competition_math --repo-type dataset
huggingface-cli download codeparrot/apps --repo-type dataset
huggingface-cli download princeton-nlp/SWE-bench --repo-type dataset
```

## Additional Evaluation Datasets

| Dataset | Size | Domain |
|---------|------|--------|
| Omni-MATH | 4K+ | Math competition |
| TheoremQA | 800 | Multi-domain math |
| BigCodeBench | 1.1K | Code tasks |
| CRUXEval | 800 | Code understanding |
| ARC-AGI | 1K | Abstract reasoning |
| LeanDojo | 98K theorems | Formal proofs |

## Pre-Training & SFT Sources (Upstream)

These produce the base policy before RLVR begins:

### Pre-Training Corpora

| Corpus | Size | Content |
|--------|------|---------|
| FineWeb-Edu | 1.3T tokens | Education-filtered web text |
| The Stack v2 | 67TB | Code in 600+ languages |
| ProofPile-2 | 55B tokens | Math-heavy text |
| peS2o | 40B tokens | Science papers |
| Dolma | 3T tokens | Books/encyclopedias |
| ROOTS | 1.6TB | Multilingual text |

**Recommended mix:** 40% web, 25% code, 15% science/math, 10% books, 10% multilingual.

### SFT Datasets

| Dataset | Size | Focus |
|---------|------|-------|
| Tulu 2 Mix | 326K | Curated instruction mix |
| OpenHermes 2.5 | 1M | Diverse instructions |
| FLAN | 1,836 tasks | Diverse NLP |
| NuminaMath-CoT | 860K | Math with chain-of-thought |
| MAmmoTH | 260K | Math reasoning |
| Magicoder-OSS | 75K | Code instructions |
| CodeFeedback | 66K | Code with feedback |

## Interactive Environments (for Agent RL)

| Environment | Tasks | Domain |
|-------------|-------|--------|
| WebArena | 812 | Web navigation |
| VisualWebArena | 910 | Visual web navigation |
| OSWorld | 369 | Computer use |
| Gymnasium | 100+ | Classic RL (Atari, MuJoCo) |
| TextWorld | proc-gen | Text adventures |
| ScienceWorld | 30 | Science experiments |
| PettingZoo | 50+ | Multi-agent |

## Coverage Matrix

| Verifier | Datasets | Total Problems | Procedural? |
|----------|----------|----------------|-------------|
| math_numerical | GSM8K | 8,792 | No |
| math_equivalence | GSM8K, MMLU-math | 10K+ | No |
| exact_match | SQuAD, TriviaQA, FEVER, SNLI, etc. | 1.5M+ | No |
| instruction_following | IFEval | 541 | Yes |
| json_schema | — | Generated | Yes |
| code_execution | HumanEval, MBPP | 1,138 | No |
| sudoku | Sudoku data | 1M+ | Yes |
| chemical_equation | — | Generated | Yes |
| regex_synthesis | — | Generated | Yes |
| unit_conversion | — | Generated | Yes |
| date_time | — | Generated | Yes |
| sql_execution | WikiSQL | 80,654 | No |
| graph_properties | — | Generated | Yes |
