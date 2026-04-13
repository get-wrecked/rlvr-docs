---
domain: Procedural Terrain Generation
category: Creative
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Procedural Terrain Generation

## Overview
Generate realistic terrain heightmaps satisfying constraints (has a river valley, mountain peak above X height, coastline). Verify via geomorphological plausibility metrics.

## Verification Mechanism
Check terrain satisfies all constraints (elevation ranges, feature presence). Verify hydraulic erosion patterns are physically plausible. Compute roughness/fractal dimension.

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
[[game-design-level-generation]], [[geomorphology]]
