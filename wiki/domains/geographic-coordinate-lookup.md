---
domain: Geographic Coordinate Lookup
category: Misc
verification_type: exact_match
dataset_scale: ~10M+ from GeoNames/OpenStreetMap
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Geographic Coordinate Lookup

## Overview
Look up coordinates for place names (geocoding) or place names for coordinates (reverse geocoding). Verify against reference.

## Verification Mechanism
Compare coordinates against GeoNames/OSM reference within distance tolerance.

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
[[geospatial-analysis]], [[geographic-trivia]]
