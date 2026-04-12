---
domain: Garbage Collector Implementation
category: Code
verification_type: execution
dataset_scale: ~100+ from GC algorithm variants
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# Garbage Collector Implementation

## Overview
Implement GC algorithms (mark-sweep, copying, generational). Verify correctness via memory leak detection and allocation tests.

## Verification Mechanism
Run allocation/deallocation test sequences, verify no memory leaks (valgrind), no use-after-free, correct reclamation.

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
[[memory-management]], [[operating-system-concepts]]
