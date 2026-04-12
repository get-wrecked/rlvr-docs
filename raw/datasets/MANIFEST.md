# Dataset Manifest

**33 files, 2.3GB downloaded.** All datasets needed for RLVR policy pre-training and verifier evaluation.

## Status: ✅ = Downloaded | 🔗 = Needs Auth/Manual | 🔧 = Procedural (unlimited)

---

## Downloaded Datasets (33 files, 2.3GB)

### Math & Numerical Reasoning
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| GSM8K train | 7,473 | `math/gsm8k_train.jsonl` (3MB) | math_numerical |
| GSM8K test | 1,319 | `math/gsm8k_test.jsonl` (732KB) | math_numerical |
| MMLU (all 57 subjects) | 15,908 | `qa/mmlu.tar` (158MB) | exact_match |

### Code & Programming
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| HumanEval | 164 | `code/humaneval.jsonl` (209KB) | code_execution |
| MBPP | 974 | `code/mbpp.jsonl` (550KB) | code_execution |
| WikiSQL | 80,654 | `code/wikisql.tar.bz2` (24MB) | sql_execution |

### Question Answering
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| SQuAD 2.0 train | 130K+ | `qa/squad_train.json` (40MB) | exact_match |
| SQuAD 2.0 dev | 11K+ | `qa/squad_dev.json` (4MB) | exact_match |
| TriviaQA | 95K | `qa/triviaqa.tar.gz` (603MB) | exact_match |
| HotpotQA val | 7,405 | `qa/hotpotqa_val.parquet` (26MB) | exact_match |
| CommonsenseQA train | 9,741 | `qa/commonsenseqa_train.jsonl` (3MB) | exact_match |
| WikiTableQuestions | 22K | `qa/wikitablequestions.zip` (27MB) | exact_match |
| COPA | 1,000 | `qa/copa.tgz` | exact_match |

### Fact Verification
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| FEVER train | 145,449 | `qa/fever_train.jsonl` (31MB) | exact_match |
| TabFact | 118K | `qa/tabfact_tables.zip` (294KB) | exact_match |

### NLI & Classification
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| SNLI | 570K | `classification/snli.zip` (90MB) | exact_match |
| MultiNLI | 433K | `classification/multinli.zip` (216MB) | exact_match |
| ANLI (all rounds) | 170K | `classification/anli.zip` (17MB) | exact_match |
| SST-2 | 70K | `classification/sst2.zip` (7MB) | exact_match |
| AG News test | 7,600 | `classification/agnews_test.csv` (1MB) | exact_match |

### Commonsense & Science
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| HellaSwag val | 10,042 | `qa/hellaswag_val.jsonl` (11MB) | exact_match |
| PIQA val | 1,838 | `qa/piqa_valid.jsonl` (484KB) | exact_match |
| Winogrande | 44K | `qa/winogrande.zip` (3MB) | exact_match |
| ARC (Challenge+Easy) | 7,787 | `science/arc.zip` (649MB) | exact_match |
| OpenBookQA | 5,957 | `science/openbookqa.zip` (1MB) | exact_match |
| SciQ | 13,679 | `science/sciq.zip` (2MB) | exact_match |

### Medical
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| MedQA (USMLE) train | 10K+ | `medical/medqa_train.jsonl` (15MB) | exact_match |

### Logic & Games
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| Lichess puzzles | 3M+ | `logic/lichess_puzzles.csv.zst` (279MB) | rule (chess engine) |
| Sudoku puzzles | 1M+ | `logic/sudoku_data.zip` (69MB) | sudoku |

### Instruction Following
| Dataset | Problems | File | Verifier |
|---------|----------|------|----------|
| IFEval | 541 | `instruction/ifeval.jsonl` (202KB) | instruction_following |

---

## Datasets Needing HuggingFace Auth or Manual Download (🔗)

Run these commands after `huggingface-cli login`:

