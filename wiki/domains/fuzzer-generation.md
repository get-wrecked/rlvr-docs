---
domain: Fuzzer Generation
category: Code & Software
verification_type: execution
dataset_scale: 800+ projects in OSS-Fuzz
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Fuzzer Generation

## Overview
Given a target program/API, generate a fuzzing harness. Verification via code coverage achieved and crashes found within a fixed time budget.

## Verification Mechanism
1. Fuzzer compiles and links against target
2. Run fuzzer for fixed time budget (e.g., 60 seconds)
3. Measure code coverage via gcov/llvm-cov
4. Count unique crashes/bugs found
5. Compare against baseline random fuzzing

## Dataset Sources
- Google FuzzBench (standard fuzzer benchmarks)
- OSS-Fuzz corpus (800+ projects with existing harnesses)
- AFL/LibFuzzer seed corpora
- Magma benchmark (real bugs with ground truth)

## Task Format
**Input**: Target source code/API + compilation instructions
**Output**: Fuzzing harness code (LibFuzzer/AFL format)

## Difficulty Curriculum
1. Simple function fuzzing (parse a string)
2. Stateful API fuzzing (sequence of calls)
3. Protocol fuzzing (network input)
4. Kernel/driver fuzzing (syzkaller-style)
5. Grammar-aware fuzzing for structured inputs

## Connections
[[test-generation]], [[code-vulnerability-detection]], [[property-based-test-generation]]
