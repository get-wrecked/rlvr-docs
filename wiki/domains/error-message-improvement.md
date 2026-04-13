---
domain: Error Message Improvement
category: Code
verification_type: execution
dataset_scale: ~5K+ from compiler/tool error outputs
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Error Message Improvement

## Overview
Improve error messages in code (compilers, CLIs, libraries). Verify improved message contains key diagnostic info.

## Verification Mechanism
Trigger error condition, verify improved message contains: error location, expected vs actual, suggested fix.

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
[[code-repair]], [[code-documentation]]
