---
domain: Minesweeper
category: Games
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Minesweeper

## Overview
Deduce mine locations from number constraints in Minesweeper. Verify against known board configuration.

## Verification Mechanism
Check deduced mine positions against actual board. Verify logical consistency with all revealed numbers.

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
