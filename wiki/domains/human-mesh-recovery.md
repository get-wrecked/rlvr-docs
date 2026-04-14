---
domain: Human Mesh Recovery (3D Body)
category: Vision
verification_type: diff
dataset_scale: ~100K from 3DPW/Human3.6M
difficulty_range: hard
modality: multimodal
status: remembered
---

# Human Mesh Recovery (3D Body)

## Overview
Human Mesh Recovery (3D Body). Data source recalled from training data.

## Verification Mechanism
diff verification.

## Dataset Sources & Reconstruction
REMEMBERED: 3DPW (outdoor), Human3.6M (indoor). Recover 3D SMPL body mesh from image.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
hard

## Limitations & Risks
Data source from memory — verify before use.
