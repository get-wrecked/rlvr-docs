---
domain: Makefile / Build Rule Generation
category: Code
verification_type: execution
dataset_scale: ~100K+ Makefiles on GitHub
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Makefile / Build Rule Generation

## Overview
Generate Makefiles with correct dependency tracking and build rules. Verify by building test projects.

## Verification Mechanism
Run make on test project. Verify all targets build successfully. Verify incremental builds work (touch file, only deps rebuild).

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
[[build-configuration]], [[dependency-resolution]]
