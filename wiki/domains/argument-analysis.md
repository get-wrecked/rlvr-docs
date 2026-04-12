---
domain: Argument Analysis & Logical Fallacy Detection
category: reasoning-language
verification_type: exact_match
dataset_scale: 50K+ (from logic courses + debate corpora)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Argument Analysis & Logical Fallacy Detection

## Overview
Analyze arguments for validity, identify logical fallacies, evaluate argument strength. Verification: match against gold labels from curated datasets of arguments with known fallacies and validity judgments.

## Verification Mechanism
```python
def verify(task_type: str, argument: str, prediction: str, gold: str) -> float:
    if task_type == "fallacy_detection":
        return 1.0 if prediction.strip().lower() == gold.strip().lower() else 0.0
    
    elif task_type == "validity":
        return 1.0 if prediction.strip().lower() == gold.strip().lower() else 0.0
    
    elif task_type == "identify_premise_conclusion":
        pred_parts = parse_argument_parts(prediction)
        gold_parts = parse_argument_parts(gold)
        premise_match = set_f1(set(pred_parts["premises"]), set(gold_parts["premises"]))
        conclusion_match = 1.0 if pred_parts["conclusion"] == gold_parts["conclusion"] else 0.0
        return 0.5 * premise_match + 0.5 * conclusion_match
```

## Dataset Sources
- **Logic (PropBank)**: Labeled arguments with fallacy types.
- **LSAT Logical Reasoning**: Standardized test questions.
- **ArgKP**: Argument key point matching.
- **Fallacy detection datasets**: Multiple academic datasets.
- **Debate corpora**: Kialo, ProCon.org with structured arguments.
- **Critical Thinking courses**: Exercise sets with solutions.
- **ArguAna**: Argument analysis dataset.

## Task Format
- **Input**: "'We should ban cars because cars cause accidents, and anything that causes harm should be banned.' — Identify the logical fallacy."
- **Output**: "Overgeneralization / hasty generalization"

## Difficulty Curriculum
- Level 1: Obvious formal fallacies (affirming the consequent, etc.)
- Level 3: Informal fallacies (ad hominem, straw man)
- Level 5: Subtle fallacies in realistic arguments
- Level 7: LSAT-level logical reasoning
- Level 9: Evaluating complex multi-step arguments

## Connections
- [[natural-language-inference]] — logical entailment
- [[legal-reasoning]] — legal argumentation
- [[commonsense-reasoning]] — common reasoning patterns
