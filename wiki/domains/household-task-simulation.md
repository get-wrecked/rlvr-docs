---
domain: Household Task Simulation (BEHAVIOR/VirtualHome)
category: Agent
verification_type: outcome
dataset_scale: ~1K+ tasks from BEHAVIOR-1K
difficulty_range: hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Household Task Simulation (BEHAVIOR/VirtualHome)

## Overview
Complete complex household activities (cook breakfast, clean bathroom, organize closet) in physics-realistic simulation. BEHAVIOR-1K defines 1,000 activities with logical+physical constraints. Requires long-horizon planning over 50-100+ steps with physical reasoning.

## Verification Mechanism
Task completion verified by checking BDDL (Behavior Domain Definition Language) goal conditions against simulator state. Every object must be in the correct state and location.

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
[[embodied-instruction-following]], [[planning-classical]], [[robotics-planning]]
