---
domain: Property-Based Test Generation
category: Code & Software
verification_type: execution
dataset_scale: unlimited (from any codebase with known bugs)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Property-Based Test Generation

## Overview
Given a function specification, generate QuickCheck/Hypothesis-style property-based tests. Properties must hold for all random inputs. Verification: properties are valid (pass on correct implementation) and catch bugs (fail on known-buggy versions).

## Verification Mechanism
1. Properties don't fail on correct implementation (no false positives)
2. Properties catch bugs on known-buggy implementations (sensitivity)
3. Properties are non-trivial (not tautologies — tested via mutant killing rate)

## Dataset Sources
- QuickCheck/Hypothesis case studies
- Defects4J, BugsInPy (buggy versions for property sensitivity testing)
- Haskell QuickCheck examples
- PropEr (Erlang) examples

## Task Format
**Input**: Function signature + specification (natural language or formal)
**Output**: Property-based test code (Hypothesis/QuickCheck/fast-check)

## Difficulty Curriculum
1. Simple algebraic properties (commutativity, associativity)
2. Roundtrip properties (encode/decode, serialize/deserialize)
3. Invariant properties (sorted output, balanced tree)
4. Metamorphic properties (changing input in known way → predictable output change)
5. Stateful property testing (sequences of operations maintain invariants)

## Connections
[[test-generation]], [[mutation-testing]], [[fuzzer-generation]]
