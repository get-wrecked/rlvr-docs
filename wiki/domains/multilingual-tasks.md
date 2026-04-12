---
domain: multilingual-tasks
category: language
verified: true
verification_type: exact
scalability: very-high
maturity: high
tags: [multilingual, cross-lingual, xnli, xtreme, low-resource]
---

# Multilingual Tasks

## Overview

Multilingual tasks extend standard NLU benchmarks (classification, QA, NER, etc.) to multiple languages. The core challenge is that the model must perform the same task across diverse languages, including low-resource languages with limited training data. Cross-lingual transfer — training in one language and testing in another — is a key capability tested here.

Verification mechanisms are the same as their monolingual counterparts (exact match, label comparison, F1), just applied across languages. This makes multilingual tasks a natural RLVR domain that inherits the verification properties of underlying tasks.

## Verification Mechanism

**Verification type: Exact (same as underlying monolingual tasks)**

The verification function is task-dependent:

- **Cross-lingual NLI (XNLI)**: Label comparison (entailment/contradiction/neutral). Exact match.
- **Cross-lingual QA (XQuAD, MLQA)**: Answer span exact match or F1, with language-appropriate normalization.
- **Cross-lingual NER (WikiAnn)**: Entity-level exact match on spans and types.
- **Cross-lingual classification**: Label comparison. Exact match.

```python
def verify_multilingual(prediction: str, gold: str, language: str) -> int:
    pred_norm = normalize(prediction, language=language)
    gold_norm = normalize(gold, language=language)
    return 1 if pred_norm == gold_norm else 0
```

**Language-specific normalization** is critical:
- Script-specific lowercasing (Turkish dotted/undotted i, German sharp s)
- Unicode normalization (NFC/NFD)
- Language-specific tokenization (CJK word segmentation, Arabic morphology)
- Diacritics handling (é vs e may or may not be equivalent depending on the language)

The verification is exact (or approximately exact, to the same degree as the monolingual version), but normalization is more complex.

## Dataset Sources

| Dataset | Task | Languages | Size | Notes |
|---------|------|-----------|------|-------|
| XNLI | NLI | 15 languages | 112K | Translates MultiNLI to 14 other languages |
| XQuAD | QA | 11 languages | 1.2K per lang | Translates SQuAD dev set |
| MLQA | QA | 7 languages | 12K | Cross-lingual QA with parallel questions |
| TyDi QA | QA | 11 languages | 200K | Natively generated QA in each language |
| WikiAnn | NER | 40+ languages | Varies | NER from Wikipedia anchor links |
| PAWS-X | Paraphrase | 7 languages | 4K per lang | Paraphrase identification |
| XCOPA | Commonsense | 11 languages | 500 per lang | Causal reasoning |
| XTREME | Multi-task | 40 languages | Multi-dataset | Benchmark combining 9 tasks |
| XTREME-R | Multi-task (revised) | 50 languages | Multi-dataset | Updated version with harder tasks |
| Tatoeba | Sentence retrieval | 122 languages | Varies | Cross-lingual sentence retrieval |
| MASSIVE | Intent + slot | 51 languages | 1M | Virtual assistant multilingual NLU |
| AmericasNLI | NLI | 10 indigenous languages | 7.5K | Low-resource indigenous languages of the Americas |

**Scalability**: Very high. Any monolingual dataset can be extended multilingually through professional translation or by constructing parallel datasets. Wikipedia and Wikidata are available in 300+ languages. The XTREME and XTREME-R benchmarks aggregate tasks across dozens of languages. Additionally, TyDi QA demonstrates that native (non-translated) multilingual data can be collected at scale.

## Task Format

**Input**: A task input in a specific language (or cross-lingual: input in language A, answer expected in language B).

**Output**: A task output in the appropriate language.

**Prompt template examples**:

Cross-lingual NLI:
```
Given the premise and hypothesis in French, determine the relationship: "entailment", "contradiction", or "neutral".

Prémisse: {premise_fr}
Hypothèse: {hypothesis_fr}

Relation:
```

Cross-lingual QA:
```
Answer the following question in Hindi based on the passage.

Passage (Hindi): {passage_hi}
Question (Hindi): {question_hi}

Answer:
```

Zero-shot cross-lingual transfer:
```
[System: Answer in the same language as the question]

{question_in_any_language}

Answer:
```

## Difficulty Curriculum

1. **Level 1 — High-resource languages, simple tasks**: NLI or classification in well-resourced languages (Spanish, French, German, Chinese). Abundant training data.
2. **Level 2 — Cross-lingual transfer**: Train on English, test on other high-resource languages. Tests zero-shot transfer.
3. **Level 3 — Medium-resource languages**: Tasks in languages like Vietnamese, Thai, or Swahili. Less training data, different scripts.
4. **Level 4 — Low-resource languages**: Tasks in languages with very limited NLP resources (indigenous languages, minority languages). AmericasNLI-style.
5. **Level 5 — Script diversity**: Tasks in languages with non-Latin scripts (Arabic, Devanagari, CJK, Cyrillic, Thai). Tests script-independent understanding.
6. **Level 6 — Cross-lingual QA**: Questions in one language, passages in another. Requires cross-lingual comprehension.
7. **Level 7 — Code-switching and mixed language**: Text that mixes multiple languages within a single passage. Requires flexible multilingual processing.

## Limitations & Risks

- **Translation artifacts**: Many multilingual datasets are translated from English, which introduces translationese and may not reflect natural language use in the target language.
- **Evaluation quality**: Machine translation of evaluation data can introduce errors. Professional translation is expensive and limited.
- **Language imbalance**: Even "multilingual" benchmarks heavily favor European languages. African, indigenous, and sign languages are underrepresented.
- **Normalization complexity**: Language-specific normalization is a significant engineering challenge. Errors in normalization produce false negative rewards.
- **Cultural bias**: Tasks designed for English-speaking contexts may not be culturally appropriate in other languages (e.g., sentiment about products not available in some countries).
- **Tokenizer dependence**: Model performance varies dramatically based on how well the tokenizer handles each language. This is a confound in RLVR.
- **Natively generated vs. translated**: Translated datasets (XNLI, XQuAD) test a different skill than natively generated ones (TyDi QA). The former tests translation robustness; the latter tests genuine multilingual understanding.

## Connections

- **[translation-reference](translation-reference.md)**: Translation is the foundation of multilingual capability, and translation data is used to create many multilingual benchmarks.
- **[text-classification](text-classification.md)**: XNLI and PAWS-X are multilingual classification tasks.
- **[question-answering-extractive](question-answering-extractive.md)**: XQuAD and MLQA are multilingual extractive QA.
- **[information-extraction](information-extraction.md)**: WikiAnn provides multilingual NER.
- **[entity-linking](entity-linking.md)**: Cross-lingual entity linking maps mentions in any language to a shared knowledge base.
- **[question-answering-closed](question-answering-closed.md)**: Multilingual closed-form QA tests knowledge across languages.
