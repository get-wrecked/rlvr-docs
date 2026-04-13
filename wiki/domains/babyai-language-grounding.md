---
domain: BabyAI (Language-Grounded Navigation)
category: Agent
verification_type: outcome
dataset_scale: 19 levels, unlimited episodes
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# BabyAI (Language-Grounded Navigation)

## Overview
Follow compositional language instructions in a grid world ('go to the red ball', 'pick up the blue key and open the green door'). Tests compositional generalization of language-to-action mapping. 19 difficulty levels from simple navigation to multi-step plans.

## Verification Mechanism
Task success (binary: did the agent achieve the goal described in the instruction?). Grid world state is fully observable and verifiable.

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
[[instruction-following]], [[planning-classical]]
