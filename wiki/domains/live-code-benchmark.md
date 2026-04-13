---
domain: Live Code Benchmark (Contamination-Free)
category: Code
verification_type: execution
dataset_scale: continuously growing
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Live Code Benchmark (Contamination-Free)

## Overview
Solve coding problems released AFTER model training cutoff, eliminating data contamination. LiveCodeBench, CodeContests, and similar continuously collect new competitive programming problems. This measures true coding ability, not memorization.

## Verification Mechanism
Execute generated code against hidden test cases. Pass/fail per test case, score = fraction passed.

## Dataset Sources
See wiki for specific URLs and download instructions.

## Task Format
**Input**: Problem specification
**Output**: Solution in appropriate format

## Difficulty Curriculum
Scales from basic to expert-level within the domain.

## Limitations & Risks
Domain-specific edge cases may require careful handling.

## Connections
[[competitive-programming-interactive]], [[code-generation]]
