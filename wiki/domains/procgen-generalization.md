---
domain: Procgen (Procedural Game Generalization)
category: Games
verification_type: outcome
dataset_scale: 16 games × unlimited levels
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Procgen (Procedural Game Generalization)

## Overview
Play 16 procedurally generated games designed to test generalization. Training levels differ from test levels. OpenAI benchmark for testing whether agents learn general game-playing skills vs. memorizing specific levels.

## Verification Mechanism
Episodic return (score) averaged across held-out test levels. Fully automated by game environments.

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
[[game-playing-atari]], [[abstract-pattern-completion]]
