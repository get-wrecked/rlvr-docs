---
domain: Firewall Rule Synthesis
category: Systems
verification_type: constraint
dataset_scale: unlimited (procedural)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Firewall Rule Synthesis

## Overview
Generate iptables/nftables rules satisfying access policies. Verify by simulating packet flows against rule set.

## Verification Mechanism
Simulate test packets through rule chain, verify accept/deny matches policy for each packet.

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
[[access-control-policy]], [[network-configuration]]
