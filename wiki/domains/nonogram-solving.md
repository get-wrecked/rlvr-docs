---
domain: Nonogram / Picross Solving
category: Games
verification_type: constraint
dataset_scale: ~100K+ puzzles available online
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Nonogram / Picross Solving

## Overview
Solve nonogram/picross puzzles. Verify all row and column clue constraints are satisfied.

## Verification Mechanism
Check each row and column against its clue (sequence of run lengths). All clues must match exactly.

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
[[logic-puzzles]], [[constraint-programming]]
