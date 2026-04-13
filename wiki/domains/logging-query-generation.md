---
domain: Log Query Generation
category: Code
verification_type: execution
dataset_scale: ~5K+ from observability platforms
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Log Query Generation

## Overview
Generate queries for log analysis tools (Splunk SPL, Elasticsearch DSL, Loki LogQL). Verify by executing against test logs.

## Verification Mechanism
Execute query against test log data, compare extracted events/aggregations to expected results.

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
[[log-anomaly-detection]], [[knowledge-base-querying]]
