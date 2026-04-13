---
domain: ARC-AGI (Abstraction & Reasoning Corpus)
category: Games
verification_type: exact_match
dataset_scale: ~1K from ARC-AGI-2
difficulty_range: hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# ARC-AGI (Abstraction & Reasoning Corpus)

## Overview
Solve ARC tasks: given 2-3 input→output grid pairs demonstrating a transformation rule, apply the rule to a new input. Tests core abstraction ability — the ability to identify the rule from examples. Frontier models score ~50% while humans score ~95%. François Chollet designed this as a direct measure of fluid intelligence.

## Verification Mechanism
Exact pixel-for-pixel match of output grid. No partial credit. The test is binary: either you found the correct transformation or you didn't.

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
[[abstract-pattern-completion]], [[latent-concept-learning]]
