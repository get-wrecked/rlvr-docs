---
domain: Mutation Testing
category: Code & Software
verification_type: execution
dataset_scale: unlimited (procedural from any codebase)
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Mutation Testing

## Overview
Generate code mutations (targeted bugs) to test test-suite quality, or write tests that kill specific mutants. Verification via running tests against mutated code — a mutant should be killed (test fails) if the test suite is adequate.

## Verification Mechanism
1. Mutant compiles successfully
2. Mutant is NOT semantically equivalent to original (at least one test should fail)
3. For "write tests to kill mutants": run new tests against mutants, verify they fail on mutant but pass on original

## Dataset Sources
- Defects4J (835 real Java bugs)
- PIT mutation framework output
- MutantBench
- Major mutation framework
- Any codebase can generate unlimited mutations procedurally

## Task Format
**Input**: Source code + test suite (+ specific mutant for test-writing variant)
**Output**: Mutated code, or tests that kill the mutant

## Difficulty Curriculum
1. Simple arithmetic operator mutations (+ → -)
2. Conditional boundary mutations
3. Return value mutations
4. Statement deletion
5. Higher-order mutations (multiple simultaneous changes)
6. Equivalent mutant detection (hardest — prove mutation doesn't change behavior)

## Limitations & Risks
- Equivalent mutants (mutations that don't change behavior) are undecidable in general
- Test execution overhead scales with number of mutants

## Connections
[[test-generation]], [[code-repair]], [[property-based-test-generation]]
