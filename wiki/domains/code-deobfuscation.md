---
domain: Code Deobfuscation
category: Code
verification_type: execution
dataset_scale: ~5K+ from CTF challenges + obfuscators
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Code Deobfuscation

## Overview
Deobfuscate code to recover readable equivalent. Verify functional equivalence via test suite execution.

## Verification Mechanism
Run both obfuscated and deobfuscated versions on test inputs, verify identical outputs.

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
[[reverse-engineering]], [[code-refactoring]]
