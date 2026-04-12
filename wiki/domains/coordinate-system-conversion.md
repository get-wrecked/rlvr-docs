---
domain: Coordinate System Conversion
category: Misc
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Coordinate System Conversion

## Overview
Convert between GPS (WGS84), UTM, MGRS, ECEF, and local coordinate systems. Verify round-trip accuracy.

## Verification Mechanism
Convert A→B→A, verify round-trip error within specified tolerance.

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
[[geospatial-analysis]], [[unit-conversion]]
