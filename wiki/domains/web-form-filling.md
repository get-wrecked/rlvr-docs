---
domain: Web Form Filling
category: Agent
verification_type: outcome
dataset_scale: ~1K+ from MiniWoB++
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Web Form Filling

## Overview
Fill web forms correctly given natural language instructions. Verify form submission results match expected.

## Verification Mechanism
Submit form in headless browser, verify submitted values match specification.

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
[[web-navigation]], [[gui-navigation]], [[instruction-following]]
