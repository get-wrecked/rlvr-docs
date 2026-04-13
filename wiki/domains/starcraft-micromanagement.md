---
domain: StarCraft II Micromanagement
category: Games
verification_type: outcome
dataset_scale: unlimited (procedural scenarios)
difficulty_range: hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# StarCraft II Micromanagement

## Overview
Control units in StarCraft II combat scenarios (micro). AlphaStar showed this requires real-time decision-making, spatial reasoning, and unit-type knowledge. Micro-specific scenarios are more tractable than full game and fully verifiable.

## Verification Mechanism
Battle outcome (units remaining, damage dealt). PySC2 provides exact unit state. Win/loss is deterministic given unit positions and actions.

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
[[multi-agent-coordination]], [[planning-classical]]
