---
domain: Color Blindness Simulation
category: Vision
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Color Blindness Simulation

## Overview
Simulate how images/colors appear to colorblind individuals (protanopia, deuteranopia, tritanopia).

## Verification Mechanism
Apply Brettel/Viénot color transformation matrices, compare output pixel values to reference implementation.

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
[[color-space-conversion]], [[accessibility-compliance]]
