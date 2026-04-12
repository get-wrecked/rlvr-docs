---
domain: ascii-art
category: creative-with-constraints
verification_type: constraint | diff
dataset_scale: ~medium (ASCII art databases + generation)
difficulty_range: easy/medium/hard
modality: text
status: needs_validation
---

# ASCII Art

## Overview

ASCII art tasks require generating visual representations using text characters. Tasks range from simple patterns (borders, boxes, geometric shapes) to complex illustrations (animals, objects, scenes). The domain sits uncomfortably between fully verifiable and subjective — some aspects (grid dimensions, specific character patterns, symmetry) are exactly checkable, while the core question "does this look like a cat?" is not programmatically verifiable.

**Honesty note: This domain has LIMITED verifiability. It is included because the verifiable subset (constrained character grids, geometric patterns, text banners) is useful, but complex artistic ASCII art cannot be reliably auto-verified.**

## Verification Mechanism

**Type: Constraint satisfaction (partial) + Diff (approximate)**

**What CAN be verified:**

```python
def verify_ascii_art(model_output: str, constraints: dict) -> float:
    lines = model_output.split('\n')
    score = 0.0
    checks = 0
    
    # Grid dimensions
    if 'height' in constraints:
        checks += 1
        score += 1.0 if len(lines) == constraints['height'] else 0.0
    if 'width' in constraints:
        checks += 1
        score += 1.0 if all(len(l) == constraints['width'] for l in lines) else 0.0
    
    # Character set restriction
    if 'allowed_chars' in constraints:
        checks += 1
        all_chars = set(''.join(lines))
        score += 1.0 if all_chars.issubset(set(constraints['allowed_chars'])) else 0.0
    
    # Symmetry
    if constraints.get('horizontal_symmetry'):
        checks += 1
        symmetric = all(l == l[::-1] for l in lines)
        score += 1.0 if symmetric else 0.0
    if constraints.get('vertical_symmetry'):
        checks += 1
        score += 1.0 if lines == lines[::-1] else 0.0
    
    # Specific characters at positions
    if 'required_positions' in constraints:
        for (r, c, char) in constraints['required_positions']:
            checks += 1
            score += 1.0 if lines[r][c] == char else 0.0
    
    # Border check
    if constraints.get('has_border'):
        checks += 1
        border_char = constraints.get('border_char', '#')
        top_ok = all(c == border_char for c in lines[0])
        bot_ok = all(c == border_char for c in lines[-1])
        sides_ok = all(l[0] == border_char and l[-1] == border_char for l in lines)
        score += 1.0 if (top_ok and bot_ok and sides_ok) else 0.0
    
    return score / checks if checks > 0 else 0.0
```

**What CANNOT be reliably verified:**
- Does the output look like a "cat"? (Would require vision model / LLM-as-judge)
- Is the art aesthetically good? (Subjective)
- Does it match a reference image? (Approximate pixel-level comparison after rendering to monospace grid is possible but unreliable for complex art)

**Approximate approach for complex art:** Render both reference and model output as monospace text images, then compute SSIM. This works for exact-reproduction tasks but not for "draw X" tasks.

**Verification confidence: HIGH for constrained geometric tasks, LOW for creative/representational art.** This is a split domain — the verifiable portion (patterns, banners, grids) is useful; the creative portion is not suitable for RLVR.

## Dataset Sources

- **ASCII Art Archive (asciiart.eu):** Thousands of categorized ASCII art pieces. Can be used for reproduction tasks.
- **FIGlet / toilet:** Text-to-banner tools that produce ASCII text banners in various fonts. Can generate unlimited (input, output) pairs.
- **Cowsay:** Simple framed text art with animals. Known output format.
- **Box-drawing character patterns:** Unicode box-drawing characters for tables and borders.
- **Geometric pattern generators:** Programmatically generate patterns (checkerboards, spirals, fractals) as ASCII.
- **Code golf ASCII art challenges:** Many code golf sites have ASCII art reproduction challenges with exact expected outputs.
- **Synthetic generation:** Define geometric shapes (rectangles, triangles, diamonds, circles) with character parameters. Unlimited scale for the constrained subset.

## Task Format

**Input (geometric, verifiable):**
```
Draw a 7x7 diamond using '*' characters on a '.' background:
```

**Output:**
```
...*...
..***..
.*****. 
*******
.*****.
..***..
...*...
```

**Input (text banner, verifiable):**
```
Generate "HELLO" in block letters, each letter 5 characters tall and 5 characters wide, 
using '#' for filled and ' ' for empty.
```

**Input (creative, LIMITED verifiability):**
```
Draw an ASCII art cat, approximately 10 lines tall.
```

**Output:**
```
  /\_/\  
 ( o.o ) 
  > ^ <  
 /|   |\ 
(_|   |_)
```
(Verifiable: height ~10 lines. Not verifiable: "looks like a cat.")

## Difficulty Curriculum

1. **Easy:** Rectangles, borders, simple patterns (checkerboards), FIGlet text banners. Fully verifiable.
2. **Medium:** Diamonds, triangles, circles (approximate), symmetric patterns, table/grid layouts. Mostly verifiable.
3. **Hard:** Simple representational art (houses, trees, simple objects) where a reference output exists for exact comparison. Partially verifiable.
4. **Very hard:** Complex representational art (animals, faces, scenes) from description. **Not reliably verifiable — outside RLVR scope for creative tasks.**

## Limitations & Risks

- **CRITICAL: Limited verifiability for creative tasks.** This is the fundamental limitation. ASCII art that "looks like X" cannot be verified without a vision model or human judge, which is outside the RLVR paradigm. The domain is viable only for the constrained/geometric subset.
- **Reference-dependent:** Reproduction tasks (copy this exact ASCII art) are verifiable but test copying, not creation. Original generation from descriptions is the interesting task but has weak verification.
- **Monospace dependency:** ASCII art assumes monospace font rendering. Different terminal fonts/sizes may display differently.
- **Character ambiguity:** Similar-looking characters (l vs 1 vs |, O vs 0) cause verification edge cases.

## Connections

- [[svg-generation]] — Both produce visual output; SVG has better verifiability.
- [[constrained-writing]] — ASCII art with constraints (dimensions, symmetry, character set) is a form of constrained text generation.
- [[html-css-generation]] — Visual output generation, but HTML has stronger verification tools.
