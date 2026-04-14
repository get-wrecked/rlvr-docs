---
domain: Hopper (MuJoCo)
category: Agent
verification_type: outcome
dataset_scale: 1 environment
difficulty_range: medium/hard
modality: multimodal
status: remembered
---

# Hopper (MuJoCo)

## Overview
Hopper (MuJoCo). Data source recalled from training data.

## Verification Mechanism
outcome verification.

## Dataset Sources & Reconstruction
REMEMBERED: Gymnasium MuJoCo. One-legged hopper. gym.make('Hopper-v4'). Apache.

## Task Format
**Input**: Problem specification
**Output**: Solution

## Difficulty Curriculum
medium/hard

## Limitations & Risks
Data source from memory — verify before use.
