---
domain: Humanoid (MuJoCo)
category: Agent
verification_type: outcome
dataset_scale: 1 environment
difficulty_range: hard
modality: multimodal
status: remembered
---

# Humanoid (MuJoCo)

## Overview
Humanoid (MuJoCo). Data source recalled from training data.

## Verification Mechanism
outcome verification.

## Dataset Sources & Reconstruction
REMEMBERED: Gymnasium MuJoCo. 3D humanoid walking. gym.make('Humanoid-v4'). Apache. 17 DOF.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
hard

## Limitations & Risks
Data source from memory — verify before use.
