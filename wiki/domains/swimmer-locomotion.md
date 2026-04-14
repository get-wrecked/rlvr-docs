---
domain: Swimmer (MuJoCo)
category: Agent
verification_type: outcome
dataset_scale: 1 environment
difficulty_range: medium
modality: multimodal
status: remembered
---

# Swimmer (MuJoCo)

## Overview
Swimmer (MuJoCo). Data source recalled from training data.

## Verification Mechanism
outcome verification.

## Dataset Sources & Reconstruction
REMEMBERED: Gymnasium MuJoCo. 3-link snake swimming. gym.make('Swimmer-v4'). Apache.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
medium

## Limitations & Risks
Data source from memory — verify before use.
