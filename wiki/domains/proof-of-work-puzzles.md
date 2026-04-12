---
domain: Proof-of-Work / Hash Puzzles
category: cryptography
verification_type: exact_match
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Proof-of-Work / Hash Puzzles

## Overview
Given a challenge (partial hash preimage, nonce finding, hash collision constraints), find an input that satisfies the hash condition. These are the same puzzles used in cryptocurrency mining but repurposed as reasoning challenges. The key insight: finding the input requires logical reasoning about string/number properties, but verification is instant (just compute the hash).

## Verification Mechanism
```python
import hashlib

def verify(challenge: dict, solution: str) -> float:
    h = hashlib.sha256(solution.encode()).hexdigest()
    if challenge["type"] == "prefix":
        return 1.0 if h.startswith(challenge["prefix"]) else 0.0
    elif challenge["type"] == "contains":
        return 1.0 if challenge["substring"] in h else 0.0
    elif challenge["type"] == "partial_preimage":
        return 1.0 if h == challenge["target_hash"] else 0.0
```

Verification is O(1) — just one hash computation.

## Dataset Sources
- Procedurally generated: unlimited
- CTF crypto challenge archives (CTFtime.org)
- Hashcat benchmark datasets

## Task Format
- **Input**: "Find a string whose SHA-256 hash starts with '0000'"
- **Output**: Any valid string (e.g., "nonce_42857")
- Difficulty scales with prefix length

## Difficulty Curriculum
- Level 1: Find string with specific last hex digit (1/16 chance random)
- Level 5: Find string with 4-character prefix (1/65536)
- Level 10: Find string with constraints on BOTH input and hash

## Limitations & Risks
- At high difficulty, this becomes brute-force search — not clear it teaches useful reasoning vs. just pattern matching. Best used at low-to-medium difficulty where reasoning about hash properties helps.
- The agent can't actually "reason" about hash outputs (they're pseudorandom). This tests heuristic search strategies more than logical reasoning.

## Connections
- [[cryptography-challenges]] — broader crypto challenges
- [[combinatorics-optimization]] — search in large spaces
