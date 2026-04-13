---
domain: Autonomous Driving Planning
category: Agent
verification_type: simulation
dataset_scale: ~1K+ scenarios from nuPlan/Waymo
difficulty_range: hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Autonomous Driving Planning

## Overview
Plan safe trajectories for autonomous vehicles given sensor data (lidar, camera, map). nuPlan and Waymo Open Motion provide real-world driving scenarios. This is safety-critical — one of the hardest agent tasks.

## Verification Mechanism
Score planned trajectory against safety metrics (collision-free, comfortable, progress toward goal, traffic rule compliance). Closed-loop simulation re-simulates other agents reacting to the plan.

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
[[map-navigation]], [[planning-classical]], [[physics-simulation]]
