---
domain: API Mock Generation
category: Code
verification_type: execution
dataset_scale: ~10K+ from OpenAPI specs
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# API Mock Generation

## Overview
Generate mock API servers from specifications. Verify by running client tests against mock.

## Verification Mechanism
Start mock server, run API client tests, verify all requests return spec-compliant responses.

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
[[api-schema-generation]], [[test-generation]]
