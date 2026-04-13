---
domain: Music Key Detection
category: Audio
verification_type: exact_match
dataset_scale: ~50K+ from annotated music datasets
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Music Key Detection

## Overview
Detect the musical key of a piece from its notes/chords. Verify against Krumhansl-Kessler key-finding algorithm.

## Verification Mechanism
Apply Krumhansl-Kessler pitch class profile correlation. Compare to ground-truth key annotations.

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
[[music-theory]], [[music-harmony-analysis]]
