---
domain: Point Cloud Semantic Segmentation
category: Vision
verification_type: diff
dataset_scale: ~15K from S3DIS/ScanNet
difficulty_range: hard
modality: multimodal
status: remembered
---

# Point Cloud Semantic Segmentation

## Overview
Point Cloud Semantic Segmentation. Data source recalled from training data.

## Verification Mechanism
diff verification.

## Dataset Sources & Reconstruction
REMEMBERED: Stanford S3DIS (large-scale indoor). ScanNet point clouds. SemanticKITTI for outdoor.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
hard

## Limitations & Risks
Data source from memory — verify before use.
