---
domain: Battleship
category: Games
verification_type: outcome
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Battleship

## Overview
Play Battleship: place ships and make targeting decisions. Verify ship placement legality and hit/miss accuracy.

## Verification Mechanism
Verify ships don't overlap or extend off grid. Verify each shot result (hit/miss/sink) is correct against hidden board.

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
[[puzzle-games]], [[probability-statistics]]
