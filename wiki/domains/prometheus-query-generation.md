---
domain: Prometheus / PromQL Query Generation
category: Code
verification_type: execution
dataset_scale: ~10K+ from monitoring dashboards
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Prometheus / PromQL Query Generation

## Overview
Generate PromQL queries for monitoring/alerting. Verify by executing against test metrics data.

## Verification Mechanism
Execute PromQL against test Prometheus instance with known metrics. Compare result to expected time series.

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
[[logging-query-generation]], [[time-series-forecasting]]
