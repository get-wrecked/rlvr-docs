---
domain: Program Synthesis from Input-Output Examples
category: code
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Program Synthesis from Input-Output Examples

## Overview
Given a set of input-output examples, synthesize a program that maps inputs to outputs correctly — including on held-out test cases. This is a classic AI problem (inductive program synthesis) reframed for RLVR. Unlike standard code generation (which provides a natural language spec), this requires the agent to infer the pattern from examples alone.

## Verification Mechanism
```python
def verify(io_examples: list[tuple], hidden_tests: list[tuple], code: str) -> float:
    # Check provided examples
    for inp, expected_out in io_examples:
        actual = execute_in_sandbox(code, inp, timeout=5)
        if actual != expected_out:
            return 0.0
    
    # Check hidden test cases (prevents overfitting to examples)
    hidden_passed = 0
    for inp, expected_out in hidden_tests:
        actual = execute_in_sandbox(code, inp, timeout=5)
        if actual == expected_out:
            hidden_passed += 1
    
    return hidden_passed / len(hidden_tests)
```

## Dataset Sources
- **ARC (Abstraction and Reasoning Corpus)**: 400 training + 400 test tasks. Grid transformation problems. Iconic benchmark.
- **ARC-AGI-2**: Extended version with harder tasks.
- **1D-ARC**: One-dimensional simplification.
- **List functions**: Synthesize list transformation functions from examples. Can generate unlimited from random programs.
- **String transformation**: FlashFill-style string transformations. Microsoft's PROSE benchmark.
- **Regex from examples**: Learn regex patterns from positive/negative examples.
- **Karel the Robot**: Generate Karel programs from grid examples. Widely used in program synthesis research.
- Procedural generation: Sample random programs, compute their I/O behavior, use as tasks.

## Task Format
- **Input**: "Given these input-output pairs: [1,2,3]→[2,4,6], [5,10]→[10,20], [0,1,0]→[0,2,0]. Write a function that maps input to output."
- **Output**: `def f(lst): return [x * 2 for x in lst]`

## Difficulty Curriculum
- Level 1: Simple arithmetic transformations (multiply by constant)
- Level 3: String manipulations (capitalize, reverse, split)
- Level 5: Conditional logic (filter, map with conditions)
- Level 7: Recursive patterns, nested structures
- Level 9: ARC-level abstract pattern recognition
- Level 10: Novel algorithmic patterns requiring creative insight

## Limitations & Risks
- Underdetermined: multiple programs may fit the examples. Hidden tests mitigate but don't fully solve this.
- Agent may overfit to examples (e.g., giant lookup table). Mitigate by requiring generalization to held-out tests.
- ARC specifically is designed to resist pattern matching — it tests fluid intelligence. May be too hard for current models.

## Connections
- [[code-generation]] — code synthesis from natural language
- [[automated-reasoning]] — pattern recognition and abstraction
- [[regex-synthesis]] — specific case of synthesis from examples
