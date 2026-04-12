---
domain: Travelling Salesman Problem
category: Math
verification_type: constraint
dataset_scale: ~2K+ from TSPLIB
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Travelling Salesman Problem

## Overview
Find shortest tour visiting all cities exactly once and returning to start.

## Verification Mechanism
Verify all cities visited exactly once, tour returns to start, compute total distance, compare to known optimal.

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
[[combinatorics-optimization]], [[graph-theory]]
