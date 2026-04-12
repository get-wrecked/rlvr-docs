---
domain: svg-generation
category: format-constrained
verification_type: execution | diff
dataset_scale: ~medium-large (synthetic + icon datasets)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# SVG Generation

## Overview

SVG generation tasks require producing Scalable Vector Graphics markup that renders to match a visual description or reference image. SVG is a text-based vector graphics format, making it uniquely suited for LLM generation — the model outputs XML-like markup that directly defines visual elements (shapes, paths, text, gradients, transformations).

This domain is valuable for RLVR because (1) SVG validity is checkable (XML parsing), (2) visual output is deterministic and measurable, and (3) it exercises spatial reasoning, geometric understanding, and structured output generation in a creative context.

## Verification Mechanism

**Type: Execution (rendering) + Diff (visual similarity)**

**Stage 1 — Validity:**
```python
from xml.etree import ElementTree as ET

def check_valid_svg(svg_string: str) -> bool:
    try:
        root = ET.fromstring(svg_string)
        return root.tag == '{http://www.w3.org/2000/svg}svg' or root.tag == 'svg'
    except ET.ParseError:
        return False
```

**Stage 2 — Rendering:**
Render SVG to PNG using CairoSVG, librsvg, or headless Chromium:
```python
import cairosvg
cairosvg.svg2png(bytestring=svg_bytes, write_to='output.png', 
                 output_width=256, output_height=256)
```

**Stage 3 — Visual comparison:**
- **SSIM:** Structural similarity between rendered output and reference image. Good baseline.
- **LPIPS:** Perceptual similarity using deep features. Better for complex images.
- **CLIP similarity:** Encode both images with CLIP, compute cosine similarity. Useful when comparing to text descriptions rather than reference images.
- **IoU for simple shapes:** For tasks like "draw a red circle", detect the shape and compute intersection-over-union with the expected shape.

**For description-to-SVG tasks (no reference image):**
Use CLIP score between the rendered SVG and the text description. This is softer verification but still automated and reproducible.

**Reward structure:**
- Valid SVG that parses: 0.2
- Renders to non-empty image: 0.1
- Visual similarity > 0.6: 0.3
- Visual similarity > 0.8: additional 0.2
- Visual similarity > 0.9: additional 0.2

**Verification confidence: MEDIUM.** SVG validity is exact. Visual similarity is well-defined for reference-image tasks. For description-to-image tasks, CLIP-based verification is approximate and can be gamed by adversarial patterns. Overall less precise than text-based verification but better than LLM-as-judge.

## Dataset Sources

- **SVG icon datasets:** SVG-repo (500k+ free SVGs), Flaticon, Iconoir, Feather Icons — with descriptions/labels.
- **Simple Shapes dataset:** Generate synthetic datasets of basic shapes (circles, rectangles, triangles) with precise specifications.
- **Logo datasets:** SVG logos from brands (many openly available in SVG format).
- **Data visualization:** Generate SVG charts (bar, line, pie) from data — verification by checking rendered output matches expected visualization.
- **Geometric construction:** Tasks like "draw an equilateral triangle inscribed in a circle of radius 100" — verifiable by checking coordinates.
- **Template generation:** "Draw a {color} {shape} at position ({x},{y}) with {property}" — unlimited generation.
- **MapSVG and cartographic data:** Map outlines and geographic shapes in SVG format.

## Task Format

**Input (description):**
```
Generate an SVG image (200x200 viewport) of a red circle centered at (100,100) 
with radius 50, on a white background.
```

**Output:**
```xml
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <rect width="200" height="200" fill="white"/>
  <circle cx="100" cy="100" r="50" fill="red"/>
</svg>
```

**Input (complex):**
```
Generate an SVG bar chart showing quarterly sales: Q1=150, Q2=230, Q3=180, Q4=300.
Include labeled axes, grid lines, and a title "2024 Sales".
```

**Input (icon reproduction):**
```
Reproduce this icon as SVG:
[image of a simple house icon]
```

## Difficulty Curriculum

1. **Easy:** Single basic shapes with specified properties (color, size, position). Verification can be exact (parse SVG attributes).
2. **Medium:** Multiple shapes, layering, basic transformations (translate, rotate, scale). Simple compositions.
3. **Hard:** Complex paths (Bezier curves), gradients, filters, text elements, data visualizations.
4. **Very hard:** Reproduce complex icons or illustrations from reference images. Animated SVGs. Interactive SVGs with CSS.

## Limitations & Risks

- **Visual similarity is approximate:** Two SVGs can look similar (high SSIM) while being semantically different, or look slightly different while being correct.
- **CLIP-based verification is gameable:** Models can learn to produce images that score well on CLIP similarity without actually matching the description. This is a known failure mode.
- **Rendering engine differences:** SVGs can render differently across engines (CairoSVG vs. Chrome vs. Firefox). Must standardize.
- **Infinite valid representations:** A red circle can be expressed as `<circle>`, an `<ellipse>` with equal radii, a `<path>` with arc commands, or even a heavily-parameterized `<polygon>`. Source comparison is useless; visual comparison is essential.
- **Complex SVGs are very long:** Detailed illustrations require thousands of tokens of SVG markup. This strains model context and output length.
- **Limited verifiability for complex descriptions:** "Draw a beautiful sunset" cannot be meaningfully verified. Stick to concrete, measurable descriptions.

## Connections

- [[html-css-generation]] — SVG is often embedded in HTML; visual verification approach is shared.
- [[latex-typesetting]] — Both produce visual output from markup.
- [[ascii-art]] — Text-based visual generation, lower fidelity but simpler verification.
- [[data-formatting]] — SVG is XML; format compliance is a subset of the verification.
