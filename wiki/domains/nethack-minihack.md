---
domain: NetHack / MiniHack (Roguelike Game)
category: Games
verification_type: outcome
dataset_scale: unlimited (procedural)
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# NetHack / MiniHack (Roguelike Game)

## Overview
Play NetHack, the notoriously difficult roguelike. Requires long-horizon planning, resource management, combat tactics, inventory management, and adaptation to procedurally generated dungeons. NeurIPS competition benchmark. MiniHack provides targeted sub-tasks.

## Verification Mechanism
Score, dungeon level reached, and ascension status. The game provides deterministic reward signals. MiniHack tasks have specific goal conditions (reach location, collect item, defeat monster).

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
[[interactive-fiction]], [[planning-classical]], [[game-playing-atari]]
