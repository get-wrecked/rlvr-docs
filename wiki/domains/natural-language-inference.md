---
domain: Natural Language Inference
category: language-reasoning
verification_type: exact_match
dataset_scale: 10M+ (from NLI datasets + mining)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Natural Language Inference

## Overview
Given a premise and hypothesis, determine the logical relationship: entailment, contradiction, or neutral. Verification: match against gold labels. This is a foundational NLU task that underlies many forms of reasoning.

## Verification Mechanism
```python
def verify(premise: str, hypothesis: str, prediction: str, gold_label: str) -> float:
    prediction_norm = prediction.strip().lower()
    gold_norm = gold_label.strip().lower()
    
    # Standard 3-way classification
    valid_labels = {"entailment", "contradiction", "neutral"}
    if prediction_norm not in valid_labels:
        return 0.0
    
    return 1.0 if prediction_norm == gold_norm else 0.0
```

## Dataset Sources
- **SNLI**: 570K sentence pairs, crowdsourced.
- **MultiNLI (MNLI)**: 433K pairs across genres.
- **ANLI (Adversarial NLI)**: 170K adversarially constructed pairs — specifically harder.
- **XNLI**: Cross-lingual NLI in 15 languages.
- **WANLI**: 108K worker-and-AI NLI pairs.
- **DocNLI**: Document-level NLI.
- **FEVER-NLI**: Fact verification reframed as NLI.
- **SciNLI**: Scientific domain NLI.
- **ContractNLI**: Legal contract NLI.
- Can mine new pairs from: Wikipedia contradictions, revision history, edit wars.

## Task Format
- **Input**: Premise: "A man is playing guitar on stage." Hypothesis: "A musician is performing."
- **Output**: "entailment"

## Difficulty Curriculum
- Level 1: Obvious entailments/contradictions (lexical overlap)
- Level 3: Requires world knowledge or common sense
- Level 5: Subtle contradictions, presupposition failures
- Level 7: Adversarial NLI (ANLI R3) — designed to fool models
- Level 9: Complex multi-sentence reasoning, legal/scientific NLI

## Limitations & Risks
- Human agreement on NLI labels is ~93% — there's inherent ambiguity. Some labels are debatable.
- Dataset artifacts allow shortcuts (learning dataset biases rather than reasoning). Mitigate with adversarial datasets (ANLI).
- 3-way classification is a simplification of real logical reasoning.

## Connections
- [[fact-verification]] — NLI applied to fact-checking
- [[reading-comprehension]] — comprehension-based reasoning
- [[logic-propositional]] — NLI is informal logical reasoning
- [[question-answering-extractive]] — related NLU task
