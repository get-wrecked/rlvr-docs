---
domain: Word Ladder Puzzles
category: Games
verification_type: constraint
dataset_scale: unlimited (from dictionary)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Word Ladder Puzzles

## Overview
Find word ladder from start to end word, changing one letter at a time, all intermediate words valid.

## Verification Mechanism
Verify each consecutive pair differs by exactly one letter. Verify every word exists in dictionary.

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
[[anagram-wordplay]], [[graph-algorithm-execution]]
