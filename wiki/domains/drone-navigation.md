---
domain: Drone Navigation & Path Planning
category: Agent
verification_type: simulation
dataset_scale: unlimited (procedural in AirSim/Flightmare)
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Drone Navigation & Path Planning

## Overview
Navigate a drone through obstacle-rich environments to reach goal locations. Requires 3D path planning, obstacle avoidance, and potentially vision-based navigation. AirSim and Flightmare provide realistic simulation.

## Verification Mechanism
Did the drone reach the goal without collision? Flight time, path efficiency, and smoothness metrics. Fully deterministic in simulation.

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
[[robotics-planning]], [[map-navigation]]
