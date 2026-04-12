---
domain: Abstract Visual Pattern Completion (ARC-like)
category: synthetic-rule-learning
verification_type: exact_match
dataset_scale: 100K+ (ARC + procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: verified
---

# Abstract Visual Pattern Completion (ARC-like)

## Overview
Given input-output grid pairs demonstrating a transformation rule, apply that rule to a new input grid. This is the ARC (Abstraction and Reasoning Corpus) paradigm — widely considered one of the hardest benchmarks for measuring fluid intelligence. Verification is exact: the output grid must match pixel-for-pixel.

This is the visual analog of NCA rule inference. Instead of temporal sequences, the agent sees spatial transformations and must infer the abstract rule (rotation, color mapping, symmetry completion, flood fill, etc.) then apply it.

## Verification Mechanism
```python
def verify(test_input: np.array, prediction: np.array, ground_truth: np.array) -> float:
    # Exact grid match — no partial credit
    return 1.0 if np.array_equal(prediction, ground_truth) else 0.0
```

Exact match is the only meaningful metric. The output grid must be precisely correct.

## Dataset Sources
- **ARC (Chollet, 2019)**: 400 training + 400 evaluation tasks. The gold standard.
- **ARC-AGI-2 (2025)**: Extended, harder version.
- **1D-ARC**: One-dimensional simplification for curriculum learning.
- **ConceptARC**: Concept-focused ARC variant.
- **LARC (Language-annotated ARC)**: ARC with natural language descriptions.
- **Mini-ARC**: Smaller grid versions for faster iteration.
- **Procedural generation**: 
  - Generate grids + apply random transformations (color swap, rotate, reflect, crop, tile)
  - Compose simple transforms into complex ones
  - Use DSL-based generators (DreamCoder-style)

## Task Format
- **Input**: "Training examples: Input1→Output1, Input2→Output2, Input3→Output3. Test: Input4→?"
- **Output**: The correct output grid for Input4

## Difficulty Curriculum
- Level 1: Color mapping (uniform substitution)
- Level 3: Geometric transforms (rotation, reflection, scaling)
- Level 5: Conditional transforms (if color=X then transform else keep)
- Level 7: Multi-step compositions (rotate + recolor + crop)
- Level 9: Full ARC difficulty (requires novel abstraction)
- Level 10: ARC-AGI-2 / open ARC tasks requiring genuine fluid intelligence

## Limitations & Risks
- ARC tasks are designed to be easy for humans but hard for models — they specifically test generalization, not pattern matching.
- Procedurally generated tasks may not capture the full difficulty of hand-designed ARC tasks.
- This is a Stage 1 (pre-training) environment — it develops the raw ability to infer abstract rules from examples.

## Connections
- [[program-synthesis-from-io]] — program synthesis version of the same problem
- [[cellular-automata-rule-inference]] — temporal rule inference
- [[automated-reasoning]] — abstract reasoning
- [[spatial-reasoning]] — spatial transformation understanding
