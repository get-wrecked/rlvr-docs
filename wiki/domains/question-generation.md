---
domain: Question Generation
category: Language
verification_type: execution
dataset_scale: ~100K+ from SQuAD/NQ
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Question Generation

## Overview
Generate questions from passages that are answerable from the text. Verify by running QA model on generated question.

## Verification Mechanism
Run extractive QA model on (passage, generated question). Verify extracted answer matches intended answer span.

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
[[question-answering-extractive]], [[educational-content]]
