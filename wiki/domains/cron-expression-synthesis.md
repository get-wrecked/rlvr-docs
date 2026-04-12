---
domain: Cron Expression Synthesis
category: Systems
verification_type: exact_match
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Cron Expression Synthesis

## Overview
Generate cron expressions from natural language schedules. Verify by expanding to next N timestamps and comparing.

## Verification Mechanism
Expand cron expression and reference to next 100 occurrence timestamps, verify exact match.

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
[[calendar-scheduling]], [[date-time-computation]]
