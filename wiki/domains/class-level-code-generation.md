---
domain: Class-Level Code Generation
category: Code
verification_type: execution
dataset_scale: ~100 from ClassEval
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Class-Level Code Generation

## Overview
Generate entire Python classes with multiple interacting methods, init, properties, and inheritance. ClassEval benchmark tests holistic class design, not just isolated functions.

## Verification Mechanism
Execute test suite that tests each method independently and their interactions. All tests must pass.

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
[[code-generation]], [[test-generation]]
