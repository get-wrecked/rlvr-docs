---
domain: Smart Contract Verification
category: Code & Software
verification_type: execution
dataset_scale: ~3M verified contracts on Etherscan + benchmarks
difficulty_range: easy/medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Smart Contract Verification

## Overview
Generate and verify Solidity/Vyper smart contracts. Unique constraints: gas optimization, reentrancy safety, EVM semantics, immutability after deployment. Covers writing new contracts, auditing existing ones, and fixing vulnerabilities.

## Verification Mechanism
1. Compile with solc/vyper compiler (must succeed)
2. Deploy to local EVM (Hardhat/Foundry/Anvil)
3. Execute test transactions, assert on-chain state changes
4. Static analysis with Slither/Mythril for known vulnerability patterns
5. Gas usage measurement and comparison

## Dataset Sources
- SmartBugs benchmark (143 annotated vulnerable contracts)
- SWC Registry (smart contract weakness classification)
- Ethernaut challenges (OpenZeppelin)
- Damn Vulnerable DeFi
- Etherscan verified source code (~3M contracts)
- Solidity-by-example

## Task Format
**Input**: Natural language specification or buggy contract code
**Output**: Solidity/Vyper source code

## Difficulty Curriculum
1. Simple storage contracts (set/get values)
2. ERC-20 token implementations
3. Multi-sig wallets, escrow contracts
4. DeFi protocols (AMMs, lending)
5. Gas-optimized implementations of complex protocols
6. Formal verification of security properties

## Limitations & Risks
- EVM changes across hard forks may invalidate older patterns
- Gas optimization can conflict with readability
- Real-world exploits often involve protocol-level logic bugs, not just code bugs

## Connections
[[code-generation]], [[formal-verification-software]], [[cybersecurity-defense]]
