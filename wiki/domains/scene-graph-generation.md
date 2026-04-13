---
domain: Scene Graph Generation
category: Vision
verification_type: diff
dataset_scale: ~100K+ from Visual Genome
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Scene Graph Generation

## Overview
Generate scene graphs from images: objects, attributes, and relationships. Verify against gold annotations.

## Verification Mechanism
Compare predicted (subject, predicate, object) triples against gold scene graph. Compute recall@K.

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
[[visual-question-answering]], [[knowledge-graph-completion]]
