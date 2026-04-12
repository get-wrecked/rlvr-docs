---
domain: html-css-generation
category: format-constrained
verification_type: execution | diff
dataset_scale: ~large (web datasets + synthetic)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# HTML/CSS Generation

## Overview

HTML/CSS generation tasks require producing web page markup and styling that matches a visual specification — either a screenshot, a wireframe, a textual description, or a reference design. This is a high-value practical domain: frontend development consumes enormous engineering time, and automated code generation from designs is a major productivity multiplier.

Verification combines execution (does it render?) with visual comparison (does it look right?), making it a solid RLVR candidate. The domain exercises spatial reasoning, structured output generation, and knowledge of web standards.

## Verification Mechanism

**Type: Execution (rendering) + Diff (visual comparison)**

**Stage 1 — Validity check:**
- Parse HTML with an HTML parser (BeautifulSoup, html5lib). Must parse without fatal errors.
- Validate CSS syntax (csslint, css-validator).
- Reward: 0.15 for valid HTML + CSS.

**Stage 2 — Rendering:**
- Render using a headless browser (Playwright, Puppeteer, Selenium) at a standardized viewport size.
- Capture screenshot.
- Reward: 0.15 for non-blank render.

**Stage 3 — Visual similarity to reference:**
```python
from skimage.metrics import structural_similarity as ssim
from PIL import Image

def visual_compare(reference_path: str, output_path: str) -> float:
    ref = Image.open(reference_path).convert('L')  # grayscale
    out = Image.open(output_path).convert('L')
    # Resize to common dimensions
    ref = ref.resize((1024, 768))
    out = out.resize((1024, 768))
    score = ssim(np.array(ref), np.array(out))
    return score
```

**Alternative metrics:**
- **Perceptual hash (pHash):** Fast approximate visual comparison.
- **LPIPS (Learned Perceptual Image Patch Similarity):** Deep learning-based perceptual metric, more aligned with human judgment.
- **DOM structure comparison:** Compare DOM trees structurally (element types, nesting, class names) independent of visual rendering.
- **Element-level comparison:** Check that specific elements exist (buttons, headers, images) at approximately correct positions.

**Reward structure:**
- Valid HTML/CSS: 0.15
- Renders non-blank: 0.15
- SSIM > 0.7: 0.3
- SSIM > 0.9: additional 0.2
- SSIM > 0.95: additional 0.2

**Verification confidence: MEDIUM-HIGH.** Rendering is deterministic given the same browser engine. Visual similarity metrics are well-established but imperfect — they can miss important differences (wrong text) and flag unimportant ones (slightly different shade of blue). Element-level checks help but add implementation complexity.

## Dataset Sources

- **Design2Code (Salt Lab, 2024):** 484 real-world web pages with screenshots + reference HTML/CSS. Purpose-built for this task.
- **WebSight (HuggingFace):** 823k synthetically generated website screenshots with corresponding HTML/CSS.
- **Common Crawl:** Billions of web pages. Extract (screenshot, HTML) pairs by rendering archived pages.
- **Figma/Sketch design files:** Community templates can be converted to screenshot + expected HTML pairs.
- **Bootstrap/Tailwind examples:** Framework documentation contains hundreds of component examples.
- **Frontend mentor challenges:** Coding challenges with reference designs and expected implementations.
- **Synthetic generation:** Use templating to generate UI specifications with known HTML/CSS solutions.

## Task Format

**Input (screenshot to code):**
```
Generate HTML and CSS that reproduces this webpage screenshot:
[image: a simple landing page with a centered heading "Welcome", 
a subtitle, a blue call-to-action button, and a footer]
```

**Output:**
```html
<!DOCTYPE html>
<html>
<head>
<style>
  body { font-family: Arial, sans-serif; text-align: center; margin: 0; padding: 0; }
  .hero { padding: 100px 20px; }
  h1 { font-size: 48px; margin-bottom: 10px; }
  .subtitle { font-size: 18px; color: #666; margin-bottom: 30px; }
  .cta { background: #007bff; color: white; border: none; padding: 15px 40px;
         font-size: 16px; border-radius: 5px; cursor: pointer; }
  footer { background: #333; color: white; padding: 20px; margin-top: 50px; }
</style>
</head>
<body>
  <div class="hero">
    <h1>Welcome</h1>
    <p class="subtitle">Your journey starts here</p>
    <button class="cta">Get Started</button>
  </div>
  <footer>© 2024 Company</footer>
</body>
</html>
```

**Input (description to code):**
```
Create a responsive navigation bar with:
- Logo on the left
- 4 nav links on the right (Home, About, Services, Contact)
- Hamburger menu on mobile (< 768px)
- Dark background (#1a1a2e), white text
```

## Difficulty Curriculum

1. **Easy:** Single-element layouts (centered text, basic button styling, simple cards).
2. **Medium:** Multi-component layouts (nav + hero + footer), basic responsive design, CSS flexbox/grid.
3. **Hard:** Complex layouts (dashboard with sidebar, multi-column grids), animations, form validation, responsive breakpoints.
4. **Very hard:** Full page reproductions from screenshots, pixel-perfect implementations, complex CSS features (gradients, transforms, clip-paths), JavaScript-dependent interactions.

## Limitations & Risks

- **Rendering environment variance:** Different browsers render CSS slightly differently. Must standardize on one browser engine (Chromium via Playwright recommended).
- **Screenshot resolution sensitivity:** Visual metrics are sensitive to viewport size, DPI, and font rendering. Must fix all rendering parameters.
- **Multiple valid implementations:** The same visual result can be achieved with flexbox, grid, floats, absolute positioning, etc. Source code comparison is useless; visual comparison is essential.
- **Dynamic content:** If the reference includes dynamic content (hover states, animations, scrolling), screenshot comparison captures only a static frame. Interactive verification requires browser automation.
- **Accessibility not captured:** A visually correct page may be semantically garbage (div soup). Consider combining with [[accessibility-compliance]] verification.
- **Slow verification:** Headless browser rendering takes 1-5 seconds per example. At RL scale, this is a significant bottleneck. Mitigation: parallel rendering, lightweight HTML subsets.

## Connections

- [[svg-generation]] — SVG can be embedded in HTML; visual verification approach is shared.
- [[latex-typesetting]] — Both are "code to visual" verification domains.
- [[accessibility-compliance]] — HTML quality extends beyond visual correctness.
- [[data-formatting]] — HTML is structured data; format compliance overlaps.
- [[design2code]] — Specific benchmark instantiation of this domain.
