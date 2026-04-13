---
domain: File Format Identification
category: Misc
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# File Format Identification

## Overview
Identify file formats from magic bytes / file headers. Verify against the file(1) command / libmagic.

## Verification Mechanism
Compare identified format against libmagic / file(1) command output.

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
[[compression-encoding]], [[data-formatting]]
