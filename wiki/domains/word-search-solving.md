---
domain: Word Search Puzzle Solving
category: Games
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Word Search Puzzle Solving

## Overview
Find all hidden words in a word search grid (horizontal, vertical, diagonal). Verify found positions.

## Verification Mechanism
For each found word, verify characters at claimed positions spell the word in a straight line.

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
[[anagram-wordplay]], [[puzzle-games]]
