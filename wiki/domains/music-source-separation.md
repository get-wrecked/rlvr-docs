---
domain: Music Source Separation
category: Audio
verification_type: diff
dataset_scale: ~100+ from MUSDB18/MedleyDB
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Music Source Separation

## Overview
Separate mixed audio into individual stems (vocals, drums, bass, other). Verify via SDR/SIR/SAR metrics against ground-truth stems.

## Verification Mechanism
Compute Signal-to-Distortion Ratio (SDR) against ground-truth separated stems. Standard metric, fully automated.

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
[[audio-source-separation]], [[signal-processing]]
