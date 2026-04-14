---
domain: Walker2D (MuJoCo)
category: Agent
verification_type: outcome
dataset_scale: 1 environment
difficulty_range: medium/hard
modality: multimodal
status: remembered
---

# Walker2D (MuJoCo)

## Overview
Walker2D (MuJoCo). Data source recalled from training data.

## Verification Mechanism
outcome verification.

## Dataset Sources & Reconstruction
REMEMBERED: Gymnasium MuJoCo. 2D bipedal walking. gym.make('Walker2d-v4'). Apache.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
medium/hard

## Limitations & Risks
Data source from memory — verify before use.
