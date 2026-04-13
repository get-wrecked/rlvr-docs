---
domain: Knowledge Distillation
category: ML
verification_type: execution
dataset_scale: unlimited (from any teacher model)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Knowledge Distillation

## Overview
Distill a large teacher model into a smaller student model while preserving performance. Verify student matches or approaches teacher accuracy at fraction of parameters.

## Verification Mechanism
Compare student test accuracy to teacher. Student must achieve >X% of teacher accuracy with <Y% of parameters. Fully automated.

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
[[ml-pipeline-optimization]], [[code-optimization]]
