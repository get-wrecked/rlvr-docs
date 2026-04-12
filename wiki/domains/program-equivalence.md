---
domain: Program Equivalence Checking
category: formal-methods
verification_type: execution
dataset_scale: unlimited (from codebases)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Program Equivalence Checking

## Overview
Determine whether two programs are semantically equivalent (produce the same output for all inputs). This is a fundamental verification task. Verification: if the agent claims equivalent, test on many inputs + try to find a distinguishing input. If the agent claims non-equivalent, verify the provided counterexample.

## Verification Mechanism
```python
def verify(program_a: str, program_b: str, claim: dict) -> float:
    if claim["equivalent"]:
        # Try to find a counterexample
        for _ in range(10000):
            test_input = generate_random_input(program_a)
            output_a = execute_safe(program_a, test_input)
            output_b = execute_safe(program_b, test_input)
            if output_a != output_b:
                return 0.0  # Found counterexample → wrong claim
        
        # Passed all random tests
        return 0.9  # High confidence but not proven
        
        # For formal verification: use symbolic execution
        # formal_result = check_equivalence_smt(program_a, program_b)
        # return 1.0 if formal_result.equivalent else 0.0
    
    else:
        # Agent claims non-equivalent and provides counterexample
        ce = claim["counterexample"]
        output_a = execute_safe(program_a, ce)
        output_b = execute_safe(program_b, ce)
        return 1.0 if output_a != output_b else 0.0
```

## Dataset Sources
- **Code clone detection datasets**: BigCloneBench (25K+ Java clone pairs).
- **Code refactoring pairs**: Before/after refactoring (should be equivalent).
- **Mutation testing**: Mutate code, determine if mutation is equivalent.
- **Competitive programming**: Same problem solved differently by different programmers.
- **Compiler optimization**: Optimized vs. unoptimized code (should be equivalent).
- **Procedural generation**: Generate two versions of the same algorithm, possibly with bugs.

## Task Format
- **Input**: Two programs + "Are these programs equivalent? If not, provide a distinguishing input."
- **Output**: "Equivalent" or "Not equivalent, counterexample: input = 5"

## Difficulty Curriculum
- Level 1: Simple arithmetic expressions
- Level 3: Loop optimizations, constant folding
- Level 5: Complex algorithmic equivalences (different algorithms, same result)
- Level 7: Subtle semantic differences (integer overflow, floating point)
- Level 9: Programs with side effects, concurrency

## Limitations & Risks
- Proving equivalence is undecidable in general. We can disprove with counterexamples (reliable) or gain high confidence with testing (good but not perfect).
- SMT-based formal equivalence checking works for bounded programs.
- Non-equivalence + counterexample has perfect verification.

## Connections
- [[code-refactoring]] — refactoring should preserve equivalence
- [[code-translation]] — translation should preserve equivalence
- [[smt-solving]] — formal equivalence via SMT
- [[compiler-tasks]] — compiler correctness is program equivalence
