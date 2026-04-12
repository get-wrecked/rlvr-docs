---
domain: Sokoban Solving
category: Games
verification_type: simulation
dataset_scale: ~90K+ levels in standard benchmarks
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Sokoban Solving

## Overview
Solve Sokoban box-pushing puzzles. Verify move sequence places all boxes on goal positions.

## Verification Mechanism
Simulate move sequence, verify all boxes on goals, player doesn't walk through walls, no illegal pushes.

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
[[puzzle-games]], [[planning-classical]]
