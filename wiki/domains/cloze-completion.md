---
domain: cloze-completion
category: language
verified: true
verification_type: exact
scalability: very-high
maturity: high
tags: [cloze, fill-in-the-blank, language-modeling, masked]
---

# Cloze Completion

## Overview

Cloze completion (fill-in-the-blank) tasks require the model to predict a missing word or phrase in a sentence or passage. The original word is removed and the model must recover it from context. This is one of the most fundamental language tasks, directly testing language modeling and contextual understanding.

The RLVR properties of cloze completion are excellent: verification is exact match against the original word, and data generation is trivial — any text corpus can produce unlimited cloze instances by masking random words. This makes it one of the most scalable RLVR domains.

## Verification Mechanism

**Verification type: Exact**

```python
def verify(prediction: str, gold_word: str) -> int:
    return 1 if normalize(prediction) == normalize(gold_word) else 0
```

Normalization: lowercase, strip whitespace, lemmatize optionally.

**Important consideration**: A blank may have multiple valid completions. "The ___ sat on the mat" could be "cat," "dog," "man," etc. Two approaches:

1. **Single gold answer**: Only the original word is accepted. This is harsh but unambiguous. Works best when context is highly constraining.
2. **Multiple valid answers**: Accept any word from a curated set of valid alternatives. Requires precomputing valid alternatives (e.g., using a language model to generate plausible completions, then filtering).

For RLVR, the single-gold-answer approach is simpler and scales better. To mitigate false negatives, choose blanking positions where the context strongly constrains the answer (named entities, specific numbers, technical terms).

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| CBT (Children's Book Test) | 688K | Children's books | Named entity, common noun, verb, preposition cloze |
| LAMBADA | 10K | Book passages | Last-word prediction requiring long-range context |
| Story Cloze Test | 3.7K | Short stories | Choose ending for a story (multiple-choice cloze) |
| CLOTH | 99K blanks | English exams | Cloze from language tests with distractors |
| SciTail Cloze | Generated | Scientific text | Science-domain cloze |
| Wikitext Cloze | Generated | Wikipedia | Generate from any Wikipedia passage |
| Any text corpus | Unlimited | Any | Mask a word, use context as input |

**Scalability**: This is arguably the most scalable RLVR domain. Every sentence in every text corpus can generate one or more cloze instances. Common Crawl alone provides billions of sentences. The generation pipeline:
1. Take any clean sentence
2. Select a word (or phrase) to mask
3. Replace with a blank
4. Gold answer = the original word

No annotation needed. Unlimited data.

## Task Format

**Input**: A sentence or passage with a blank (marked by `___` or `[BLANK]`).

**Output**: The word or phrase that fills the blank.

**Prompt template example**:
```
Fill in the blank with the most appropriate word.

The capital of France is ___.

Answer:
```

For longer context:
```
Read the passage and fill in the blank.

{passage_with_blank}

The missing word is:
```

For multiple-choice:
```
Choose the word that best completes the sentence.

The researcher published her findings in a prestigious ___.

(A) restaurant  (B) journal  (C) highway  (D) blanket

Answer:
```

## Difficulty Curriculum

1. **Level 1 — High-frequency function words**: Blanking common prepositions, articles, and conjunctions. "She went ___ the store." (to)
2. **Level 2 — Content words with strong local context**: Blanking nouns/verbs highly constrained by adjacent words. "The cat ___ on the mat." (sat)
3. **Level 3 — Named entities**: Blanking proper nouns. "The Eiffel Tower is in ___." (Paris). Tests factual knowledge.
4. **Level 4 — Long-range dependency**: LAMBADA-style where the answer requires understanding 5+ sentences of prior context.
5. **Level 5 — Technical and domain-specific terms**: "The process of photosynthesis converts ___ into glucose." Requires domain knowledge.
6. **Level 6 — Multi-word blanks**: Blanking phrases rather than single words. "She was ___ about the outcome." (cautiously optimistic)
7. **Level 7 — Pragmatic and contextual inference**: The blank requires understanding tone, pragmatics, or implicit context. "After the team lost their tenth game, the coach's expression was ___." Requires inference, not just pattern matching.

## Limitations & Risks

- **Multiple valid completions**: The fundamental limitation. Many blanks have multiple correct answers, but only the original word is in the gold standard. This creates false negative rewards, especially for common-context blanks.
- **Position bias**: Models may learn that certain positions (sentence-final, after "is") have predictable patterns, leading to overfitting on positional cues rather than genuine comprehension.
- **Frequency bias**: Models prefer high-frequency words, which may not be the original word. A model that always predicts common words may score well on average but fail on specific instances.
- **Trivial instances**: Many cloze instances are too easy (heavily constrained by local context) and provide no training signal for strong models.
- **Lack of reasoning depth**: Unlike multi-hop QA or semantic parsing, most cloze tasks test pattern completion rather than reasoning. The RL reward does not incentivize deep understanding.

## Connections

- **[question-answering-extractive](question-answering-extractive.md)**: Cloze is a simplified form of QA where the question is the context with a blank.
- **[question-answering-closed](question-answering-closed.md)**: Factual cloze (blanking named entities) tests the same knowledge recall as closed-form QA.
- **[spelling-grammar](spelling-grammar.md)**: Filling in blanks uses the same contextual language modeling skills needed for error correction.
- **[data-entry-correction](data-entry-correction.md)**: Correcting garbled words is like filling in blanks where the blank contains partial information.
- **[entity-linking](entity-linking.md)**: Entity prediction in cloze (which entity fills the blank?) relates to entity disambiguation.
- **[anagram-wordplay](anagram-wordplay.md)**: Both test word-level knowledge and vocabulary skills, though through different mechanisms.
