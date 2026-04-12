---
domain: Error-Correcting Code Design
category: coding-theory
verification_type: execution
dataset_scale: 10K+ (from coding theory benchmarks)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Error-Correcting Code Design

## Overview
Design error-correcting codes (linear codes, LDPC, turbo, polar, Reed-Solomon) that achieve specified rate and distance properties. Verification: compute minimum distance, check generator/parity-check matrix properties, simulate bit error rate.

## Verification Mechanism
```python
import numpy as np
from galois import GF2

def verify(specs: dict, code: dict) -> float:
    G = np.array(code["generator_matrix"])
    n, k = G.shape[1], G.shape[0]  # [n, k] code
    
    score = 0.0
    checks = 0
    
    # Check rate
    checks += 1
    actual_rate = k / n
    if abs(actual_rate - specs.get("rate", k/n)) < 0.01:
        score += 1
    
    # Check minimum distance
    checks += 1
    H = compute_parity_check(G)
    d_min = compute_min_distance(G)
    if d_min >= specs.get("min_distance", 1):
        score += 1
    
    # Verify G*H^T = 0 (parity check)
    if "parity_check_matrix" in code:
        checks += 1
        H = np.array(code["parity_check_matrix"])
        product = (G @ H.T) % 2
        if np.all(product == 0):
            score += 1
    
    # Singleton bound check: d <= n - k + 1
    checks += 1
    if d_min <= n - k + 1:
        score += 1
    
    # BER simulation (if target SNR specified)
    if "target_ber" in specs:
        checks += 1
        ber = simulate_ber(G, H, snr=specs["target_snr"], n_trials=10000)
        if ber <= specs["target_ber"]:
            score += 1
    
    return score / checks
```

## Dataset Sources
- **MinT database**: Tables of best known codes (minimum distance for given n, k).
- **Magma code tables**: Algebraic coding theory computations.
- **IEEE channel coding benchmarks**: Standard evaluation.
- **LDPC code collections**: Published LDPC matrices (802.11, 5G NR).
- **Polar code benchmarks**: From 5G standardization.
- **Textbook exercises**: Lin & Costello, Blahut.

## Task Format
- **Input**: "Design a binary linear [15, 7] code with minimum distance at least 5"
- **Output**: Generator matrix G (7×15 binary matrix)

## Difficulty Curriculum
- Level 1: Simple repetition and parity codes
- Level 3: Hamming codes, Reed-Muller codes
- Level 5: BCH codes, Reed-Solomon codes
- Level 7: LDPC design, turbo code design
- Level 9: Capacity-approaching code design, polar code construction

## Connections
- [[information-theory]] — theoretical bounds (Shannon limit)
- [[linear-algebra-computation]] — matrix operations over GF(2)
- [[combinatorics-optimization]] — code design as optimization
