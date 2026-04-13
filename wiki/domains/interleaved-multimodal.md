---
domain: Interleaved Image-Text Reasoning
category: Vision
verification_type: exact_match
dataset_scale: ~5K+ from MuirBench/DEMON
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Interleaved Image-Text Reasoning

## Overview
Reason over sequences of interleaved images and text (instruction manuals, comics, tutorials, scientific figures with captions). Requires tracking information across both modalities.

## Verification Mechanism
Compare answer to gold standard. Tasks include ordering, comparison, difference detection — all with verifiable correct answers.

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
[[multimodal-reasoning]], [[visual-question-answering]]
