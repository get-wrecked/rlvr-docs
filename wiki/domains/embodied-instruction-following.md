---
domain: Embodied Instruction Following (ALFRED/TEACh)
category: Agent
verification_type: outcome
dataset_scale: ~25K+ tasks from ALFRED/TEACh
difficulty_range: hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Embodied Instruction Following (ALFRED/TEACh)

## Overview
Follow natural language instructions in simulated household environments (pick up the mug, put it on the table, turn on the faucet). Requires vision + navigation + manipulation + language grounding. ALFRED and TEACh are the gold-standard benchmarks. This is a critical AGI capability — translating language to embodied action sequences.

## Verification Mechanism
Task success rate: did the agent achieve the goal state? Verified by checking simulator state (object positions, states like 'clean', 'hot', 'sliced'). Partial credit via goal condition completion fraction. AI2-THOR simulator provides deterministic state checking.

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
[[computer-use]], [[gui-navigation]], [[planning-classical]]
