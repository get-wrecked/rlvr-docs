---
domain: Image-to-Code (UI Screenshot → Code)
category: multimodal-code
verification_type: diff
dataset_scale: 100K+ (from web screenshots + design tools)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Image-to-Code (UI Screenshot → Code)

## Overview
Given a screenshot of a UI (web page, mobile app, desktop application), generate the code (HTML/CSS/React/SwiftUI) that reproduces it. Verification: render the generated code, compare screenshot to original using visual similarity metrics.

## Verification Mechanism
```python
from PIL import Image
from skimage.metrics import structural_similarity as ssim

def verify(reference_image: Image, generated_code: str, framework: str) -> float:
    # Render the generated code
    if framework in ["html", "react"]:
        rendered = headless_browser_screenshot(generated_code)
    elif framework == "swiftui":
        rendered = xcode_preview_screenshot(generated_code)
    
    if rendered is None:
        return 0.0  # Code doesn't render
    
    # Resize to same dimensions
    ref = np.array(reference_image.resize((512, 512)))
    gen = np.array(rendered.resize((512, 512)))
    
    # Structural similarity
    similarity = ssim(ref, gen, multichannel=True)
    
    # Element detection (check key UI elements exist)
    ref_elements = detect_ui_elements(reference_image)
    gen_elements = detect_ui_elements(rendered)
    element_recall = len(set(ref_elements) & set(gen_elements)) / max(len(ref_elements), 1)
    
    # Combine
    return 0.6 * similarity + 0.4 * element_recall
```

## Dataset Sources
- **Design2Code**: Benchmark for screenshot-to-code.
- **WebSight**: 823K synthetic webpage screenshots with HTML/CSS.
- **Pix2Code**: UI screenshot to DSL.
- **RICO**: 66K Android UI screenshots with view hierarchies.
- **Clay**: UI component screenshots with code.
- **Figma-to-code datasets**: Design file exports.
- **Web archive screenshots**: Cached web pages with source code.

## Task Format
- **Input**: Screenshot of a login page
- **Output**: HTML/CSS that reproduces the layout

## Difficulty Curriculum
- Level 1: Simple layouts (single column, few elements)
- Level 3: Multi-column layouts, forms, navigation
- Level 5: Complex responsive layouts, animations
- Level 7: Full web application pages
- Level 9: Pixel-perfect reproduction of complex designs

## Connections
- [[html-css-generation]] — HTML/CSS generation
- [[code-generation]] — code from specification
- [[visual-question-answering]] — visual understanding
- [[gui-navigation]] — UI understanding
