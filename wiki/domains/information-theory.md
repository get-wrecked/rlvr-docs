---
domain: Information Theory Computation
category: math-applied
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Information Theory Computation

## Overview
Compute information-theoretic quantities: entropy, mutual information, channel capacity, rate-distortion, Huffman codes, Lempel-Ziv, error-correcting codes. Verification: direct computation.

## Verification Mechanism
```python
import numpy as np
from scipy.stats import entropy as scipy_entropy

def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "entropy":
        correct = scipy_entropy(problem["distribution"], base=2)
        return 1.0 if abs(answer - correct) < 0.001 else 0.0
    
    elif task_type == "mutual_information":
        joint = np.array(problem["joint_distribution"])
        px = joint.sum(axis=1)
        py = joint.sum(axis=0)
        mi = scipy_entropy(px, base=2) + scipy_entropy(py, base=2) - scipy_entropy(joint.flatten(), base=2)
        return 1.0 if abs(answer - mi) < 0.001 else 0.0
    
    elif task_type == "huffman_code":
        # Verify Huffman code is prefix-free and optimal
        code = answer["codebook"]
        # Check prefix-free property
        codes = list(code.values())
        for i, c1 in enumerate(codes):
            for c2 in codes[i+1:]:
                if c1.startswith(c2) or c2.startswith(c1):
                    return 0.0  # Not prefix-free
        
        # Check optimality (average code length)
        avg_length = sum(problem["probs"][s] * len(code[s]) for s in code)
        H = scipy_entropy(list(problem["probs"].values()), base=2)
        if avg_length < H:
            return 0.0  # Impossible — shorter than entropy
        if avg_length <= H + 1:
            return 1.0  # Within theoretical bound
        return 0.5  # Valid but not optimal
    
    elif task_type == "channel_capacity":
        # For simple channels (BSC, BEC), verify against known formulas
        correct = compute_channel_capacity(problem["channel_type"], problem["params"])
        return 1.0 if abs(answer - correct) < 0.01 else 0.0
    
    elif task_type == "error_correcting_code":
        # Verify code properties (minimum distance, rate)
        code_matrix = np.array(answer["generator_matrix"])
        min_dist = compute_minimum_distance(code_matrix)
        if min_dist >= problem["required_distance"]:
            return 1.0
        return 0.0
```

## Dataset Sources
- **Information theory textbooks**: Cover & Thomas — exercises with solutions.
- **Error-correcting code databases**: MinT (tables of known best codes).
- **Compression benchmarks**: Canterbury corpus, Silesia corpus.
- **Channel coding benchmarks**: Standard channel models.
- **Procedural generation**: Generate random distributions, compute information-theoretic quantities. Unlimited.

## Task Format
- **Input**: "Compute the entropy of X where P(X=a)=0.5, P(X=b)=0.25, P(X=c)=0.25"
- **Output**: "H(X) = 1.5 bits"

## Difficulty Curriculum
- Level 1: Entropy of simple distributions
- Level 3: Huffman coding, mutual information
- Level 5: Channel capacity (BSC, BEC)
- Level 7: Rate-distortion, source coding theorems
- Level 9: Network information theory, multi-user channels

## Limitations & Risks
- Computation is exact for discrete distributions. Continuous distributions require numerical integration.
- Error-correcting code design is combinatorial — verification is easier than construction.

## Connections
- [[probability-statistics]] — probability theory
- [[compression-encoding]] — practical compression
- [[signal-processing]] — digital communication
- [[cryptography-challenges]] — information-theoretic security
