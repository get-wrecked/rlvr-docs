---
domain: Connect Four
category: Games
verification_type: outcome
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Connect Four

## Overview
Play Connect Four optimally. Verify legal moves and win detection (4 in a row).

## Verification Mechanism
Check each move is legal (column not full). Detect wins by checking rows, columns, diagonals for 4 consecutive.

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
[[board-games]], [[multi-agent-coordination]]
