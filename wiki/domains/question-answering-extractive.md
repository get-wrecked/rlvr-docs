---
domain: question-answering-extractive
category: knowledge
verified: true
verification_type: exact
scalability: very-high
maturity: very-high
tags: [qa, reading-comprehension, span-extraction, squad]
---

# Extractive Question Answering

## Overview

Extractive QA is one of the oldest and most well-established RLVR domains. The model is given a passage and a question, and must identify a contiguous span in the passage that answers the question. Verification is straightforward: compare the predicted span to the gold answer span using exact match (EM) or token-level F1.

This domain has massive, high-quality datasets, well-understood evaluation, and near-perfect verification. It is a canonical RLVR environment.

## Verification Mechanism

**Verification type: Exact (with standard normalization)**

Two standard metrics serve as verification functions:

1. **Exact Match (EM)**: After normalization (lowercasing, removing articles/punctuation, stripping whitespace), check if the predicted answer string equals any of the gold answer strings. Binary: 1 if match, 0 otherwise.

2. **Token-level F1**: Compute precision and recall at the token level between the predicted answer and the best-matching gold answer. This provides a softer reward signal.

For RLVR, EM is the cleaner reward (fully binary, no approximation). F1 can serve as a shaped reward for curriculum training.

**Normalization details** (standard SQuAD evaluation):
- Lowercase all text
- Remove articles (a, an, the)
- Remove punctuation
- Collapse whitespace
- Compare against all acceptable gold answers (most datasets provide multiple valid answer spans)

This verification is fully programmatic, deterministic, and fast.

## Dataset Sources

| Dataset | Size | Source | Answer Type | Notes |
|---------|------|--------|-------------|-------|
| SQuAD 1.1 | 100K questions | Wikipedia paragraphs | Extractive span | The foundational extractive QA dataset |
| SQuAD 2.0 | 150K questions | Wikipedia paragraphs | Extractive span or unanswerable | Adds 50K unanswerable questions |
| Natural Questions | 307K questions | Google queries + Wikipedia | Short answer span or long answer | Real user questions |
| TriviaQA | 95K questions | Trivia + Wikipedia/Web | Extractive span | Questions written independently of evidence |
| NewsQA | 120K questions | CNN articles | Extractive span | News domain |
| SearchQA | 140K questions | Jeopardy + web snippets | Extractive span | Retrieved web context |
| DuoRC | 186K questions | Movie plots | Extractive span | Paraphrased passages |
| MRQA | 10+ datasets | Various | Extractive span | Unified format across datasets |

**Scalability**: Extractive QA data can be generated at massive scale. Any factual passage can serve as context; questions can be generated via cloze deletion, template filling, or backtranslation. The SQuAD/NQ paradigm extends to any domain with text.

## Task Format

**Input**: A context passage and a question.

**Output**: A text span extracted from the passage (or "unanswerable" for SQuAD 2.0-style tasks).

**Prompt template example**:
```
Answer the question by extracting the relevant span from the passage. Answer with the exact text from the passage.

Passage: {context}

Question: {question}

Answer:
```

## Difficulty Curriculum

1. **Level 1 — Lexical overlap**: The question words appear almost verbatim near the answer in the passage. Simple lookup.
2. **Level 2 — Paraphrase**: The question paraphrases the relevant sentence. Requires synonym recognition.
3. **Level 3 — Multi-sentence reasoning**: The answer requires combining information from multiple sentences in the passage.
4. **Level 4 — Unanswerable questions**: The model must recognize when the passage does not contain the answer (SQuAD 2.0).
5. **Level 5 — Long documents**: Context is a full Wikipedia article (Natural Questions). The model must locate relevant content in thousands of tokens.
6. **Level 6 — Distractor passages**: Multiple passages are provided, some irrelevant. The model must identify which passage contains the answer.
7. **Level 7 — Adversarial examples**: Adversarially constructed passages with distractors designed to fool models (Adversarial SQuAD).

## Limitations & Risks

- **Answer format constraint**: Forcing extractive answers means the gold span must appear verbatim in the passage. This excludes questions requiring synthesis, calculation, or abstraction.
- **Multiple valid spans**: Sometimes several spans are equally valid answers, but only a few are annotated as gold. This creates false negatives in the reward signal.
- **Dataset saturation**: Models have nearly saturated SQuAD (superhuman performance). The reward signal becomes uninformative at the top of the performance range.
- **Shortcut learning**: Models may exploit dataset artifacts (e.g., answer-type patterns, position biases) rather than learning genuine comprehension.
- **Passage dependence**: The model's ability is bounded by the quality and relevance of the provided passage. This is a test of comprehension, not knowledge.

## Connections

- **[question-answering-closed](question-answering-closed.md)**: Closed-form QA removes the passage constraint, testing knowledge recall.
- **[reading-comprehension](reading-comprehension.md)**: Multi-hop reading comprehension is a harder version requiring cross-passage reasoning.
- **[fact-verification](fact-verification.md)**: QA and fact verification are reformulations of each other (a claim is a question with a yes/no answer).
- **[table-understanding](table-understanding.md)**: Table QA is extractive QA over structured data.
- **[information-extraction](information-extraction.md)**: Both involve identifying relevant spans in text.
- **[cloze-completion](cloze-completion.md)**: Cloze tasks are a simple form of extractive QA where the question is the sentence with a blank.
