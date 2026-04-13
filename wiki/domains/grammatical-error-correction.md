---
domain: Grammatical Error Correction
category: Language
verification_type: diff
dataset_scale: ~50K+ from BEA/CoNLL shared tasks
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Grammatical Error Correction

## Overview
Correct grammatical errors in text. Verify against gold-standard corrections from BEA shared task.

## Verification Mechanism
Compare corrections against gold edits. Compute ERRANT-based precision/recall/F0.5.

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
[[spelling-grammar]], [[text-simplification]]
