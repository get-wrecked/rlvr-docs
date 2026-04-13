---
domain: 3D Printing G-code Generation
category: Code
verification_type: execution
dataset_scale: ~10K+ from Thingiverse models
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# 3D Printing G-code Generation

## Overview
Generate or optimize G-code for 3D printers from STL models. Verify via G-code validator and print simulation.

## Verification Mechanism
Parse G-code, verify all commands are valid, simulate print path, verify no collisions, check layer adhesion geometry.

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
[[cad-modeling]], [[code-generation]]
