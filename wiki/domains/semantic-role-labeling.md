---
domain: Semantic Role Labeling
category: nlp
verification_type: exact_match
dataset_scale: 113K+ predicates (PropBank)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Semantic Role Labeling

## Overview

Semantic role labeling (SRL) answers "who did what to whom, where, when, and how" by identifying predicates in a sentence and assigning semantic roles (agent, patient, instrument, location, etc.) to their arguments. The output is a predicate-argument structure: for each verb or predicate, the labeled spans of text that fill each role.

SRL has excellent RLVR properties. Gold annotations exist in large quantities, the output is structured (labeled spans), and evaluation uses well-defined span-level F1 metrics. The task requires genuine linguistic understanding -- syntactic parsing alone is insufficient because semantic roles do not map one-to-one to syntactic positions.

## Verification Mechanism

```python
def verify_srl(predicted_frames: list, gold_frames: list) -> dict:
    """
    Each frame = {"predicate": (start, end), "arguments": [{"role": "ARG0", "span": (s, e)}, ...]}
    """
    pred_tuples = set()
    for frame in predicted_frames:
        pred_span = frame["predicate"]
        for arg in frame["arguments"]:
            pred_tuples.add((pred_span, arg["role"], arg["span"]))

    gold_tuples = set()
    for frame in gold_frames:
        pred_span = frame["predicate"]
        for arg in frame["arguments"]:
            gold_tuples.add((pred_span, arg["role"], arg["span"]))

    tp = len(pred_tuples & gold_tuples)
    precision = tp / len(pred_tuples) if pred_tuples else 0.0
    recall = tp / len(gold_tuples) if gold_tuples else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0

    return {
        "precision": precision,
        "recall": recall,
        "f1": f1,
        "reward": f1
    }
```

## Dataset Sources

- **PropBank**: 113K+ annotated predicates over the Penn Treebank (Wall Street Journal). Roles: ARG0 (proto-agent), ARG1 (proto-patient), ARG2-5 (verb-specific), ARGM-TMP, ARGM-LOC, ARGM-MNR, etc. The standard SRL resource.
- **CoNLL-2005 Shared Task**: PropBank-based SRL benchmark. ~40K training predicates, standard train/dev/test splits on WSJ + Brown test set.
- **CoNLL-2009 Shared Task**: Extends to dependency-based SRL in 7 languages (English, Chinese, Czech, German, Japanese, Catalan, Spanish). ~40K predicates per language.
- **FrameNet**: ~200K annotated sentences across 1,200+ semantic frames. Richer frame semantics than PropBank but smaller and more complex annotation scheme.
- **NomBank**: Nominal predicates (noun-based SRL). ~115K instances complementing PropBank's verb focus.
- **Abstract Meaning Representation (AMR)**: 59K sentence-AMR pairs. AMR includes SRL as a component but goes further into full semantic graphs.

## Task Format

- **Input**: Sentence with a target predicate indicated.
```
Sentence: "The company [fired] the manager after the scandal."
Target predicate: "fired"

Identify all semantic roles for the predicate "fired".
```
- **Output**: Labeled argument spans.
```
Predicate: fired
ARG0 (agent): "The company"
ARG1 (patient): "the manager"
ARGM-TMP (temporal): "after the scandal"
```

## Difficulty Curriculum

- Level 1: Simple transitive sentences with clear agent/patient ("The cat ate the fish")
- Level 2: Sentences with adjuncts (time, location, manner)
- Level 3: Passive voice and syntactic alternations ("The fish was eaten by the cat")
- Level 4: Relative clauses, control/raising constructions
- Level 5: Nominal predicates ("The company's destruction of the forest")
- Level 6: Long-distance dependencies, embedded clauses
- Level 7: Metaphorical/figurative predicate use
- Level 8: Multi-predicate sentences with shared arguments
- Level 9: Cross-lingual SRL (CoNLL-2009 multilingual), rare predicates, complex event structures

## Limitations & Risks

- **PropBank frame specificity**: Each verb sense has its own set of numbered arguments (ARG0-5), and the meaning of ARG2 for "give" differs from ARG2 for "run". The model must learn per-verb frame definitions, not just generic role labels.
- **Span boundary ambiguity**: Annotators sometimes disagree on exact span boundaries (should "the big red" be part of the argument span?). Off-by-one errors in span boundaries count as wrong under exact match.
- **Preprocessing pipeline errors**: Traditional SRL systems depend on syntactic parses. If using a pipeline, parse errors propagate. End-to-end approaches avoid this but are harder to train.
- **Genre transfer**: PropBank is predominantly Wall Street Journal text. Performance drops significantly on informal text, dialogue, and literary language.
- **FrameNet coverage**: FrameNet frames provide richer semantics but cover only ~13K lexical units. Many words have no FrameNet frame, limiting evaluation coverage.

## Connections

- [[coreference-resolution]] — SRL identifies "who" within a sentence; coreference links those mentions across sentences
- [[dialogue-state-tracking]] — Slot filling in dialogue draws on the same argument-extraction capabilities
- [[dependency-parsing]] — Syntactic structure strongly correlates with semantic roles
- [[named-entity-recognition]] — NER identifies entity mentions that often fill SRL argument slots
- [[reading-comprehension]] — Many QA tasks implicitly require SRL ("Who did X to Y?")
