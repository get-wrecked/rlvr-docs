---
domain: Video Temporal Reasoning
category: Vision
verification_type: exact_match
dataset_scale: ~10K+ from NExT-QA/EgoSchema
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Video Temporal Reasoning

## Overview
Answer questions requiring temporal reasoning over video: 'What did the person do BEFORE picking up the cup?' Requires understanding event sequences, causal relationships, and temporal ordering in video.

## Verification Mechanism
Compare answer against gold-standard temporal annotations. Multiple choice or free-form.

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
[[video-understanding]], [[temporal-reasoning]]
