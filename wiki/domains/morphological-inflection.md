---
domain: Morphological Inflection
category: nlp-linguistics
verification_type: exact_match
dataset_scale: 400+ languages, millions of forms (UniMorph)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Morphological Inflection

## Overview

Morphological inflection generates the correct surface form of a word given its lemma and a set of morphological features. For example: lemma "run" + features {PAST, 3SG} -> "ran"; lemma "Haus" (German) + features {PL, DAT} -> "Hausern". This task tests whether a model has internalized the morphological rules (and exceptions) of a language.

RLVR applicability is excellent: verification is exact string match against the gold inflected form. The task is unambiguous in the vast majority of cases (a lemma + feature bundle has exactly one correct surface form in standard language), and datasets exist for 400+ languages.

## Verification Mechanism

```python
import unicodedata

def normalize_form(s: str) -> str:
    """Unicode NFC normalization for consistent comparison."""
    return unicodedata.normalize("NFC", s.strip().lower())

def verify_inflection(predicted: str, gold: str) -> dict:
    """Exact match verification for morphological inflection."""
    pred_norm = normalize_form(predicted)
    gold_norm = normalize_form(gold)

    exact = pred_norm == gold_norm

    # Character-level edit distance for partial credit
    edit_dist = levenshtein(pred_norm, gold_norm)
    max_len = max(len(pred_norm), len(gold_norm), 1)
    char_accuracy = 1.0 - edit_dist / max_len

    return {
        "exact_match": 1.0 if exact else 0.0,
        "character_accuracy": char_accuracy,
        "edit_distance": edit_dist,
        "reward": 1.0 if exact else char_accuracy * 0.5  # partial credit
    }

def levenshtein(s1: str, s2: str) -> int:
    if len(s1) < len(s2):
        return levenshtein(s2, s1)
    if len(s2) == 0:
        return len(s1)
    prev_row = range(len(s2) + 1)
    for i, c1 in enumerate(s1):
        curr_row = [i + 1]
        for j, c2 in enumerate(s2):
            insertions = prev_row[j + 1] + 1
            deletions = curr_row[j] + 1
            substitutions = prev_row[j] + (c1 != c2)
            curr_row.append(min(insertions, deletions, substitutions))
        prev_row = curr_row
    return prev_row[-1]
```

## Dataset Sources

- **UniMorph**: Morphological paradigms for 400+ languages with millions of inflected forms. UniMorph 4.0 covers ~170 languages with high quality. Each entry: (lemma, features, inflected form). Freely available.
- **SIGMORPHON Shared Tasks (2016-2023)**: Annual competitions on morphological inflection/reinflection. Standardized train/dev/test splits for 50-100 languages per year. Low-resource settings (100 training examples) to high-resource (10K+).
- **Wiktionary extractions**: Paradigm tables scraped from Wiktionary for hundreds of languages. Noisy but massive coverage.
- **CELEX**: Morphological database for English, Dutch, German. ~160K lemmas with full paradigms.
- **MorphyNet**: Derivational morphology resource for 15 languages.

## Task Format

- **Input**: Lemma and morphological feature bundle.
```
Lemma: "swim"
Features: V;PST;PTCP
(verb, past tense, participle)

Generate the inflected form.
```
- **Output**: The inflected word.
```
swum
```

Another example:
```
Lemma: "hermoso" (Spanish)
Features: ADJ;FEM;PL

Output: "hermosas"
```

## Difficulty Curriculum

- Level 1: Regular inflection in English (walk -> walked, cat -> cats)
- Level 2: Regular inflection in moderately inflected languages (Spanish, French verb conjugation)
- Level 3: Irregular but common forms (English: go -> went, be -> was)
- Level 4: Morphophonological rules (German umlaut: Haus -> Hauser, Finnish vowel harmony)
- Level 5: Rich case systems (Turkish, Finnish: 15+ cases with agglutinative morphology)
- Level 6: Nonconcatenative morphology (Arabic root-pattern: k-t-b -> kutub, kataba)
- Level 7: Low-resource languages with limited training data (100 examples)
- Level 8: Polysynthetic languages (Inuktitut, Mohawk: single words encoding entire sentences)
- Level 9: Rare/archaic forms, dialectal variation, and suppletion across paradigms

## Limitations & Risks

- **Syncretism**: Multiple feature bundles can map to the same surface form (English "sheep" is both SG and PL). This means multiple correct inputs map to one output, which is fine, but the reverse (one input with multiple valid outputs) is rare and would cause verification issues.
- **Spelling variants**: Some languages have legitimate spelling variants (e.g., "grey" vs. "gray"). UniMorph typically standardizes to one, but this could penalize valid alternatives.
- **Script/encoding issues**: Unicode normalization is essential. Languages with diacritics, ligatures, or non-Latin scripts can have multiple valid byte representations of the same character.
- **Contextual allomorphy**: A few languages have inflectional choices that depend on phonological or syntactic context beyond the lemma+features. These cases are rare but exist.
- **Data quality**: UniMorph entries are partially auto-generated from Wiktionary. Some low-resource languages have errors. SIGMORPHON shared task data is more curated.
- **Memorization concern**: High-frequency forms may be memorized from pretraining data rather than learned through morphological rules. Testing on held-out lemmas (not held-out forms of seen lemmas) is important.

## Connections

- [[text-normalization]] — Text normalization involves mapping non-standard forms to standard ones, a related string transduction task
- [[word-sense-disambiguation]] — Both require language-specific lexical knowledge
- [[machine-translation]] — Generating correct morphological forms is a major challenge in MT, especially for morphologically rich target languages
- [[constrained-writing]] — Inflection can be seen as a heavily constrained string generation task
