---
domain: Typographic Layout
category: creative-constrained
verification_type: constraint
dataset_scale: ~25K designs (Crello), thousands of magazine layouts
difficulty_range: easy/medium/hard
modality: text
status: needs_validation
---

# Typographic Layout

## Overview

Typographic layout arranges text elements (headings, subheadings, body text, captions) within a defined canvas satisfying design constraints: no overlapping elements, alignment to grids, visual hierarchy through size/weight, readability requirements (minimum font size, sufficient contrast), and aesthetic principles (balance, whitespace).

RLVR verification is partially achievable: geometric constraints (no overlap, alignment, boundary compliance) are exactly checkable, and readability metrics (font size, contrast ratio) are computable. Aesthetic quality is harder but can be approximated through learned quality models or design heuristics. This domain bridges the gap between purely creative and fully verifiable tasks.

## Verification Mechanism

```python
def verify_layout(elements: list, canvas: dict, constraints: dict) -> dict:
    """
    elements: [{"type": "heading", "text": "...", "x": 10, "y": 20,
                "width": 200, "height": 40, "font_size": 24, "color": "#000"}, ...]
    canvas: {"width": 800, "height": 600}
    """
    score = 0.0
    checks = 0

    # 1. No overlap between elements
    checks += 1
    overlap_found = False
    for i in range(len(elements)):
        for j in range(i + 1, len(elements)):
            if rectangles_overlap(elements[i], elements[j]):
                overlap_found = True
                break
    if not overlap_found:
        score += 1.0

    # 2. All elements within canvas bounds
    checks += 1
    all_in_bounds = all(
        e['x'] >= 0 and e['y'] >= 0 and
        e['x'] + e['width'] <= canvas['width'] and
        e['y'] + e['height'] <= canvas['height']
        for e in elements
    )
    if all_in_bounds:
        score += 1.0

    # 3. Visual hierarchy: headings larger than body text
    checks += 1
    headings = [e for e in elements if e['type'] == 'heading']
    body = [e for e in elements if e['type'] == 'body']
    if headings and body:
        min_heading_size = min(e['font_size'] for e in headings)
        max_body_size = max(e['font_size'] for e in body)
        if min_heading_size > max_body_size:
            score += 1.0
    else:
        score += 1.0  # no hierarchy to check

    # 4. Minimum font size
    checks += 1
    min_font = constraints.get('min_font_size', 8)
    if all(e['font_size'] >= min_font for e in elements):
        score += 1.0

    # 5. Alignment check: elements should align to grid or each other
    checks += 1
    grid_size = constraints.get('grid_size', 8)
    aligned_count = sum(
        1 for e in elements
        if e['x'] % grid_size == 0 and e['y'] % grid_size == 0
    )
    alignment_ratio = aligned_count / len(elements) if elements else 1.0
    score += alignment_ratio

    # 6. Contrast ratio (WCAG compliance)
    if 'background_color' in constraints:
        checks += 1
        bg = constraints['background_color']
        all_pass = all(
            contrast_ratio(e['color'], bg) >= 4.5  # WCAG AA for normal text
            for e in elements
        )
        if all_pass:
            score += 1.0

    # 7. Reading order: elements should flow top-to-bottom, left-to-right
    checks += 1
    text_elements = [e for e in elements if e['type'] in ('heading', 'body', 'subheading')]
    if text_elements:
        sorted_by_position = sorted(text_elements, key=lambda e: (e['y'], e['x']))
        in_order = all(
            sorted_by_position[i]['y'] <= sorted_by_position[i+1]['y']
            for i in range(len(sorted_by_position) - 1)
        )
        if in_order:
            score += 1.0
    else:
        score += 1.0

    return {
        "constraint_score": score / checks if checks > 0 else 0.0,
        "checks_passed": score,
        "total_checks": checks,
        "no_overlap": not overlap_found,
        "in_bounds": all_in_bounds,
        "reward": score / checks if checks > 0 else 0.0
    }

def rectangles_overlap(a: dict, b: dict) -> bool:
    """Check if two rectangles overlap."""
    return not (a['x'] + a['width'] <= b['x'] or
                b['x'] + b['width'] <= a['x'] or
                a['y'] + a['height'] <= b['y'] or
                b['y'] + b['height'] <= a['y'])

def contrast_ratio(fg_hex: str, bg_hex: str) -> float:
    """WCAG contrast ratio between two colors."""
    def relative_luminance(hex_color):
        r, g, b = int(hex_color[1:3], 16)/255, int(hex_color[3:5], 16)/255, int(hex_color[5:7], 16)/255
        r = r/12.92 if r <= 0.03928 else ((r+0.055)/1.055)**2.4
        g = g/12.92 if g <= 0.03928 else ((g+0.055)/1.055)**2.4
        b = b/12.92 if b <= 0.03928 else ((b+0.055)/1.055)**2.4
        return 0.2126*r + 0.7152*g + 0.0722*b
    l1 = relative_luminance(fg_hex)
    l2 = relative_luminance(bg_hex)
    lighter = max(l1, l2)
    darker = min(l1, l2)
    return (lighter + 0.05) / (darker + 0.05)
```

