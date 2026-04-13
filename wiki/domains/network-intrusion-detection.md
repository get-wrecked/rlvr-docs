---
domain: Network Intrusion Detection
category: Security
verification_type: exact_match
dataset_scale: ~500K+ from CICIDS/NSL-KDD
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Network Intrusion Detection

## Overview
Classify network traffic as benign or specific attack types. Verify against labeled pcap datasets.

## Verification Mechanism
Compare predicted labels against gold-standard traffic classifications. Compute detection rate / false positive rate.

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
[[cybersecurity-defense]], [[log-anomaly-detection]]
