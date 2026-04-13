---
domain: 15-Puzzle / Sliding Tile
category: Games
verification_type: simulation
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# 15-Puzzle / Sliding Tile

## Overview
Solve the classic 15-puzzle (sliding tiles). Verify move sequence produces goal state.

## Verification Mechanism
Apply move sequence to initial configuration, verify final state matches goal. Check move count optimality.

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
