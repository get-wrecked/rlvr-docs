---
domain: Database Index Selection
category: Systems
verification_type: execution
dataset_scale: unlimited (from query workloads)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Database Index Selection

## Overview
Recommend database indexes for a query workload. Verify by measuring query plan improvements via EXPLAIN.

## Verification Mechanism
Create recommended indexes in test DB, run EXPLAIN on workload queries, verify cost reduction.

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
[[database-query-optimization]], [[sql-generation]]
