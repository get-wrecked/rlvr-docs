---
domain: Commonsense Reasoning
category: reasoning
verification_type: exact_match
dataset_scale: 1M+ (from multiple benchmarks)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Commonsense Reasoning

## Overview
Answer questions requiring commonsense knowledge about the physical world, social interactions, temporal events, and spatial relationships. Verification: exact match against gold labels from curated benchmarks. This develops the "obvious" knowledge that even sophisticated models sometimes lack.

## Verification Mechanism
```python
def verify(question: str, prediction: str, gold: str, task_format: str) -> float:
    if task_format == "multiple_choice":
        return 1.0 if prediction.strip().upper() == gold.strip().upper() else 0.0
    elif task_format == "true_false":
        return 1.0 if prediction.strip().lower() == gold.strip().lower() else 0.0
    elif task_format == "fill_blank":
        return 1.0 if normalize(prediction) == normalize(gold) else 0.0
```

## Dataset Sources
- **PIQA**: 20K physical intuition questions ("How do you clean a blender?")
- **CommonsenseQA**: 12K multiple-choice commonsense questions.
- **CommonsenseQA 2.0**: Harder adversarial version.
- **WinoGrande**: 44K coreference resolution requiring commonsense.
- **HellaSwag**: 70K sentence completion with commonsense.
- **COPA**: Choice of Plausible Alternatives.
- **Social IQA**: 38K questions about social situations.
- **PhysicalIQA**: Physical commonsense reasoning.
- **OpenBookQA**: Science commonsense with an "open book."
- **ARC (AI2 Reasoning Challenge)**: 8K grade-school science questions.
- **CODAH**: 3K adversarially authored commonsense.
- **TuringAdvice**: Situational judgment.
- **ConceptNet**: Commonsense knowledge graph (for knowledge, not directly as task).

## Task Format
- **Input**: "What is the most likely result of 'playing in the rain without an umbrella'? A) Getting wet B) Finding a rainbow C) Learning to swim D) Building a house"
- **Output**: "A"

## Difficulty Curriculum
- Level 1: Basic physical commonsense (PIQA easy)
- Level 3: Social commonsense (Social IQA)
- Level 5: Multi-step commonsense reasoning
- Level 7: Adversarial commonsense (CODAH, CommonsenseQA 2.0)
- Level 9: Novel situations requiring genuine inference from commonsense knowledge

## Limitations & Risks
- Gold labels have ~95% human agreement — some inherent ambiguity.
- Multiple-choice format may not test deep reasoning (can exploit statistical shortcuts).
- Adversarial benchmarks (WinoGrande, HellaSwag) mitigate but don't eliminate this.

## Connections
- [[natural-language-inference]] — reasoning about language
- [[question-answering-closed]] — QA format
- [[spatial-reasoning]] — spatial commonsense
- [[temporal-reasoning]] — temporal commonsense
