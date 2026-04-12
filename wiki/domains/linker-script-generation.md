---
domain: Linker Script Generation
category: Systems
verification_type: execution
dataset_scale: ~1K+ from embedded projects
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Linker Script Generation

## Overview
Generate linker scripts for embedded targets. Verify correct memory layout via objdump inspection.

## Verification Mechanism
Link with generated script, verify section placement via objdump/readelf matches specification.

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
[[embedded-firmware]], [[assembly-generation]]
