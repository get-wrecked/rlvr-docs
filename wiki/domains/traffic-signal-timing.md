---
domain: Traffic Signal Timing Optimization
category: Systems
verification_type: simulation
dataset_scale: unlimited (procedural from traffic data)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Traffic Signal Timing Optimization

## Overview
Optimize traffic signal timing plans to minimize delay/maximize throughput. Verify via traffic simulation (SUMO).

## Verification Mechanism
Simulate traffic with proposed timing in SUMO, measure average delay, throughput, queue lengths. Compare to baseline.

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
[[scheduling]], [[combinatorics-optimization]]
