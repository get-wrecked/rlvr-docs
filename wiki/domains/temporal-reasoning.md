---
domain: temporal-reasoning
category: knowledge
verified: true
verification_type: exact
scalability: medium
maturity: medium
tags: [temporal, time, ordering, duration, timeline]
---

# Temporal Reasoning

## Overview

Temporal reasoning tasks require the model to answer questions about time: ordering events, computing durations, reasoning about temporal relations (before, after, during, overlaps), and understanding temporal expressions. The model must parse temporal information from text and perform logical reasoning over timelines.

Verification is exact: answers are specific dates, durations, orderings, or temporal relations that can be compared against gold annotations derived from known timelines and temporal knowledge bases.

## Verification Mechanism

**Verification type: Exact**

Verification depends on the specific task type:

**Temporal ordering**:
```python
def verify_ordering(predicted_order: list[str], gold_order: list[str]) -> int:
    return 1 if predicted_order == gold_order else 0
```

**Duration questions**:
```python
def verify_duration(predicted: str, gold: str, tolerance: float = 0.0) -> int:
    pred_val = parse_duration(predicted)  # e.g., "3 years" -> 3.0
    gold_val = parse_duration(gold)
    return 1 if abs(pred_val - gold_val) <= tolerance else 0
```

**Temporal relation classification**:
```python
def verify_relation(predicted: str, gold: str) -> int:
    # Relations: BEFORE, AFTER, DURING, OVERLAPS, etc.
    return 1 if normalize(predicted) == normalize(gold) else 0
```

**Date questions**:
```python
def verify_date(predicted: str, gold: str) -> int:
    pred_date = parse_date(predicted)  # Normalize to YYYY-MM-DD
    gold_date = parse_date(gold)
    return 1 if pred_date == gold_date else 0
```

All verification functions are programmatic and deterministic. Date and duration parsing requires normalization (handling various formats), but the comparison itself is exact.

## Dataset Sources

| Dataset | Task | Size | Source | Notes |
|---------|------|------|--------|-------|
| TempEval-3 | Temporal relation classification | 11K relations | TimeBank + AQUAINT | TimeML annotation, 13 relation types |
| TimeBank | Temporal relation annotation | 183 docs | News articles | Dense temporal annotation |
| MATRES | Temporal ordering | 13.6K relations | TempEval-3 documents | Simplified 4-way relations (BEFORE, AFTER, EQUAL, VAGUE) |
| TempQuestions | Temporal QA | 1.3K questions | Curated | Questions requiring temporal reasoning |
| TimeQA | Temporal QA | 20K questions | Wikidata + Wikipedia | Temporal knowledge questions with time constraints |
| CronQuestions | Temporal KGQA | 410K questions | Wikidata | Temporal knowledge graph QA |
| TDDiscourse | Document-level temporal | 2.6K relations | Wikipedia + news | Cross-sentence temporal relations |
| TimeDial | Temporal dialogue | 1.1K dialogues | Conversations | Temporal reasoning in dialogue |
| ARC-temporal | Temporal science questions | Subset of ARC | Science exams | Temporal aspects of science reasoning |

**Scalability**: Moderate. Wikidata provides millions of temporally-scoped facts (birth dates, event dates, tenure periods) that can be converted to temporal QA pairs. Wikipedia event timelines and biographical data provide additional source material. CronQuestions demonstrates that large-scale temporal QA can be generated from Wikidata.

## Task Format

**Input**: A question about temporal relations, ordering, or duration. Optionally includes a passage with temporal information.

**Output**: A date, duration, ordering, or temporal relation label.

**Prompt template examples**:

Temporal ordering:
```
Order the following events from earliest to latest:
1. {event_a}
2. {event_b}
3. {event_c}

Correct order (by number):
```

Temporal relation:
```
What is the temporal relationship between Event A and Event B?

Event A: {event_a}
Event B: {event_b}

Passage: {context}

Relationship (BEFORE, AFTER, SIMULTANEOUS, or INCLUDES):
```

Duration question:
```
Based on the following information, how long did the event last?

{passage}

Duration:
```

## Difficulty Curriculum

1. **Level 1 — Explicit date comparison**: "Was the Moon landing before or after Woodstock?" Both dates are well-known.
2. **Level 2 — Duration computation**: "How many years did World War II last?" Requires subtracting known dates.
3. **Level 3 — Temporal expression parsing**: "The treaty was signed three days after the battle." Requires resolving relative temporal expressions.
4. **Level 4 — Multi-event ordering**: Order 5+ events on a timeline, some with ambiguous or approximate dates.
5. **Level 5 — Temporal scoping**: "Who was the President during the Cuban Missile Crisis?" Requires matching time periods to facts.
6. **Level 6 — Cross-document temporal reasoning**: Combine temporal information from multiple passages to establish event ordering.
7. **Level 7 — Counterfactual temporal reasoning**: "If X had happened one year earlier, would it have overlapped with Y?" Requires constructing and reasoning over hypothetical timelines.

## Limitations & Risks

- **Dataset scale**: High-quality temporal reasoning datasets are relatively small compared to other RLVR domains. TempEval has ~11K relations; TimeQA has 20K questions. This limits RLVR training data.
- **Temporal expression ambiguity**: "Next Friday" depends on when the text was written. Resolving temporal expressions to absolute dates requires document metadata that may be missing.
- **Approximate dates**: Many historical events have approximate or disputed dates. "Around 1500 BCE" makes exact verification problematic.
- **Changing knowledge**: Temporal facts can become outdated (e.g., "current president" changes over time). Training data must account for temporal reference frames.
- **Annotation difficulty**: Temporal relation annotation has moderate inter-annotator agreement, especially for the VAGUE category. This introduces noise in the reward signal.
- **Granularity mismatches**: An answer of "1945" vs "August 1945" vs "August 15, 1945" may all be correct at different granularities.

## Connections

- **[question-answering-closed](question-answering-closed.md)**: Temporal QA is a specialized form of closed-form QA.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: Temporal KGC predicts time-scoped triples.
- **[fact-verification](fact-verification.md)**: Temporal fact verification requires checking temporal claims.
- **[reading-comprehension](reading-comprehension.md)**: Understanding temporal aspects of narratives is part of comprehension.
- **[information-extraction](information-extraction.md)**: Temporal information extraction (event times, durations) feeds temporal reasoning.
- **[entity-linking](entity-linking.md)**: Temporal context helps disambiguate entities (which "President Bush?").
