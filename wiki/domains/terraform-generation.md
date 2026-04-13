---
domain: Terraform / IaC Generation
category: Code
verification_type: execution
dataset_scale: ~100K+ Terraform configs on GitHub
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Terraform / IaC Generation

## Overview
Generate Terraform configurations. Verify via terraform validate, plan, and policy checks (OPA/Sentinel).

## Verification Mechanism
terraform init + validate passes. terraform plan produces expected resource graph. Policy checks pass.

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
[[infrastructure-as-code]], [[config-generation]]
