---
domain: Sentence Ordering / Text Coherence
category: Language
verification_type: exact_match
dataset_scale: ~50K+ from NIPS/arXiv shuffled abstracts
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Sentence Ordering / Text Coherence

## Overview
Reorder shuffled sentences into coherent paragraph. Verify against original order.

## Verification Mechanism
Compare predicted order against original sentence order. Compute Kendall's tau or perfect match accuracy.

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
[[reading-comprehension]], [[summarization-extractive]]
