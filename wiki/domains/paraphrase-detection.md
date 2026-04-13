---
domain: Paraphrase Detection
category: Language
verification_type: exact_match
dataset_scale: ~500K+ from QQP/MRPC/PAWS
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Paraphrase Detection

## Overview
Determine if two sentences are paraphrases. Verify against labeled paraphrase datasets.

## Verification Mechanism
Compare classification (paraphrase/not) against gold labels. Compute accuracy/F1.

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
[[semantic-textual-similarity]], [[natural-language-inference]]