```bash
# Math
huggingface-cli download hendrycks/competition_math --repo-type dataset -d raw/datasets/math/math_hendrycks
huggingface-cli download EleutherAI/hendrycks_math --repo-type dataset -d raw/datasets/math/math_eleuther

# Code
huggingface-cli download codeparrot/apps --repo-type dataset -d raw/datasets/code/apps
huggingface-cli download deepmind/code_contests --repo-type dataset -d raw/datasets/code/codecontests
huggingface-cli download princeton-nlp/SWE-bench --repo-type dataset -d raw/datasets/code/swebench
huggingface-cli download HKUNLP/DS-1000 --repo-type dataset -d raw/datasets/code/ds1000

# QA
huggingface-cli download google-research-datasets/natural_questions --repo-type dataset -d raw/datasets/qa/nq

# SQL
huggingface-cli download xlangai/spider --repo-type dataset -d raw/datasets/code/spider
huggingface-cli download BIRD-Bench/BIRD --repo-type dataset -d raw/datasets/code/bird

# Medical
huggingface-cli download bigbio/med_qa --repo-type dataset -d raw/datasets/medical/medqa_full
huggingface-cli download openlifescienceai/medmcqa --repo-type dataset -d raw/datasets/medical/medmcqa

# Legal
huggingface-cli download nguha/legalbench --repo-type dataset -d raw/datasets/legal/legalbench

# Formal Proofs (just clone)
git clone --depth 1 https://github.com/openai/miniF2F raw/datasets/math/minif2f
git clone --depth 1 https://github.com/zhangir-azerbayev/ProofNet raw/datasets/math/proofnet
```

## Procedurally Generated (🔧 — Unlimited, No Download)

These verifiers generate training data at runtime:

| Verifier | Generation Method | Est. Rate |
|----------|-------------------|-----------|
| unit_conversion | Random value + random unit pair from conversion table | 100K/sec |
| date_time | Random dates + random operations (add days, day-of-week) | 100K/sec |
| chemical_equation | Random formulas, compute balanced coefficients | 10K/sec |
| sudoku | Constraint-based generation with uniqueness guarantee | 1K/sec |
| regex_synthesis | Sample patterns, generate positive/negative examples | 50K/sec |
| instruction_following | Sample constraint templates, compose randomly | 50K/sec |
| json_schema | Random schema generation, random valid/invalid instances | 50K/sec |
| graph_properties | Random graph generation (Erdos-Renyi), compute properties | 10K/sec |

---

## Coverage Matrix: Verifier × Dataset

| Verifier | Downloaded Datasets | Total Problems | Procedural? |
|----------|-------------------|----------------|-------------|
| math_numerical | GSM8K (8.8K) | 8,792 | No |
| math_equivalence | GSM8K (8.8K), MMLU-math | 10K+ | No |
| exact_match | SQuAD, TriviaQA, FEVER, SNLI, MultiNLI, ANLI, CommonsenseQA, MMLU, HellaSwag, PIQA, Winogrande, ARC, MedQA, etc. | 1.5M+ | No |
| instruction_following | IFEval (541) | 541 | Yes (unlimited) |
| json_schema | — | 0 (generate) | Yes (unlimited) |
| code_execution | HumanEval (164), MBPP (974) | 1,138 | No |
| sudoku | Sudoku data (1M+) | 1M+ | Yes (unlimited) |
| chemical_equation | — | 0 (generate) | Yes (unlimited) |
| regex_synthesis | — | 0 (generate) | Yes (unlimited) |
| unit_conversion | — | 0 (generate) | Yes (unlimited) |
| date_time | — | 0 (generate) | Yes (unlimited) |
| sql_execution | WikiSQL (80K) | 80,654 | No |
| graph_properties | — | 0 (generate) | Yes (unlimited) |

**Total downloaded problems: ~1.7M+**
**Total with procedural generation: Unlimited**
