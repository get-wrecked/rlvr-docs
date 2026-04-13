---
domain: Sliding Block / Klotski Puzzles
category: Games
verification_type: simulation
dataset_scale: ~1K+ standard puzzles
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Sliding Block / Klotski Puzzles

## Overview
Solve sliding block puzzles (Klotski). Move blocks within grid to get target block to exit.

## Verification Mechanism
Simulate each move (verify block doesn't overlap others), verify target block reaches exit position.

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
[[puzzle-games]], [[sokoban-solving]]
