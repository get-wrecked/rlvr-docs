---
domain: SQL Schema Reverse Engineering
category: Code
verification_type: execution
dataset_scale: ~10K+ from database dumps
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# SQL Schema Reverse Engineering

## Overview
Infer database schema (CREATE TABLE) from query examples or data. Verify schema supports all example queries.

## Verification Mechanism
Create schema in test DB, run all example queries successfully, verify constraints hold.

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
