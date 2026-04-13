---
domain: Music Score Reading / Transcription
category: Expert
verification_type: exact_match
dataset_scale: ~100K+ from IMSLP/MuseScore
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Music Score Reading / Transcription

## Overview
Read sheet music images and produce MIDI/MusicXML. Verify against ground-truth transcription.

## Verification Mechanism
Compare output notes (pitch, duration, timing) against gold-standard MIDI/MusicXML.

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
[[music-audio-processing]], [[optical-character-recognition]]
