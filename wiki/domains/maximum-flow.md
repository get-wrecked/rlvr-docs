---
domain: Maximum Flow / Min-Cut
category: Math
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Maximum Flow / Min-Cut

## Overview
Compute maximum flow in networks. Verify flow conservation at all nodes and capacity constraints on all edges.

## Verification Mechanism
Check flow conservation (in = out at non-source/sink), capacity constraints, verify value equals min-cut.

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
[[graph-theory]], [[mathematical-programming]]
