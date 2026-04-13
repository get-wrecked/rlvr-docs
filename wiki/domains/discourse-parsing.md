---
domain: Discourse Parsing
category: Language
verification_type: diff
dataset_scale: ~10K+ from RST Discourse Treebank
difficulty_range: hard
modality: text
status: strong_hypothesis
---

# Discourse Parsing

## Overview
Parse document-level discourse structure (RST trees). Verify against gold-standard discourse annotations.

## Verification Mechanism
Compare discourse tree structure against gold standard. Compute Parseval-style metrics.

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
[[dependency-parsing]], [[reading-comprehension]]
