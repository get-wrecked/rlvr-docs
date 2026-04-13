---
domain: Email Address Validation
category: Misc
verification_type: rule
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Email Address Validation

## Overview
Validate email addresses against RFC 5321/5322 rules. Handle edge cases (quoted strings, IP literals).

## Verification Mechanism
Validate against reference RFC 5322 parser. Check domain MX record existence.

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
[[data-formatting]], [[protocol-compliance]]
