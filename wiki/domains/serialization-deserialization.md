---
domain: Serialization & Deserialization
category: Code
verification_type: execution
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Serialization & Deserialization

## Overview
Write serialization/deserialization code for various formats (protobuf, MessagePack, Avro, CBOR). Verify via round-trip testing.

## Verification Mechanism
Serialize data, deserialize, assert equality with original. Test edge cases (nested, optional, large payloads).

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
[[structured-output-generation]], [[api-schema-generation]]
