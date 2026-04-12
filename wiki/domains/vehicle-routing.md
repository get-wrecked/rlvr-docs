---
domain: Vehicle Routing Problem
category: Math
verification_type: constraint
dataset_scale: ~1K+ from CVRPLIB
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Vehicle Routing Problem

## Overview
Solve VRP: find optimal routes for fleet visiting all customers within capacity and time windows.

## Verification Mechanism
Verify all customers visited exactly once, vehicle capacity not exceeded, time windows respected, compute total distance.

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
[[combinatorics-optimization]], [[scheduling]], [[map-navigation]]
