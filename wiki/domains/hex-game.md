---
domain: Hex Game
category: Games
verification_type: outcome
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Hex Game

## Overview
Play Hex optimally on small boards (up to 11x11). Verify winner via connection checking algorithm.

## Verification Mechanism
Check if a connected path exists from one side to the other for the winning player. Verify move legality.

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
[[board-games]], [[go-game]]
