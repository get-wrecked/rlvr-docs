---
domain: Mathematical Cryptanalysis
category: cryptography-math
verification_type: execution
dataset_scale: 10K+ (from CTF crypto + textbooks)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Mathematical Cryptanalysis

## Overview
Break cryptographic systems using mathematical techniques: factor RSA moduli, solve discrete logarithms, exploit weak parameters, perform frequency analysis on classical ciphers, exploit padding oracles. Verification: the decrypted plaintext matches the original (exact match).

## Verification Mechanism
```python
def verify(task_type: str, ciphertext: bytes, solution: dict, ground_truth: dict) -> float:
    if task_type == "decrypt":
        return 1.0 if solution["plaintext"] == ground_truth["plaintext"] else 0.0
    
    elif task_type == "factor":
        n = ground_truth["n"]
        p, q = solution["p"], solution["q"]
        return 1.0 if p * q == n and is_prime(p) and is_prime(q) else 0.0
    
    elif task_type == "discrete_log":
        g, h, p = ground_truth["g"], ground_truth["h"], ground_truth["p"]
        x = solution["x"]
        return 1.0 if pow(g, x, p) == h else 0.0
    
    elif task_type == "find_key":
        key = solution["key"]
        plaintext = decrypt(ground_truth["ciphertext"], key, ground_truth["algorithm"])
        return 1.0 if plaintext == ground_truth["plaintext"] else 0.0
```

## Dataset Sources
- **CryptoHack**: 200+ crypto challenges with flags.
- **CTF crypto challenges**: From CTFtime archives.
- **CryptoPals**: 64 crypto challenges (Matasano).
- **RSA Factoring Challenge**: Historical instances (some factored).
- **Weak key databases**: Collections of known weak crypto parameters.
- **Textbook exercises**: Katz & Lindell, Stinson — with solutions.

## Task Format
- **Input**: "This RSA ciphertext was encrypted with n=3233, e=17. Decrypt it."
- **Output**: "p=61, q=53, d=2753, plaintext='HELLO'"

## Difficulty Curriculum
- Level 1: Caesar cipher, simple substitution, base64 encoding
- Level 3: Vigenère cipher, XOR with short key
- Level 5: RSA with small modulus, weak PRNG exploitation
- Level 7: Padding oracle attacks, elliptic curve weaknesses
- Level 9: Novel cryptanalytic attacks on custom schemes

## Limitations & Risks
- Higher difficulty requires significant computation (factoring large numbers). Keep instances tractable.
- This is for educational/defensive cryptanalysis, not offensive capability.
- Verification is trivial: either you recovered the plaintext or you didn't.

## Connections
- [[cryptography-challenges]] — general crypto challenges
- [[cybersecurity-ctf]] — CTF context
- [[number-theory-computation]] — mathematical foundations
- [[abstract-algebra]] — group theory in cryptography
