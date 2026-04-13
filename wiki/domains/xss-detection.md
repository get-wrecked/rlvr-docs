---
domain: XSS Detection/Prevention
category: Security
verification_type: execution
dataset_scale: ~5K+ from OWASP/XSS datasets
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# XSS Detection/Prevention

## Overview
Identify Cross-Site Scripting vulnerabilities or implement proper sanitization. Verify via browser sandbox.

## Verification Mechanism
Inject XSS payload into sandboxed browser context, verify script execution or proper sanitization.

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
[[code-vulnerability-detection]], [[html-css-generation]]
