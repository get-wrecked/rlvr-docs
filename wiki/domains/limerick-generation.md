---
domain: Limerick Generation
category: Creative
verification_type: rule
dataset_scale: ~10K+ limericks in databases
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Limerick Generation

## Overview
Generate limericks with verified AABBA rhyme scheme and anapestic meter via phoneme analysis.

## Verification Mechanism
Check rhyme scheme via CMU pronouncing dictionary (last phonemes match). Verify syllable count per line (8-8-5-5-8).

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
[[poetry-formal]], [[phonetic-transcription]]
