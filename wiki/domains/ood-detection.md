---
domain: Out-of-Distribution Detection
category: ML
verification_type: exact_match
dataset_scale: ~100K+ from OpenOOD benchmark
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Out-of-Distribution Detection

## Overview
Detect whether inputs are from the training distribution or out-of-distribution. Critical for safe deployment. OpenOOD provides standardized benchmarks.

## Verification Mechanism
AUROC and FPR@95TPR on ID vs. OOD test sets. Fully automated metric computation.

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
[[anomaly-detection]], [[image-classification]]
