---
domain: Data Augmentation Strategy Design
category: ML
verification_type: execution
dataset_scale: unlimited (from any ML task)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Data Augmentation Strategy Design

## Overview
Design data augmentation pipelines (transforms, mixing strategies, synthetic generation) that improve model generalization. AutoAugment, RandAugment, and AugMax showed this is learnable.

## Verification Mechanism
Train model with and without proposed augmentation. Compare test accuracy. Must improve without degrading performance. Fully automated.

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
[[ml-pipeline-optimization]], [[feature-engineering]]
