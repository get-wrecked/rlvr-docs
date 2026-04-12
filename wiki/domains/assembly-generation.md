---
domain: Assembly Generation
category: Code & Software
verification_type: execution
dataset_scale: ~10K+ from ExeBench + university assignments
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Assembly Generation

## Overview
Given a specification or C equivalent, generate x86/ARM/RISC-V assembly. Verification by assembling, linking, executing with test inputs, and comparing outputs.

## Verification Mechanism
1. Assemble with nasm/gas without errors
2. Link and execute with test inputs
3. Compare output to expected results
4. Optionally verify register/memory state via GDB scripting

## Dataset Sources
- ExeBench (assembly from compiled C)
- CS:APP lab assignments
- NASM tutorial exercises
- University computer architecture courses (MIPS/RISC-V)
- AnghaBench compiled outputs

## Task Format
**Input**: Function specification or C equivalent
**Output**: x86-64, ARM, or RISC-V assembly

## Difficulty Curriculum
1. Simple arithmetic functions
2. Array/string manipulation
3. Calling convention compliance
4. SIMD/vectorization (SSE, AVX, NEON)
5. Hand-optimized assembly beating compiler output

## Connections
[[code-generation]], [[compiler-tasks]], [[reverse-engineering]], [[code-optimization]]
