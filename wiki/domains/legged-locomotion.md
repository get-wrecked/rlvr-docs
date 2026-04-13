---
domain: Legged Locomotion on Terrain
category: Agent
verification_type: outcome
dataset_scale: unlimited (procedural terrain)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Legged Locomotion on Terrain

## Overview
Control legged robots (quadrupeds, bipeds, hexapods) to traverse rough terrain, stairs, gaps, and obstacles. DeepMind and ETH Zurich have shown remarkable sim-to-real transfer. Tests proprioceptive control and adaptation.

## Verification Mechanism
Distance traveled, time to reach goal, stability metrics (no falls), energy efficiency. Verified by MuJoCo/Isaac Sim physics.

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
[[robotics-planning]], [[control-systems]], [[pid-controller-tuning]]
