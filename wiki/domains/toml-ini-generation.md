---
domain: TOML/INI Configuration Generation
category: Code
verification_type: execution
dataset_scale: ~50K+ config files on GitHub
difficulty_range: easy/medium
modality: code
status: strong_hypothesis
---

# TOML/INI Configuration Generation

## Overview
Generate TOML/INI configuration files matching specifications. Verify by parsing and checking values.

## Verification Mechanism
Parse with toml library, verify all required keys present with correct types and values.

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
[[config-generation]], [[build-configuration]]
