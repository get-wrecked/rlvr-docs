---
domain: MEV Strategy Design (DeFi)
category: Expert
verification_type: execution
dataset_scale: unlimited (from blockchain data)
difficulty_range: hard/superhuman
modality: code
status: strong_hypothesis
---

# MEV Strategy Design (DeFi)

## Overview
Design Maximal Extractable Value strategies (arbitrage, liquidation, sandwich) on Ethereum. Verify profitability by simulating against historical mempool data using Foundry's fork mode.

## Verification Mechanism
Fork mainnet at historical block, simulate strategy execution, verify profit > gas cost. Fully deterministic replay.

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
[[smart-contract-verification]], [[financial-calculation]]
