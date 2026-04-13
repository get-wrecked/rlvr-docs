---
domain: Meta-World (Robotic Manipulation Meta-RL)
category: Agent
verification_type: outcome
dataset_scale: 50 manipulation tasks
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Meta-World (Robotic Manipulation Meta-RL)

## Overview
Solve 50 distinct robotic manipulation tasks (push, pick-place, open drawer, turn dial, etc.) in MuJoCo simulation. The meta-RL challenge: learn to quickly adapt to new tasks from the same distribution. Key benchmark for few-shot robot learning.

## Verification Mechanism
Task success rate per environment (binary: did the object reach target position/state?). MuJoCo provides exact object state for deterministic verification.

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
[[robotics-planning]], [[inverse-kinematics]]
