---
domain: spelling-grammar
category: language
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [gec, spelling, grammar, error-correction, writing]
---

# Spelling and Grammar Correction

## Overview

Spelling and grammar correction (grammatical error correction, GEC) tasks require the model to identify and fix errors in text: misspellings, subject-verb disagreement, wrong tense, missing articles, incorrect word order, and other grammatical mistakes. Given an erroneous sentence, the model produces the corrected version.

Verification compares the model's corrected output against gold-standard corrections. This domain has well-established benchmarks from shared tasks and learner corpora, and synthetic training data can be generated at massive scale by injecting errors into clean text.

## Verification Mechanism

**Verification type: Exact (with established evaluation tools)**

GEC evaluation uses specialized metrics that go beyond simple string comparison:

### ERRANT (ERRor ANnotation Toolkit)
The standard GEC evaluation tool:
1. Compute the edit operations between the source (erroneous) sentence and the model's output
2. Compute the edit operations between the source sentence and the gold correction
3. Compare edits: precision, recall, F0.5 (precision-weighted, since false corrections are worse than missed corrections)

```python
def verify(source: str, prediction: str, gold: str) -> float:
    pred_edits = compute_edits(source, prediction)
    gold_edits = compute_edits(source, gold)
    tp = len(pred_edits & gold_edits)
    precision = tp / len(pred_edits) if pred_edits else 1.0
    recall = tp / len(gold_edits) if gold_edits else 1.0
    f05 = (1.25 * precision * recall) / (0.25 * precision + recall) if (precision + recall) > 0 else 0.0
    return f05
```

### GLEU (Generalized Language Evaluation Understanding)
A variant of BLEU adapted for GEC that accounts for both corrections and preserved correct text.

### Simple exact match
For binary RLVR reward, the simplest approach:
```python
def verify_simple(prediction: str, gold: str) -> int:
    return 1 if normalize(prediction) == normalize(gold) else 0
```

This is harsh (requires perfect correction) but fully exact.

### M2 Scorer
The MaxMatch (M2) scorer computes F-score based on edit-level matching between system output and gold annotations. It is the traditional GEC evaluation metric.

All verification methods are fully programmatic.

## Dataset Sources

| Dataset | Size | Source | Error Types | Notes |
|---------|------|--------|-------------|-------|
| BEA-2019 Shared Task | 34K sentences | Write & Improve + LOCNESS | All grammar/spelling | Primary GEC benchmark |
| CoNLL-2014 | 1.4K sentences | NUCLE | All grammar | Shared task test set |
| NUCLE (NUS Corpus of Learner English) | 57K sentences | Student essays | All grammar | Annotated learner errors |
| Lang-8 | 1M sentences | Lang-8 platform | All grammar | Language learner corrections from native speakers |
| JFLEG | 1.5K sentences | GUG corpus | Fluency + grammar | Fluency-focused evaluation |
| FCE (First Certificate in English) | 33K sentences | Cambridge exams | All grammar | Exam-based learner corpus |
| W&I+LOCNESS | 34K sentences | Write & Improve + LOCNESS | All grammar | Combined learner corpus |
| PIE (Parallel Italian-English) | 9.4K sentences | Italian learner English | All grammar | Cross-lingual errors |
| CWEB | 14K sentences | Web text | Spelling | Web-crawled text with corrections |
| GitHub Typo Corpus | 200K+ edits | GitHub commits | Spelling/typos | Naturally occurring corrections |

**Scalability**: Massive through synthetic generation. Take any clean text and inject errors:
- Randomly introduce spelling errors (character substitutions, insertions, deletions)
- Inject grammatical errors (wrong tense, wrong article, subject-verb disagreement)
- Drop or swap words
- Introduce L1-specific error patterns for language learner simulation

The GitHub Typo Corpus demonstrates that naturally occurring corrections can be mined at scale from edit histories.

## Task Format

**Input**: A sentence or passage containing spelling and/or grammar errors.

**Output**: The corrected sentence or passage.

**Prompt template example**:
```
Correct all spelling and grammar errors in the following sentence. Output only the corrected sentence.

Input: {erroneous_sentence}

Corrected:
```

For minimal-edit correction:
```
Fix the errors in the following text. Make only the necessary corrections, preserving the original meaning and style.

Text: {erroneous_text}

Corrected text:
```

## Difficulty Curriculum

1. **Level 1 — Obvious misspellings**: "Teh cat sat on teh mat." Single-character errors in common words.
2. **Level 2 — Common grammar errors**: Subject-verb agreement, article usage, basic tense errors. "He go to school yesterday."
3. **Level 3 — Multiple errors per sentence**: Several interacting errors. Requires planning corrections that don't conflict.
4. **Level 4 — Subtle grammar**: Comma splices, dangling modifiers, parallelism errors. Requires deeper grammatical understanding.
5. **Level 5 — Style and fluency**: Awkward phrasing that is technically grammatical but unnatural. JFLEG-style fluency improvement.
6. **Level 6 — Domain-specific text**: Technical, medical, or legal text where corrections require domain knowledge.
7. **Level 7 — L1-transfer errors**: Errors caused by native language interference (e.g., article usage for speakers of languages without articles). Requires understanding error patterns.

## Limitations & Risks

- **Multiple valid corrections**: A sentence like "He goed to the store" could be corrected to "He went to the store" or "He goes to the store" — the correct tense depends on context. Gold annotations capture one correction, penalizing valid alternatives.
- **Overcorrection vs. undercorrection tradeoff**: The F0.5 metric (precision-weighted) penalizes false corrections more than missed corrections. But RLVR with exact match may encourage overly conservative behavior (making fewer corrections to avoid errors).
- **Context dependence**: Some errors can only be corrected with broader context (paragraph or document level). Sentence-level correction datasets miss these.
- **Fluency vs. grammar**: The boundary between grammatical correction and style improvement is fuzzy. What counts as an "error" vs. a "style choice"?
- **Evaluation sensitivity**: Minor formatting differences (spacing, punctuation style) can cause false negatives in exact match evaluation.
- **Cultural and dialect variation**: "Correct" grammar depends on dialect and register. American vs. British English, formal vs. informal writing. Datasets often encode one standard, penalizing valid alternatives.

## Connections

- **[data-entry-correction](data-entry-correction.md)**: OCR correction and spelling correction share techniques and overlap in scope.
- **[cloze-completion](cloze-completion.md)**: Filling in blanks tests similar language modeling skills used for error detection.
- **[text-classification](text-classification.md)**: Error detection (is this sentence grammatical?) is a classification task.
- **[translation-reference](translation-reference.md)**: Post-editing machine translation output is closely related to grammar correction.
- **[text-to-code-nl](text-to-code-nl.md)**: Code repair (fixing bugs) is analogous to grammar correction in natural language.
- **[multilingual-tasks](multilingual-tasks.md)**: GEC is needed for all languages, and cross-lingual error patterns inform correction strategies.
