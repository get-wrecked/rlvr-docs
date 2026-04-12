---
domain: accessibility-compliance
category: format-constrained
verification_type: execution | rule
dataset_scale: ~medium (web datasets + synthetic)
difficulty_range: easy/medium/hard
modality: text | code
status: strong_hypothesis
---

# Accessibility Compliance (WCAG)

## Overview

Accessibility compliance tasks require modifying or generating HTML that meets Web Content Accessibility Guidelines (WCAG) standards. The Web is the primary interface for information access, and accessibility ensures it works for people with disabilities — including screen reader users, keyboard-only users, and people with visual or cognitive impairments.

This is a strong RLVR domain because automated accessibility checkers (axe-core, WAVE, Lighthouse) provide programmatic, deterministic verification of many WCAG criteria. Not all WCAG requirements are automatable (some require human judgment), but a substantial subset is.

## Verification Mechanism

**Type: Execution-based (accessibility linting tools)**

**Primary tool: axe-core** (Deque Systems' accessibility engine, open source)

```python
from playwright.sync_api import sync_playwright

def verify_accessibility(html: str) -> dict:
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        page.set_content(html)
        
        # Inject axe-core and run analysis
        page.add_script_tag(url="https://cdnjs.cloudflare.com/ajax/libs/axe-core/4.8.2/axe.min.js")
        results = page.evaluate("axe.run()")
        
        violations = results['violations']
        passes = results['passes']
        
        browser.close()
        
    total_rules = len(violations) + len(passes)
    pass_rate = len(passes) / total_rules if total_rules > 0 else 0
    
    return {
        "violations": len(violations),
        "violation_details": [v['id'] for v in violations],
        "passes": len(passes),
        "pass_rate": pass_rate,
        "reward": pass_rate
    }
```

**Automatable WCAG checks (partial list):**
- Images have alt text (WCAG 1.1.1)
- Form inputs have labels (WCAG 1.3.1)
- Sufficient color contrast (WCAG 1.4.3) — minimum 4.5:1 ratio
- Page has language attribute (WCAG 3.1.1)
- No duplicate IDs (WCAG 4.1.1)
- Links have discernible text (WCAG 2.4.4)
- Headings are in logical order (WCAG 1.3.1)
- ARIA attributes are valid (WCAG 4.1.2)
- Skip navigation links present (WCAG 2.4.1)
- Focus indicators visible (WCAG 2.4.7)
- Tables have headers (WCAG 1.3.1)

**Reward structure:**
- All axe-core violations resolved: 0.6
- All axe-core "best practice" checks pass: 0.2
- Original content/functionality preserved: 0.2

**Verification confidence: MEDIUM-HIGH.** Axe-core covers about 40-60% of WCAG 2.1 AA criteria automatically. The remaining criteria require human judgment (e.g., "Is this alt text actually descriptive?" — axe-core checks that alt text *exists*, not that it's *good*). For RLVR purposes, focusing on automatable criteria is appropriate and still very valuable.

## Dataset Sources

- **WebAIM Million:** Annual analysis of the top 1 million websites for accessibility. Identifies common issues.
- **Common Crawl + axe-core:** Run axe-core on Common Crawl pages to identify (inaccessible HTML, accessibility violation) pairs. Create tasks: "Fix the accessibility issues in this HTML."
- **GOV.UK and US government sites:** Required to be accessible; provide examples of compliant HTML.
- **A11y Project resources:** Curated examples and patterns.
- **W3C WAI tutorials:** Official WCAG implementation examples.
- **Deque University:** Training materials with before/after accessibility examples.
- **Synthetic generation:** Generate inaccessible HTML (images without alt, forms without labels, low-contrast text) and task the model with fixing it.
- **Real-world accessibility audits:** Published audit reports identify specific violations.

## Task Format

**Input (fix violations):**
```
Fix all accessibility violations in this HTML:

<div>
  <img src="hero.jpg">
  <div style="color: #999; background: #fff;">Important notice</div>
  <input type="text" placeholder="Enter email">
  <div onclick="submit()">Submit</div>
  <div class="nav">
    <div onclick="goHome()">Home</div>
    <div onclick="goAbout()">About</div>
  </div>
</div>
```

**Output:**
```html
<html lang="en">
<body>
  <nav aria-label="Main navigation">
    <a href="/" tabindex="0">Home</a>
    <a href="/about" tabindex="0">About</a>
  </nav>
  <main>
    <img src="hero.jpg" alt="Hero banner showing company headquarters">
    <p style="color: #595959; background: #fff;">Important notice</p>
    <label for="email">Email address</label>
    <input type="email" id="email" placeholder="Enter email" aria-required="true">
    <button type="submit" onclick="submit()">Submit</button>
  </main>
</body>
</html>
```

**Input (generate accessible):**
```
Generate an accessible HTML form for user registration with:
- Name (required), email (required), password (required), phone (optional)
- Submit button
- Error messages for invalid inputs
- Must pass all axe-core checks
```

## Difficulty Curriculum

1. **Easy:** Fix single violations (add alt text, add form labels, fix color contrast).
2. **Medium:** Fix multiple violations in a page (combine alt text, labels, ARIA roles, semantic HTML). Generate accessible components from scratch.
3. **Hard:** Make complex interactive widgets accessible (modals, dropdowns, tab interfaces, data tables) — requires ARIA patterns. Fix pages with deeply nested accessibility issues.
4. **Very hard:** Full page accessibility remediation (fix all violations while preserving visual design and functionality). Accessible single-page application patterns. Dynamic content accessibility (live regions, focus management).

## Limitations & Risks

- **Partial coverage:** ~40-60% of WCAG criteria are automatable. Tasks involving the remaining criteria (meaningful alt text, logical reading order, consistent navigation) would need human judgment.
- **Tool dependency:** axe-core, WAVE, and Lighthouse may flag different issues. Must standardize on one tool version.
- **Preserving functionality:** When fixing accessibility, the model must not break existing functionality. Verifying that functionality is preserved adds complexity.
- **Alt text quality:** Automated tools check presence, not quality. `alt="image"` passes axe-core but is terrible. Could partially address by requiring alt text to contain relevant keywords.
- **Rendering requirement:** Running axe-core requires a browser environment, adding infrastructure overhead.

## Connections

- [[html-css-generation]] — Accessibility is a quality dimension of HTML generation.
- [[protocol-compliance]] — WCAG is a compliance standard, similar to protocol RFCs.
- [[instruction-following]] — Applying WCAG rules to HTML is a form of instruction following.
- [[config-generation]] — Both involve generating standards-compliant structured output.
