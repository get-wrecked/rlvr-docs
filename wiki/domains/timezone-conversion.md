---
domain: Timezone Conversion
category: Misc
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Timezone Conversion

## Overview
Convert datetimes between timezones, handling DST transitions. Verify against pytz/IANA database.

## Verification Mechanism
Convert using pytz/zoneinfo reference, verify result matches exactly including DST edge cases.

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
[[date-time-computation]], [[calendar-scheduling]]
