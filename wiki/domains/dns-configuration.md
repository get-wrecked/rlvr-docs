---
domain: DNS Configuration
category: Systems
verification_type: execution
dataset_scale: ~10K+ zone file examples
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# DNS Configuration

## Overview
Generate DNS zone files. Verify by parsing with DNS tools and simulating resolution.

## Verification Mechanism
Parse zone file with dnspython, verify SOA/NS/A/CNAME/MX records resolve correctly.

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
[[network-configuration]], [[config-generation]]
