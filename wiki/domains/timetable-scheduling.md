---
domain: Timetable Scheduling
category: Math
verification_type: constraint
dataset_scale: ~1K+ from ITC competition benchmarks
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Timetable Scheduling

## Overview
Schedule classes/events/exams with no conflicts. Verify all hard constraints (no overlaps, room capacity, teacher availability).

## Verification Mechanism
Check no teacher/room/student has overlapping events. Verify room capacities. Count soft constraint violations.

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
[[scheduling]], [[constraint-programming]]