## Dataset Sources

- **Crello (CrelloV2)**: ~25,000 graphic design templates with element positions, sizes, fonts, and colors. Covers social media posts, presentations, banners, and cards. Includes text elements, images, and shapes with bounding boxes.
- **Magazine Layout Dataset**: Academic datasets of magazine page layouts with annotated text blocks, images, and whitespace. Several thousand pages from various publications.
- **PubLayNet**: 360,000 document page images with annotated layout elements (text, title, table, figure, list). Microsoft Research. Focused on document layout rather than creative design.
- **RICO**: 66,261 mobile UI screenshots with element hierarchies. Useful for UI layout specifically.
- **WebUI**: Web page layouts with DOM element positions and styling.
- **Canva/Figma template libraries**: Thousands of design templates with structured element data (commercial, but templates are viewable).
- **Procedural generation**: Define a canvas size and a set of text elements with hierarchy, then enumerate valid layouts satisfying basic constraints. Unlimited supply for simple cases.
- **LayoutGAN/LayoutTransformer training data**: Processed design datasets used in layout generation research.

## Task Format

- **Input**: A set of text elements to arrange within a canvas.
```
Arrange the following elements on an 800x600px canvas for a social media post:

Elements:
1. Heading: "Summer Sale" (must be prominent, top area)
2. Subheading: "Up to 50% Off" (below heading)
3. Body: "Valid June 1-30. Online and in-store. Excludes clearance items."
4. Call-to-action: "Shop Now" (button-like, bottom area)
5. Logo placeholder: 100x40px (corner)

Constraints:
- No element overlap
- Minimum font size: 12px
- Grid: 8px
- Background: #FFFFFF
- All text must have contrast ratio >= 4.5 against background
```
- **Output**: Element specifications with positions and sizes.
```json
[
  {"type": "heading", "text": "Summer Sale", "x": 200, "y": 80,
   "width": 400, "height": 72, "font_size": 48, "color": "#1A1A1A"},
  {"type": "subheading", "text": "Up to 50% Off", "x": 240, "y": 168,
   "width": 320, "height": 48, "font_size": 32, "color": "#E63946"},
  {"type": "body", "text": "Valid June 1-30...", "x": 200, "y": 248,
   "width": 400, "height": 80, "font_size": 16, "color": "#333333"},
  {"type": "cta", "text": "Shop Now", "x": 300, "y": 400,
   "width": 200, "height": 56, "font_size": 24, "color": "#FFFFFF",
   "background": "#E63946"},
  {"type": "logo", "text": "", "x": 680, "y": 16,
   "width": 100, "height": 40, "font_size": 0, "color": "#000000"}
]
```

## Difficulty Curriculum

- Level 1: Two non-overlapping text blocks on a large canvas (trivial placement)
- Level 2: 3-4 elements with clear hierarchy (heading, body, caption)
- Level 3: 5-6 elements with alignment constraints (grid snap, centering)
- Level 4: Mixed element types (text + image placeholders + buttons) with hierarchy
- Level 5: Responsive layout (same elements for different canvas sizes)
- Level 6: Multi-column layouts with balanced visual weight
- Level 7: Magazine-style layouts with text wrapping around images
- Level 8: Complex constraints (specific whitespace ratios, golden ratio proportions, baseline grid alignment)
- Level 9: Full page layouts with 15+ elements, mixed media, and stylistic consistency requirements

## Limitations & Risks

- **Aesthetic quality is unverifiable**: Constraint satisfaction ensures technical validity but not visual appeal. Two layouts can satisfy all constraints while one looks professional and the other looks amateur. Design quality requires human judgment or learned aesthetic models.
- **Text rendering**: The model outputs positions and sizes but does not render actual text. Line breaking, text overflow, and font metrics depend on the rendering engine. A layout that looks correct in the model's output may overflow when rendered with real fonts.
- **Font metrics**: Font size alone does not determine text bounding box size. Different fonts at the same point size have different ascenders, descenders, and character widths. Without a rendering engine, text dimensions are approximate.
- **Design conventions vary by context**: A social media post, a business card, and a magazine page follow different layout conventions. The model must learn context-appropriate design.
- **Whitespace is meaningful**: Good design uses negative space intentionally. Constraint checking can verify minimum margins but cannot assess whether whitespace is used effectively.
- **Output format complexity**: Layout specifications are structured JSON with many interdependent fields. Text-based LLMs can generate this but errors in one field (wrong coordinate) can cascade.
- **Limited training data**: Crello has ~25K designs, which is small by ML standards. Magazine layouts are even scarcer in structured form.

## Connections

- [[pixel-art-generation]] — Both involve spatial arrangement within constrained visual grids
- [[ascii-art]] — Both produce structured visual output; ASCII art uses character grids, layout uses element positions
- [[html-css-generation]] — HTML/CSS is a specific implementation of layout; this domain is more abstract
- [[accessibility-compliance]] — WCAG contrast and readability requirements overlap
- [[svg-generation]] — SVG can implement typographic layouts; this domain defines the abstract layout
