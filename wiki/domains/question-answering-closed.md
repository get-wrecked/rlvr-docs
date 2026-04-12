---
domain: question-answering-closed
category: knowledge
verified: true
verification_type: exact
scalability: very-high
maturity: high
tags: [qa, trivia, closed-form, exact-answer, knowledge-recall]
---

# Closed-Form Question Answering

## Overview

Closed-form QA tasks require the model to produce a short, exact answer to a factual question — a date, number, entity name, or other discrete value. Unlike extractive QA, no passage is provided: the model must answer from its own knowledge or from retrieved information. Verification is exact match against known correct answers.

This domain has enormous scale thanks to decades of trivia, quiz bowl, and knowledge base QA datasets. It is one of the cleanest RLVR environments: the answer is unambiguous, verification is trivial, and data is abundant.

## Verification Mechanism

**Verification type: Exact**

The verification function normalizes both the predicted answer and the gold answer(s), then checks for an exact string match.

**Normalization pipeline**:
1. Lowercase
2. Strip leading/trailing whitespace
3. Remove articles (a, an, the)
4. Remove punctuation
5. Normalize Unicode (NFD)
6. For numerical answers: parse to a canonical numeric form
7. For date answers: parse to YYYY-MM-DD format

Most datasets provide multiple acceptable answer aliases (e.g., "USA", "United States", "United States of America", "U.S.A."). The prediction matches if it equals any alias.

```python
def verify(prediction: str, gold_answers: list[str]) -> int:
    pred_normalized = normalize(prediction)
    for gold in gold_answers:
        if pred_normalized == normalize(gold):
            return 1
    return 0
```

This is fully deterministic, binary, and fast.

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| TriviaQA | 95K questions | Trivia websites | Entity and fact-based trivia questions |
| Natural Questions (short answer) | 307K questions | Google search | Real user queries with short answers |
| WebQuestions | 6.6K questions | Freebase | Entity answers from knowledge graphs |
| SimpleQuestions | 108K questions | Freebase | Single-fact questions |
| Quiz Bowl (QANTA) | 100K+ questions | Academic quiz competitions | Progressively revealed clues |
| Jeopardy! | 200K+ questions | TV show archives | Broad-domain trivia |
| ARC (AI2 Reasoning Challenge) | 7.7K questions | Science exams | Multiple-choice science questions |
| MMLU | 15K questions | Professional exams | 57 subjects, multiple choice |
| CommonsenseQA | 12K questions | ConceptNet | Commonsense reasoning |
| CuratedTREC | 2.2K questions | TREC QA track | Manually curated factoid questions |

**Scalability**: Trivia and quiz data is available at massive scale. Knowledge bases (Wikidata, Freebase) contain billions of facts that can be converted to QA pairs via templates. Quiz bowl competitions generate thousands of new questions annually. MMLU-style exam data grows with every published exam.

## Task Format

**Input**: A factual question.

**Output**: A short, exact answer (entity name, number, date, etc.).

**Prompt template example**:
```
Answer the following question with a short, exact answer. Give only the answer, no explanation.

Question: {question}

Answer:
```

For multiple-choice variants:
```
Answer the following question. Choose from the given options.

Question: {question}
Options: (A) {option_a} (B) {option_b} (C) {option_c} (D) {option_d}

Answer:
```

## Difficulty Curriculum

1. **Level 1 — High-frequency facts**: "What is the capital of France?" Answers are common knowledge found frequently in training data.
2. **Level 2 — Specific dates and numbers**: "In what year was the Eiffel Tower completed?" Requires precise factual recall.
3. **Level 3 — Multi-attribute questions**: "Who was the 16th president of the United States?" Requires combining attribute constraints.
4. **Level 4 — Rare entities**: "What is the smallest country in Africa by area?" Answers involving less commonly discussed entities.
5. **Level 5 — Domain-specific expertise**: MMLU-style questions requiring professional knowledge (medicine, law, engineering).
6. **Level 6 — Temporal questions**: "Who was the UK Prime Minister in 1952?" Requires temporally scoped knowledge.
7. **Level 7 — Quiz bowl progressive clues**: Paragraph-length clues revealed incrementally, rewarding early recognition of the answer.

## Limitations & Risks

- **Answer aliasing**: Despite normalization, some correct answers may not match any gold alias. "JFK" vs "John Fitzgerald Kennedy" vs "President Kennedy." Comprehensive alias lists are essential but never complete.
- **Knowledge cutoff**: For questions about recent events or changing facts (current population, current president), the gold answer may become outdated.
- **Ambiguous questions**: "Who played Batman?" has multiple correct answers depending on unstated context (which film/show). Datasets try to avoid these, but some slip through.
- **Memorization vs. reasoning**: Many closed-form QA datasets can be "solved" by memorizing fact associations rather than developing genuine reasoning. The RL reward does not distinguish these.
- **Cultural and geographic bias**: Most large trivia datasets are English-centric and biased toward Western knowledge.

## Connections

- **[question-answering-extractive](question-answering-extractive.md)**: Extractive QA provides a passage; closed-form does not. Same verification mechanism.
- **[reading-comprehension](reading-comprehension.md)**: Multi-hop RC bridges extractive and closed-form QA with complex reasoning.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: KG facts are the structured backbone of closed-form QA.
- **[fact-verification](fact-verification.md)**: A factual question can be rephrased as a claim to verify.
- **[temporal-reasoning](temporal-reasoning.md)**: Temporal QA is a specialized form of closed-form QA.
- **[cloze-completion](cloze-completion.md)**: Cloze tasks test similar knowledge recall in a fill-in-the-blank format.
- **[multilingual-tasks](multilingual-tasks.md)**: Cross-lingual QA extends closed-form QA to multiple languages.
