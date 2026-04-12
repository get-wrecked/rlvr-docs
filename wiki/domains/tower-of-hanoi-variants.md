---
domain: Tower of Hanoi Variants
category: Games
verification_type: simulation
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Tower of Hanoi Variants

## Overview
Solve generalized Tower of Hanoi with constraints (Frame-Stewart for 4+ pegs, forbidden moves). Verify valid move sequences.

## Verification Mechanism
Simulate moves, verify each move is legal (smaller on larger), verify final state matches goal.

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
