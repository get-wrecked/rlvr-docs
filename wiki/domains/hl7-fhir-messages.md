---
domain: HL7/FHIR Healthcare Message Generation
category: Expert
verification_type: constraint
dataset_scale: ~10K+ from FHIR spec examples
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# HL7/FHIR Healthcare Message Generation

## Overview
Generate valid HL7 v2/FHIR healthcare messages. Verify against official schema validators.

## Verification Mechanism
Validate against FHIR StructureDefinition schemas, check required fields, verify code systems.

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
[[structured-output-generation]], [[medical-coding]]
