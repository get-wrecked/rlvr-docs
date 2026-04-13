---
domain: Tool-Augmented Question Answering
category: Agent
verification_type: exact_match
dataset_scale: ~10K+ from ToolBench/API-Bank
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Tool-Augmented Question Answering

## Overview
Answer questions by selecting and calling the right APIs from a large tool library (1000+ APIs). ToolBench provides real-world API scenarios. Requires planning which tools to use, in what order, with what parameters.

## Verification Mechanism
Verify API call sequence produces correct final answer. Check each API call has valid parameters. Compare answer to gold standard.

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
[[natural-language-to-api]], [[api-usage]]
