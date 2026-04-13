---
domain: Digital Circuit Optimization
category: Systems
verification_type: simulation
dataset_scale: unlimited (procedural)
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Digital Circuit Optimization

## Overview
Optimize digital circuits for area, power, and timing. Given a gate-level netlist, find an equivalent circuit with fewer gates/shorter critical path. Verify functional equivalence + metric improvement.

## Verification Mechanism
Verify functional equivalence via simulation (same truth table). Measure gate count, critical path delay, estimated power. Must improve at least one metric without degrading others.

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
[[chip-design-rtl]], [[compiler-optimization-passes]]
