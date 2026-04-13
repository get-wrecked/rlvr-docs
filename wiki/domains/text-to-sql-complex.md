---
domain: Complex Text-to-SQL (Multi-table)
category: Code
verification_type: execution
dataset_scale: ~10K+ from Spider/BIRD
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Complex Text-to-SQL (Multi-table)

## Overview
Convert complex natural language questions to SQL queries involving multiple tables, subqueries, aggregations.

## Verification Mechanism
Execute generated SQL against test database, compare result set to gold query result.

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
