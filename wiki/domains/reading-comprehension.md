---
domain: reading-comprehension
category: knowledge
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [multi-hop, reasoning, hotpotqa, comprehension]
---

# Multi-Hop Reading Comprehension

## Overview

Multi-hop reading comprehension requires the model to answer questions that cannot be resolved by a single passage or a single reasoning step. The model must retrieve and combine information from multiple paragraphs, performing chains of reasoning (typically 2-4 hops) to arrive at the answer.

This domain pushes beyond simple extractive QA into genuine reasoning. Verification remains exact (answer exact match), but the underlying task is substantially harder. It is well-suited for RLVR because it combines a clean reward signal with a difficult reasoning challenge.

## Verification Mechanism

**Verification type: Exact**

The primary verification is exact match (EM) and F1 on the predicted answer, identical to extractive QA:

```python
def verify(prediction: str, gold_answers: list[str]) -> int:
    pred_norm = normalize(prediction)
    for gold in gold_answers:
        if pred_norm == normalize(gold):
            return 1
    return 0
```

**Supporting evidence verification** (for datasets that annotate reasoning chains):

Some datasets (HotpotQA, MuSiQue) also annotate the supporting sentences needed for the answer. This allows a richer reward:

```python
def verify_with_evidence(
    pred_answer: str, pred_evidence: list[str],
    gold_answer: str, gold_evidence: list[str]
) -> float:
    answer_correct = normalize(pred_answer) == normalize(gold_answer)
    evidence_f1 = compute_f1(pred_evidence, gold_evidence)
    return 0.5 * int(answer_correct) + 0.5 * evidence_f1
```

This shaped reward encourages correct reasoning chains, not just lucky guesses. However, the pure answer-based EM reward is simpler and still fully exact.

## Dataset Sources

| Dataset | Size | Hops | Source | Notes |
|---------|------|------|--------|-------|
| HotpotQA | 113K questions | 2 | Wikipedia | Bridge and comparison questions; supporting fact annotation |
| MuSiQue | 25K questions | 2-4 | Wikipedia | Compositional, harder than HotpotQA |
| 2WikiMultiHopQA | 192K questions | 2 | Wikipedia | Large-scale, template-generated |
| StrategyQA | 2.8K questions | Multi-step | Wikipedia | Yes/no questions requiring implicit decomposition |
| MultiRC | 9.9K questions | Multi-sentence | Various | Multiple-choice with multiple correct answers |
| DROP | 96K questions | Multi-step numerical | Wikipedia paragraphs | Discrete reasoning (math, counting, sorting) |
| IIRC | 13K questions | 2 | Wikipedia | Questions requiring retrieving additional context |
| Musique-Ans | 25K questions | 2-4 | Wikipedia | Answerable subset with decomposition annotations |

**Scalability**: Wikipedia provides an enormous pool for multi-hop question generation. Wikidata's relational structure naturally supports multi-hop chains (A -> relation1 -> B -> relation2 -> C). Template-based generation (as in 2WikiMultiHopQA) scales to hundreds of thousands of questions. Compositional question generation from knowledge graphs can produce millions of examples.

## Task Format

**Input**: A question and a set of passages (or a full-text context with multiple paragraphs).

**Output**: A short answer (entity, yes/no, number).

**Prompt template example**:
```
Answer the following question using the provided passages. The answer may require combining information from multiple passages.

Passage 1: {passage_1}
Passage 2: {passage_2}
...

Question: {question}

Answer:
```

For chain-of-thought reasoning:
```
Answer the following question step by step, then give the final answer.

Passages: {passages}

Question: {question}

Reasoning:
Step 1:
Step 2:
...
Final Answer:
```

## Difficulty Curriculum

1. **Level 1 — Two-hop bridge questions**: "Where was the director of Titanic born?" Hop 1: director = James Cameron. Hop 2: born in Kapuskasing, Ontario.
2. **Level 2 — Two-hop comparison questions**: "Are the directors of Titanic and Avatar the same person?" Requires looking up both directors and comparing.
3. **Level 3 — Three-hop chains**: Questions requiring three sequential reasoning steps.
4. **Level 4 — Numerical multi-hop**: DROP-style questions requiring discrete reasoning (counting, arithmetic, sorting) over multiple facts.
5. **Level 5 — Four-hop and compositional**: MuSiQue-style questions with 4 hops. Requires sustained reasoning without losing track.
6. **Level 6 — Implicit decomposition**: StrategyQA-style questions where the reasoning steps are not obvious from the question surface form. The model must figure out what to look up.
7. **Level 7 — Open-domain multi-hop**: No passages provided; the model must recall or reason about multi-hop facts from its knowledge alone.

## Limitations & Risks

- **Shortcut reasoning**: Models may find single-hop shortcuts that yield the correct answer without genuine multi-hop reasoning. For example, entity type matching or statistical co-occurrence can shortcut some bridge questions. The EM reward does not distinguish correct reasoning from lucky shortcuts.
- **Distractor passages**: Providing distractor passages alongside gold passages is important to prevent simple information extraction. Without distractors, models can often find the answer without multi-hop reasoning.
- **Answer leakage**: Some multi-hop questions can be answered from a single passage if that passage contains the final answer directly. Dataset construction must guard against this.
- **Template artifacts**: Template-generated datasets (2WikiMultiHopQA) may contain patterns that models exploit without genuine reasoning.
- **Evidence annotation cost**: Supporting evidence annotations (which enable richer rewards) are expensive to produce and not always available.
- **Question decomposition quality**: The quality of multi-hop questions depends on the decomposition — poorly constructed questions may be trivially decomposable or have ambiguous intermediate answers.

## Connections

- **[question-answering-extractive](question-answering-extractive.md)**: Multi-hop RC extends extractive QA with reasoning chains.
- **[question-answering-closed](question-answering-closed.md)**: Open-domain multi-hop QA tests knowledge recall with reasoning.
- **[fact-verification](fact-verification.md)**: Multi-hop fact verification (HoVer) is closely related.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: Multi-hop KG reasoning underlies many reading comprehension questions.
- **[semantic-parsing](semantic-parsing.md)**: Question decomposition for multi-hop QA is related to semantic parsing.
- **[temporal-reasoning](temporal-reasoning.md)**: Temporal multi-hop questions require both temporal and multi-hop reasoning.
