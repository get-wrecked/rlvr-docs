---
domain: Shogi (Japanese Chess)
category: Games
verification_type: rule
dataset_scale: ~50K+ from Shogi databases
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Shogi (Japanese Chess)

## Overview
Play Shogi with legal move generation, piece drops, and promotion rules. Verify move legality via Shogi engine.

## Verification Mechanism
Verify each move is legal according to Shogi rules. Evaluate position quality against Shogi engine (YaneuraOu).

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
[[chess]], [[board-games]]
