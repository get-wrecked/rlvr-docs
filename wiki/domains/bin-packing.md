---
domain: Bin Packing
category: Math
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Bin Packing

## Overview
Pack items into minimum bins respecting capacity constraints. Verify all items placed and no overflow.

## Verification Mechanism
Check every item is assigned to exactly one bin. Verify no bin exceeds capacity. Count bins used.

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
[[combinatorics-optimization]], [[scheduling]]
