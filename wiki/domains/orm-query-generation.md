---
domain: ORM Query Generation
category: Code
verification_type: execution
dataset_scale: ~10K+ adapted from Spider/Bird
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# ORM Query Generation

## Overview
Generate ORM code (SQLAlchemy, Django ORM, Prisma) from natural language. Verify by executing against seeded test DB and comparing results.

## Verification Mechanism
Execute ORM code against test database, compare result set to expected output or equivalent raw SQL result.

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
[[sql-generation]], [[database-query-optimization]]
