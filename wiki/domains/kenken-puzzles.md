---
domain: KenKen Puzzles
category: Games
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# KenKen Puzzles

## Overview
Solve KenKen grid puzzles with arithmetic cage constraints and Latin square property.

## Verification Mechanism
Verify each cage satisfies its arithmetic operation and target, verify Latin square (no repeats in row/column).

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
[[logic-puzzles]], [[sudoku-generation]]
