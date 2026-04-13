---
domain: Lights Out Puzzle
category: Games
verification_type: simulation
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Lights Out Puzzle

## Overview
Solve Lights Out puzzles (toggling a light flips it and its neighbors). Verify all lights are off.

## Verification Mechanism
Apply toggle sequence to initial board, verify all cells are off. Can solve via GF(2) linear algebra.

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
[[puzzle-games]], [[linear-algebra-computation]]
