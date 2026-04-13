---
domain: Rule Following Under Adversarial Pressure
category: Agent
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Rule Following Under Adversarial Pressure

## Overview
Follow specified rules (format constraints, content restrictions, behavioral guidelines) even when user prompts try to trick the agent into breaking them. Tests robustness of instruction following.

## Verification Mechanism
Check all rules satisfied in output (word count, forbidden topics, format requirements) even when prompt contains adversarial jailbreak-style instructions. Fully automated constraint checking.

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
[[instruction-following]], [[constrained-writing]]
