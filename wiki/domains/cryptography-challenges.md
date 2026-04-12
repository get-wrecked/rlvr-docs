---
domain: cryptography-challenges
category: miscellaneous
verification_type: exact_match
dataset_scale: ~large (CTF archives + generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Cryptography Challenges

## Overview

Cryptography challenges task the model with breaking ciphers, exploiting cryptographic vulnerabilities, or solving crypto-related puzzles. These come primarily from Capture The Flag (CTF) competitions, where the answer is a specific flag string, making verification trivial and exact. The domain ranges from classical cipher decryption (Caesar, Vigenere, substitution) to modern cryptographic attacks (RSA with small exponents, padding oracle, elliptic curve weaknesses).

This is a high-value RLVR domain because (1) verification is perfect (flag matches or doesn't), (2) it exercises mathematical reasoning, pattern recognition, and multi-step problem solving, and (3) difficulty scales smoothly from trivial to research-level.

## Verification Mechanism

**Type: Exact match**

The verification is simple: compare the model's output to the known flag or plaintext.

```python
def verify(model_output: str, expected_flag: str) -> float:
    # Strip whitespace, normalize
    cleaned = model_output.strip()
    if cleaned == expected_flag:
        return 1.0
    # Partial credit: check if flag appears anywhere in output
    if expected_flag in cleaned:
        return 0.9
    return 0.0
```

For non-CTF crypto tasks (e.g., "decrypt this ciphertext"), verification compares against the known plaintext.

For tasks requiring the model to identify a vulnerability or attack type, verification checks against a known label (e.g., "padding oracle attack").

**Verification confidence: VERY HIGH.** Flag-based verification is perfect. Plaintext comparison is exact. No ambiguity.

## Dataset Sources

- **CTFtime.org:** Archive of thousands of CTF competitions. Crypto challenges with writeups (containing flags) are widely available.
- **CryptoHack (cryptohack.org):** Structured crypto challenges from basic to advanced, with known solutions.
- **PicoCTF:** Beginner-friendly CTF with extensive crypto section. All flags known.
- **OverTheWire (Krypton):** Progressive cryptography wargame.
- **Cryptopals (cryptopals.com):** 8 sets of 48 challenges covering classical to modern attacks. All solutions publicly available.
- **Project Euler:** Several problems involve number theory/cryptography.
- **Synthetic generation for classical ciphers:** Encrypt random plaintext with Caesar/Vigenere/substitution ciphers. Unlimited scale.
- **RSA challenge generation:** Generate weak RSA instances (small e, close p and q, shared factors) programmatically.

## Task Format

**Input (easy):**
```
The following message was encrypted with a Caesar cipher. Decrypt it:
"Wkh txlfn eurzq ira mxpsv ryhu wkh odcb grj"
```

**Output:**
```
The quick brown fox jumps over the lazy dog
```

**Input (hard):**
```
An RSA public key has n = 323, e = 5. The ciphertext is 34.
Find the plaintext (as an integer).
```

**Output:**
```
17
```

**Input (CTF-style):**
```
You intercepted the following encrypted message and the Python script used to encrypt it:
[script showing XOR with repeating key]
[hex-encoded ciphertext]
Find the flag in format flag{...}
```

**Output:**
```
flag{x0r_1s_n0t_encrypt10n}
```

## Difficulty Curriculum

1. **Easy:** Caesar cipher (brute force 25 shifts), ROT13, base64 decode, simple XOR with single byte.
2. **Medium:** Vigenere cipher (frequency analysis), substitution cipher, multi-step encoding chains (base64 -> hex -> XOR), RSA with trivially small n.
3. **Hard:** RSA attacks (Wiener's, Hastad's broadcast, common modulus), block cipher mode attacks (ECB penguin, CBC bit flipping), Diffie-Hellman with small subgroup.
4. **Very hard:** Padding oracle attacks, elliptic curve crypto with weak curves, lattice-based attacks (LLL on knapsack), side-channel reasoning.
5. **Superhuman:** Novel cryptanalytic attacks, zero-knowledge proof construction, advanced number theory exploitation.

## Limitations & Risks

- **Security concerns:** Training a model to break cryptography raises dual-use concerns. However, CTF challenges test known, well-documented attacks — not novel cryptanalysis. The model learns to apply textbook attacks, not discover new ones.
- **Flag format brittleness:** Minor formatting differences (whitespace, case) can cause false negatives. Mitigation: normalize comparison, accept flag with or without wrapper format.
- **Computational requirements:** Some attacks require extensive computation (factoring, brute force) that an LLM cannot do internally. The model must reason about the attack strategy rather than execute it numerically. For training, prefer challenges solvable by reasoning.
- **Writeup availability bias:** Easy CTF challenges have more writeups than hard ones, creating dataset imbalance.
- **Code execution may be needed:** Many crypto challenges are best solved by writing and running code. If the RLVR setup includes a code sandbox, this becomes a code-generation task with crypto domain knowledge.

## Connections

- [[encryption-decryption]] — Overlapping domain; encryption-decryption focuses on correct algorithm application, this on breaking/attacking.
- [[compression-encoding]] — Encoding/decoding skills are prerequisite for many crypto challenges.
- [[code-generation]] — Many CTF crypto challenges are best solved by writing exploit scripts.
- [[math-competition]] — Number theory skills directly applicable to RSA and other public-key attacks.
