---
domain: Constrained Image Generation (Code-Based)
category: vision-creative
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Constrained Image Generation (Code-Based)

## Overview
Generate code (Python with PIL/matplotlib/turtle/Processing) that produces an image satisfying specified visual constraints: specific shapes at specific positions, color requirements, symmetry properties, text rendering, pixel-level patterns. Verification: render the image, check constraints programmatically.

## Verification Mechanism
```python
from PIL import Image
import numpy as np

def verify(constraints: dict, code: str) -> float:
    # Execute the code to produce an image
    try:
        image = execute_image_code(code, timeout=30)
    except Exception:
        return 0.0
    
    pixels = np.array(image)
    score = 0.0
    checks = 0
    
    # Dimensions
    if "size" in constraints:
        checks += 1
        if image.size == tuple(constraints["size"]):
            score += 1
    
    # Background color
    if "background" in constraints:
        checks += 1
        corners = [pixels[0,0], pixels[0,-1], pixels[-1,0], pixels[-1,-1]]
        expected = np.array(constraints["background"])
        if all(np.allclose(c, expected, atol=5) for c in corners):
            score += 1
    
    # Contains specific colors
    if "required_colors" in constraints:
        for color in constraints["required_colors"]:
            checks += 1
            if color_present(pixels, color, tolerance=10):
                score += 1
    
    # Shape detection (circles, rectangles)
    if "shapes" in constraints:
        detected = detect_shapes(image)
        for req_shape in constraints["shapes"]:
            checks += 1
            if matches_shape(detected, req_shape):
                score += 1
    
    # Symmetry
    if "symmetry" in constraints:
        checks += 1
        if constraints["symmetry"] == "horizontal":
            flipped = np.flip(pixels, axis=1)
            if np.allclose(pixels, flipped, atol=5):
                score += 1
        elif constraints["symmetry"] == "vertical":
            flipped = np.flip(pixels, axis=0)
            if np.allclose(pixels, flipped, atol=5):
                score += 1
    
    # Text presence (OCR check)
    if "text" in constraints:
        checks += 1
        detected_text = ocr(image)
        if constraints["text"].lower() in detected_text.lower():
            score += 1
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **Turtle graphics exercises**: Programming education datasets.
- **Processing.org examples**: Creative coding examples.
- **Generative art communities**: OpenProcessing, r/generative.
- **Procedural generation**: Specify random visual constraints, verify code output.
- **Flag/logo reconstruction**: Known flags/logos as visual targets.

## Task Format
- **Input**: "Generate a 400x400 image with a red circle (radius 50) centered at (200,200) and a blue rectangle from (50,300) to (150,350) on a white background."
- **Output**: Python code that generates the image

## Difficulty Curriculum
- Level 1: Single shape, basic colors
- Level 3: Multiple shapes with specific positions
- Level 5: Complex compositions, gradients, patterns
- Level 7: Recreate logos/flags from description
- Level 9: Generative art (fractal patterns, L-systems with specific properties)

## Limitations & Risks
- Shape detection is approximate. Simple constraints (color, dimensions) are exact.
- Complex visual matching (e.g., "looks like a house") is not reliably verifiable. Stick to geometric/structural constraints.
- This is NOT about aesthetic quality — it's about precise constraint satisfaction.

## Connections
- [[svg-generation]] — SVG-specific image generation
- [[html-css-generation]] — web visual generation
- [[code-generation]] — the output is code
- [[constrained-writing]] — constrained generation in another modality
