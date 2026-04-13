---
domain: Inverse Kinematics
category: Science
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Inverse Kinematics

## Overview
Compute joint angles for a robot arm to reach a target end-effector position. Verify via forward kinematics.

## Verification Mechanism
Apply computed joint angles via forward kinematics, verify end-effector position matches target within tolerance.

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
[[robotics-planning]], [[geometry-computation]]
