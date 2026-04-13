---
domain: Cross-Domain Expert Questions (HLE-style)
category: Expert
verification_type: exact_match
dataset_scale: ~3K+ from hardest expert questions
difficulty_range: superhuman
modality: text
status: strong_hypothesis
---

# Cross-Domain Expert Questions (HLE-style)

## Overview
Answer the hardest verifiable questions across all academic disciplines — the kind that only a handful of domain experts can answer. Questions sourced from professors, researchers, and competition designers. Frontier models score <10%.

## Verification Mechanism
Compare answer to expert-verified gold standard. Questions are designed to have unambiguous correct answers.

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
[[graduate-level-qa]], [[math-competition]]
