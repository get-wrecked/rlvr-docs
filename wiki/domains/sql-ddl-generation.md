---
domain: SQL DDL Schema Generation
category: Code
verification_type: execution
dataset_scale: ~100K+ schemas available
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# SQL DDL Schema Generation

## Overview
Generate CREATE TABLE statements with constraints from ER diagrams or requirements. Verify by executing DDL.

## Verification Mechanism
Execute DDL in test database. Verify tables created, constraints enforced (FK, unique, check), types correct.

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
[[database-design]], [[sql-generation]]
