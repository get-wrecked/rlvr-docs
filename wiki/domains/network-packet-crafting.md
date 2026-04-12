---
domain: Network Packet Crafting
category: Security
verification_type: execution
dataset_scale: unlimited (from protocol specs)
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Network Packet Crafting

## Overview
Construct network packets matching protocol specifications (TCP, DNS, HTTP). Verify via packet parser (Scapy).

## Verification Mechanism
Parse crafted packet with Scapy/Wireshark dissector, verify all fields match specification.

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
[[protocol-compliance]], [[network-configuration]]
