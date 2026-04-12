---
domain: Dependency Parsing
category: Language
verification_type: diff
dataset_scale: ~100K+ from Universal Dependencies treebanks
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Dependency Parsing

## Overview
Parse sentences into dependency trees. Verify against gold-standard Universal Dependencies treebanks.

## Verification Mechanism
Compare output tree (head + deprel for each token) against gold standard. Compute UAS/LAS scores.

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
[[semantic-parsing]], [[coreference-resolution]]
