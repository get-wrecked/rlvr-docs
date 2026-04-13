---
domain: Load Testing Script Generation
category: Code
verification_type: execution
dataset_scale: ~5K+ from k6/locust examples
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Load Testing Script Generation

## Overview
Generate load testing scripts (k6, Locust, JMeter). Verify by running against mock server and checking metrics.

## Verification Mechanism
Execute load test against mock endpoint, verify request rate/concurrency/assertions match specification.

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
[[test-generation]], [[api-usage]]
