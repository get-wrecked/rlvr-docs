---
domain: Handwritten Math Expression Recognition
category: Vision
verification_type: exact_match
dataset_scale: ~100K+ from CROHME
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Handwritten Math Expression Recognition

## Overview
Recognize handwritten mathematical expressions and produce LaTeX. Verify via LaTeX compilation and rendering comparison.

## Verification Mechanism
Parse generated LaTeX, compare expression tree against gold standard. Verify renders identically.

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
[[optical-character-recognition]], [[latex-typesetting]]
