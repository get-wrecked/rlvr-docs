---
domain: Intuitive Physics Reasoning
category: Vision
verification_type: exact_match
dataset_scale: ~10K+ from PHYRE/IntPhys/Physion
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Intuitive Physics Reasoning

## Overview
Predict physical outcomes from images/video: Will the tower fall? Which ball reaches the ground first? Where will the object land? PHYRE and Physion test intuitive physics understanding. This is core to how humans understand the world.

## Verification Mechanism
Compare predicted outcome against physics simulation ground truth. For PHYRE, binary success (did the action achieve the goal?). For prediction tasks, compare predicted trajectory to simulated trajectory.

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
[[physics-simulation]], [[spatial-reasoning]]
