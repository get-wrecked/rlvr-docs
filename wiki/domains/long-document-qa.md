---
domain: Long Document Question Answering
category: Language
verification_type: exact_match
dataset_scale: ~10K+ from NarrativeQA/QuALITY/LongBench
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Long Document Question Answering

## Overview
Answer questions about very long documents (10K-100K+ tokens): novels, legal contracts, research papers, codebases. Tests whether the model can find and reason over relevant information buried in long context.

## Verification Mechanism
Compare answer against gold standard from human annotators. For multiple choice (QuALITY), exact match. For free-form, F1 overlap.

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
[[reading-comprehension]], [[question-answering-extractive]]
