---
domain: compression-encoding
category: miscellaneous
verification_type: exact_match
dataset_scale: ~infinite (generate from any data)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Compression & Encoding

## Overview

Compression and encoding tasks require transforming data according to specified encoding schemes: Base64 encode/decode, hex encode/decode, URL encoding, Huffman coding, run-length encoding (RLE), LZ77/LZ78 concepts, and other data representation transformations. These are deterministic, well-defined operations with exact verification.

While individually simple, these tasks exercise precise procedural execution, attention to detail, and understanding of information representation — foundational skills for working with data systems.

## Verification Mechanism

**Type: Exact match**

```python
import base64, zlib, urllib.parse

def verify_encoding(task_type: str, input_data: str, model_output: str) -> float:
    if task_type == 'base64_encode':
        expected = base64.b64encode(input_data.encode()).decode()
    elif task_type == 'base64_decode':
        expected = base64.b64decode(input_data.encode()).decode()
    elif task_type == 'hex_encode':
        expected = input_data.encode().hex()
    elif task_type == 'hex_decode':
        expected = bytes.fromhex(input_data).decode()
    elif task_type == 'url_encode':
        expected = urllib.parse.quote(input_data)
    elif task_type == 'url_decode':
        expected = urllib.parse.unquote(input_data)
    elif task_type == 'rle_encode':
        expected = run_length_encode(input_data)
    elif task_type == 'rle_decode':
        expected = run_length_decode(input_data)
    # ... more types
    
    return 1.0 if model_output.strip() == expected.strip() else 0.0
```

**For Huffman coding:** Verify by checking:
1. The Huffman tree is valid (no code is a prefix of another).
2. Decoding the encoded output with the given tree produces the original data.
3. The encoding is optimal (minimum total bits for the given frequency distribution).

**Verification confidence: VERY HIGH.** All encoding operations are deterministic. Exact match is the correct verification method. The only edge case is whitespace/formatting in the output.

## Dataset Sources

- **Synthetic generation (primary):** Take any text or binary data, apply encoding. Unlimited scale.
- **Standard test vectors:** Base64, hex, URL encoding all have well-known test cases.
- **Huffman coding textbook examples:** Data structures textbooks contain worked examples.
- **Compression algorithm visualizations:** Educational sites with step-by-step examples.
- **CTF challenges (encoding):** Many CTF challenges involve multi-layer encoding chains.
- **Real-world data:** Encode excerpts from Wikipedia, Project Gutenberg, etc.

## Task Format

**Input (Base64):**
```
Encode the following string in Base64: "Hello, World!"
```

**Output:**
```
SGVsbG8sIFdvcmxkIQ==
```

**Input (RLE):**
```
Apply run-length encoding to: "AAABBBCCDDDDAA"
Format: count followed by character (e.g., "3A")
```

**Output:**
```
3A3B2C4D2A
```

**Input (Huffman):**
```
Given the following character frequencies, build a Huffman tree and encode "ABAC":
A: 4, B: 2, C: 1, D: 1

Provide the Huffman codes for each character and the encoded binary string.
```

**Output:**
```
Codes: A=0, B=10, C=110, D=111
Encoded: 0 10 0 110 = 010011​0
```

**Input (multi-layer):**
```
The following data has been encoded with these steps: 
1. UTF-8 text -> Hex encoded -> Base64 encoded
Decode it back to the original text:
"NTQ2NTczNzQyMDUzNzQ3MjY5NmU2Nw=="
```

## Difficulty Curriculum

1. **Easy:** Single-step encode/decode (Base64, hex, URL encoding). Short inputs.
2. **Medium:** Run-length encoding, multi-step encoding chains (hex -> base64), longer inputs, Huffman tree construction for small alphabets.
3. **Hard:** Huffman coding with optimization proof, LZ77 encoding (sliding window, match references), multi-layer encoding with unknown order (figure out the encoding stack).
4. **Very hard:** Arithmetic coding, Burrows-Wheeler transform, understanding compression ratios and theoretical limits (Shannon entropy).

## Limitations & Risks

- **Computational limits:** Base64 encoding of long strings requires mechanical character-by-character processing that LLMs are bad at. Keep inputs short enough for the model to process.
- **Low reasoning depth:** Most encoding tasks are mechanical procedure execution. Value is in precision, not insight.
- **Overlap with other domains:** Base64 and hex are also covered in [[encryption-decryption]]. Deduplicate or use as building blocks.
- **Not a standalone training domain:** Best used as a component within broader tasks (e.g., CTF challenges that require decoding as a step).

## Connections

- [[encryption-decryption]] — Encoding is a prerequisite skill for cryptographic operations.
- [[cryptography-challenges]] — Multi-layer encoding is common in CTF crypto challenges.
- [[data-formatting]] — Data representation transformations overlap conceptually.
- [[protocol-compliance]] — Many protocols use Base64, URL encoding, etc.
