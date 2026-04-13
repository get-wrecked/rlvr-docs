---
domain: Chess Endgame Tablebase Queries
category: Games
verification_type: exact_match
dataset_scale: ~1B+ positions in Syzygy tablebases
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Chess Endgame Tablebase Queries

## Overview
Determine optimal play in chess endgames (DTZ/DTM). Verify against Syzygy/Lomonosov tablebase lookup.

## Verification Mechanism
Query Syzygy tablebase API for position. Verify move leads to optimal DTZ (distance to zeroing).

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
