---
domain: latex-typesetting
category: format-constrained
verification_type: execution | diff
dataset_scale: ~large (im2latex + arXiv)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# LaTeX Typesetting

## Overview

LaTeX typesetting tasks require generating valid LaTeX code from either textual descriptions or images of typeset content. The model must produce LaTeX that (1) compiles without errors and (2) renders to match a reference visual. This domain is valuable because LaTeX is the standard for scientific communication, and automating its generation saves enormous human effort.

The domain has strong RLVR properties: compilation success is binary and exact, and visual similarity to a reference can be measured automatically. It exercises both format knowledge and spatial/structural reasoning.

## Verification Mechanism

**Type: Execution (compilation) + Diff (visual comparison)**

**Stage 1 — Compilation check:**
```bash
pdflatex -interaction=nonstopmode -halt-on-error output.tex
# Return code 0 = success, nonzero = failure
```
Binary reward: 0.3 for successful compilation, 0.0 for failure.

**Stage 2 — Visual similarity:**
1. Render both reference and model output to images (using `pdflatex` + `convert` or `pdf2image`).
2. Compare using image similarity metrics:
   - **Exact match:** Pixel-by-pixel comparison after normalization. Too strict for most cases.
   - **SSIM (Structural Similarity Index):** Tolerates minor rendering differences. Score 0-1.
   - **Normalized edit distance on rendered text:** OCR both images, compare extracted text.
   - **BLEU/CER on LaTeX source:** Compare source code tokens (weaker but doesn't require rendering).

**Reward structure:**
- Compilation success: 0.3
- SSIM > 0.95: additional 0.5
- SSIM > 0.99: additional 0.2 (near-perfect bonus)

**For math expressions specifically:** Parse both LaTeX expressions into a canonical form (e.g., SymPy) and check mathematical equivalence. This handles visually identical expressions written differently (`\frac{1}{2}` vs. `\tfrac{1}{2}` vs. `1/2`).

**Verification confidence: HIGH for compilation, MEDIUM-HIGH for visual similarity.** Compilation is binary. Visual similarity is well-defined but has a threshold problem — how close is "close enough"? SSIM > 0.95 is a reasonable threshold. Minor differences in spacing, font weight, etc. should not be penalized.

## Dataset Sources

- **im2latex-100k:** 100k images of LaTeX math expressions with corresponding source code. The standard benchmark.
- **arXiv bulk data:** Millions of LaTeX source files from arXiv papers. Can extract (rendered image, source) pairs.
- **Detexify training data:** Handwritten symbol recognition pairs.
- **MathJax rendering:** Render LaTeX math to images programmatically, creating unlimited pairs.
- **Wikipedia math expressions:** Thousands of math expressions with LaTeX source in wiki markup.
- **LaTeX documentation and examples:** CTAN package documentation contains countless examples.
- **Synthetic generation:** Generate LaTeX from templates with varying complexity (fractions, matrices, aligned equations, TikZ figures).

## Task Format

**Input (image to LaTeX):**
```
Convert this image to LaTeX:
[image of a rendered mathematical equation: integral of x^2 dx from 0 to 1]
```

**Output:**
```latex
\int_0^1 x^2 \, dx
```

**Input (description to LaTeX):**
```
Write LaTeX for a 3x3 matrix with entries [[1,2,3],[4,5,6],[7,8,9]], 
centered in a displayed equation.
```

**Output:**
```latex
\[
\begin{pmatrix}
1 & 2 & 3 \\
4 & 5 & 6 \\
7 & 8 & 9
\end{pmatrix}
\]
```

**Input (full document):**
```
Generate a LaTeX document with: title "Analysis Report", author "J. Smith",
two sections with lorem ipsum text, a numbered equation, and a table with
3 rows and 4 columns.
```

## Difficulty Curriculum

1. **Easy:** Single math expressions (fractions, exponents, Greek letters). im2latex-style tasks.
2. **Medium:** Multi-line equations (align environment), matrices, simple tables.
3. **Hard:** Full documents with figures (TikZ), bibliographies, complex table layouts, custom macros.
4. **Very hard:** Reproduce a full page from an image. Complex TikZ diagrams. Beamer presentations.

## Limitations & Risks

- **Multiple valid representations:** The same visual output can be produced by many different LaTeX source codes. Source-level comparison is too strict; visual comparison is necessary.
- **Rendering environment dependency:** LaTeX output depends on installed packages, fonts, and compiler version. Must standardize the compilation environment.
- **Compilation can be slow:** Full document compilation takes seconds. At RL training scale, this adds up. Mitigation: use lightweight compilation for math-only tasks, parallelize compilation.
- **Visual comparison imprecision:** SSIM can miss semantically important differences (e.g., `+` vs `-` in a small equation) while flagging irrelevant ones (slightly different whitespace). OCR-based comparison helps for text/math content.
- **Package availability:** Model may generate LaTeX requiring packages not installed. Must either provide a standard package set or penalize for unavailable packages.

## Connections

- [[html-css-generation]] — Both generate structured markup verified by rendering.
- [[svg-generation]] — Both produce visual output from code.
- [[data-formatting]] — LaTeX is a structured format with strict syntax rules.
- [[markdown-formatting]] — Markdown to LaTeX conversion is a specific subtask.
