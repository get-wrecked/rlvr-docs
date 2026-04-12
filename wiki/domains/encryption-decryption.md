---
domain: encryption-decryption
category: miscellaneous
verification_type: exact_match
dataset_scale: ~infinite (generate from any plaintext)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Encryption / Decryption

## Overview

Encryption/decryption tasks require correctly applying specified cryptographic algorithms to transform plaintext to ciphertext (or vice versa). Unlike [[cryptography-challenges]] (which focuses on *breaking* crypto), this domain tests correct *application* of algorithms: "Encrypt this message using AES-256-CBC with this key and IV" or "Decrypt this RSA ciphertext given the private key."

Verification is exact: encrypt/decrypt with a reference implementation and compare. The domain exercises procedural algorithm execution, precise attention to parameters, and understanding of cryptographic modes and padding.

## Verification Mechanism

**Type: Exact match**

```python
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives import padding
import base64

def verify_encryption(model_output: str, plaintext: bytes, key: bytes, 
                       iv: bytes, algorithm: str) -> float:
    # Compute expected ciphertext
    if algorithm == 'AES-CBC':
        cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
        encryptor = cipher.encryptor()
        padder = padding.PKCS7(128).padder()
        padded = padder.update(plaintext) + padder.finalize()
        expected = base64.b64encode(encryptor.update(padded) + encryptor.finalize()).decode()
    # ... other algorithms
    
    return 1.0 if model_output.strip() == expected else 0.0
```

For decryption tasks: compare model's plaintext output against known plaintext.

**Supported algorithms for training:**
- Symmetric: Caesar cipher, Vigenere, XOR, AES (ECB, CBC, CTR), DES/3DES, ChaCha20
- Asymmetric: RSA (small key sizes for mental math), Diffie-Hellman key computation
- Hashing: MD5, SHA-256 (given input, produce hash — or identify hash type)
- Encoding: Base64, Base32, hex encoding (technically encoding, not encryption)

**Verification confidence: VERY HIGH.** Cryptographic operations are deterministic given identical inputs (plaintext, key, IV, mode, padding). Exact match is appropriate.

## Dataset Sources

- **Synthetic generation (primary):** Take any plaintext, select an algorithm and random key, compute the ciphertext. Unlimited scale.
- **Cryptopals challenges (sets 1-2):** Correct-implementation challenges (as opposed to attack challenges).
- **NIST test vectors:** Official test vectors for AES, SHA, and other NIST standards. Authoritative ground truth.
- **RFC test vectors:** RFCs for cryptographic protocols include test vectors (e.g., RFC 3602 for AES-CBC test vectors).
- **CTF archives (easy challenges):** Many CTF challenges test correct decryption rather than cryptanalysis.
- **Textbook exercises:** Cryptography textbooks (Stallings, Menezes) contain worked examples.

## Task Format

**Input (classical):**
```
Encrypt the message "HELLO WORLD" using a Vigenere cipher with key "SECRET".
```

**Output:**
```
ZINCS PGVNU
```

**Input (modern):**
```
Decrypt the following AES-128-ECB ciphertext.
Key (hex): 2b7e151628aed2a6abf7158809cf4f3c
Ciphertext (hex): 3ad77bb40d7a3660a89ecaf32466ef97
Provide the plaintext in hex.
```

**Output:**
```
6bc1bee22e409f96e93d7e117393172a
```

**Input (encoding):**
```
Decode the following Base64 string: SGVsbG8gV29ybGQh
```

**Output:**
```
Hello World!
```

## Difficulty Curriculum

1. **Easy:** Caesar cipher (shift by N), ROT13, Base64 encode/decode, hex encode/decode.
2. **Medium:** Vigenere cipher, XOR with known key, simple RSA (small numbers: n=33, e=3, decrypt c=27).
3. **Hard:** AES encryption/decryption with specified mode and padding, multi-step operations (encrypt then base64 then hex), hash computation for specific inputs.
4. **Very hard:** Full RSA key generation from primes, Diffie-Hellman shared secret computation, HMAC computation, understanding and applying padding schemes (PKCS7, OAEP).

Note: For modern algorithms with large key sizes, the model cannot realistically compute AES rounds mentally. Tasks should either use small examples where mental computation is feasible, or test understanding of the process (e.g., "What is the output of the first AES round given...").

## Limitations & Risks

- **Computational limits of LLMs:** An LLM cannot execute 10 rounds of AES on 128-bit blocks mentally. Practical tasks for modern algorithms must be restricted to small examples, parameter identification, or high-level process description.
- **Security sensitivity:** Training a model on encryption/decryption details is benign (this is public textbook knowledge), but care should be taken not to train on operational security data.
- **Padding and encoding details:** Small differences in padding (PKCS7 vs. zero-padding) or encoding (base64 with/without newlines) cause exact-match failures. Must be explicit about all parameters.
- **Overlap with cryptography-challenges:** The boundary between "correctly apply algorithm" and "break weak crypto" is blurry. This domain focuses on the former; [[cryptography-challenges]] on the latter.

## Connections

- [[cryptography-challenges]] — Complementary domain: this = apply algorithms correctly, that = break/attack algorithms.
- [[compression-encoding]] — Encoding operations (base64, hex) are shared.
- [[math-competition]] — Number theory (modular arithmetic, primes) underlies public-key crypto.
- [[protocol-compliance]] — Cryptographic protocols have specific parameter requirements.
