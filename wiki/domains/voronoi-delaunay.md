---
domain: Voronoi / Delaunay Computation
category: Math
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Voronoi / Delaunay Computation

## Overview
Compute Voronoi diagrams and Delaunay triangulations. Verify Delaunay property (no point inside circumcircle).

## Verification Mechanism
For each triangle, verify no other point lies inside its circumcircle. Verify Voronoi-Delaunay duality.

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
[[geometry-computation]], [[graph-theory]]
