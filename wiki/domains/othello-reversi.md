---
domain: Othello / Reversi
category: Games
verification_type: outcome
dataset_scale: unlimited (self-play)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Othello / Reversi

## Overview
Play Othello/Reversi with legal move generation and scoring. Verify move legality and final score.

## Verification Mechanism
Verify each move flips correct discs according to rules. Count final discs for scoring.

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
