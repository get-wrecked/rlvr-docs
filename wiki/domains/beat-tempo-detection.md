---
domain: Beat Tracking & Tempo Detection
category: Audio
verification_type: exact_match
dataset_scale: ~5K+ from MIREX/Ballroom dataset
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Beat Tracking & Tempo Detection

## Overview
Detect tempo (BPM) and beat positions from audio. Verify against annotated beat timestamps.

## Verification Mechanism
Compare detected BPM to gold BPM within tolerance. Compare beat positions (F-measure with tolerance window).

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
[[music-audio-processing]], [[signal-processing]]
