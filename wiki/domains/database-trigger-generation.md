---
domain: Database Trigger & Procedure Generation
category: Code
verification_type: execution
dataset_scale: ~5K+ from database courses
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Database Trigger & Procedure Generation

## Overview
Generate database triggers, stored procedures, and event handlers. Verify by executing against test data.

## Verification Mechanism
Create trigger in test DB, perform triggering action, verify side effects (audit log, cascade, validation) match spec.

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
[[sql-generation]], [[state-machine-implementation]]
