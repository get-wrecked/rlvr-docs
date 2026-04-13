---
domain: Semantic Version Comparison
category: Misc
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy
modality: text
status: strong_hypothesis
---

# Semantic Version Comparison

## Overview
Parse and compare semantic version strings (1.2.3-beta < 1.2.3). Verify ordering against semver spec.

## Verification Mechanism
Parse versions per semver.org spec, verify comparison results match specification.

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
[[dependency-resolution]], [[data-formatting]]
