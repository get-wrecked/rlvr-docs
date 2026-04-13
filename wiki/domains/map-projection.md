---
domain: Map Projection Computation
category: Science
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Map Projection Computation

## Overview
Convert between map projections (Mercator, Lambert, UTM, stereographic). Verify via PROJ reference.

## Verification Mechanism
Compare projected coordinates against PROJ library output. Verify round-trip error within tolerance.

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
[[cartography]], [[coordinate-system-conversion]]
