---
domain: Single-Step Retrosynthetic Verification
category: Science
verification_type: execution
dataset_scale: ~1M+ from USPTO reaction database
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Single-Step Retrosynthetic Verification

## Overview
Given a product molecule and proposed retrosynthetic disconnection, verify the step is chemically valid.

## Verification Mechanism
Apply forward reaction template to proposed reactants via RDKit/RDChiral. Verify product matches target.

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
[[chemistry-retrosynthesis]], [[forward-reaction-prediction]]
