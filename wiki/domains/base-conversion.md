---
domain: Number Base Conversion
category: Misc
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Number Base Conversion

## Overview
Convert numbers between arbitrary bases (binary, octal, hex, base64, etc.). Verify via round-trip.

## Verification Mechanism
Convert A→B→A, verify round-trip produces original. Cross-check with Python int(s, base).

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
[[math-numerical]], [[compression-encoding]]
