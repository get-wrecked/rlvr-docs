---
domain: math-word-problems-visual
category: vision/math
verification_type: exact_match
dataset_scale: >50K problems
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: verified
---

# Visual Math Problems

## Overview

Visual math problems require understanding mathematical content presented in images — geometry diagrams, function graphs, bar charts with math questions, geometric constructions, visual proofs, and problems combining formulas with figures. This domain sits at the intersection of mathematical reasoning and visual understanding. Unlike pure text math (which can be solved from notation alone), these problems fundamentally require parsing visual information: measuring angles from diagrams, reading coordinates from graphs, interpreting geometric relationships from figures. Verification is clean: the answer is an exact number, expression, or multiple-choice selection.

## Verification Mechanism

1. **Exact numeric answer match** (exact match): Most visual math problems have a single correct numeric answer (e.g., "The area of triangle ABC is 24"). Verification: compare agent's answer to ground truth after normalization (handle floating point tolerance, unit conversions, equivalent forms like 1/2 = 0.5).
2. **Multiple choice selection** (exact match): Many benchmarks present multiple choice options. Verification: agent's selected option matches correct option. Completely unambiguous.
3. **Expression equivalence** (exact match): For symbolic answers (e.g., "x^2 + 2x + 1"), use symbolic math libraries (SymPy) to check equivalence. Handles commutativity, distribution, etc.
4. **Interval/approximation matching** (constraint-based): For problems requiring measurement from diagrams (estimate the angle), accept answers within a tolerance. E.g., ground truth 45 degrees, accept 43-47.

## Dataset Sources

- **MathVista**: https://mathvista.github.io/ — 6,141 problems from 31 multimodal math datasets. Covers geometry, statistics, algebra, and scientific reasoning with figures. Mix of multiple choice and free-form answers. Gold standard comprehensive benchmark.
- **GeoQA**: ~5,000 Chinese geometry problems with diagrams. Translated versions available. Problems involve computing angles, lengths, areas from geometric figures.
- **GeoQA+**: Extended version with ~12K problems and more diverse geometric reasoning.
- **Geometry3K**: https://lupantech.github.io/inter-gps/ — 3,002 multi-choice geometry problems with formal annotations. Diagrams + text.
- **UniGeo**: Unified geometry benchmark combining calculation and proving tasks with diagrams.
- **CLEVR-Math**: Mathematical reasoning over CLEVR-style synthetic scenes. Counting, spatial arithmetic.
- **IconQA**: https://iconqa.github.io/ — 107K problems requiring abstract diagram understanding (icons, shapes, color patterns). Mostly elementary-level.
- **TabMWP**: https://promptpg.github.io/TabMWP/ — 38K math word problems with tabular context. Tables are presented as images or structured data.
- **MathVerse**: Multi-version visual math benchmark testing whether models truly use visual information.
- **MATH-Vision (MATH-V)**: https://mathvision-cuhk.github.io/ — 3,040 problems from real math competitions with visual content.
- **We-Math**: Fine-grained multi-step visual math evaluation.

## Task Format

**Multiple choice geometry**:
- Input: Image of a geometric diagram (triangle with marked angles, circle with inscribed polygon, etc.) + question text (e.g., "In the figure, angle A = 40 degrees and AB = AC. What is angle B?") + answer choices.
- Output: Selected choice (A/B/C/D).
- Verification: Exact match with correct choice.

**Free-form numeric answer**:
- Input: Image + question (e.g., "What is the area of the shaded region?").
- Output: Numeric answer (e.g., "16pi" or "50.27").
- Verification: Numeric comparison with tolerance (relative or absolute).

**Graph/chart math**:
- Input: Image of a graph + quantitative question (e.g., "At what x-value does the function reach its maximum?").
- Output: Numeric or expression answer.
- Verification: Exact match or tolerance-based comparison.

## Difficulty Curriculum

1. **Counting objects** (trivial): Count shapes in an image. CLEVR-Math level.
2. **Basic geometry** (easy): Identify shapes, basic angle sums. Elementary level.
3. **Area and perimeter** (easy-medium): Compute areas of standard shapes from labeled diagrams.
4. **Multi-step geometry** (medium): Apply multiple theorems (similar triangles, Pythagorean theorem). GeoQA level.
5. **Circle theorems** (medium-hard): Inscribed angles, tangent lines, arc lengths.
6. **Coordinate geometry** (hard): Read coordinates from graphs, compute distances and slopes.
7. **Graph analysis** (hard): Interpret function graphs, find intersections, extrema.
8. **Competition geometry** (very hard): AMC/AIME level problems requiring creative constructions. MATH-V level.
9. **Olympiad geometry** (superhuman): IMO/Putnam level problems requiring novel insights and multiple diagram transformations.

## Limitations & Risks

- **Diagram quality matters**: Low-resolution, hand-drawn, or poorly formatted diagrams significantly affect model performance. Training data diagrams may not match test data quality.
- **Text-in-image conflation**: Many problems contain text overlaid on diagrams. OCR errors or missed text can invalidate the problem. High-quality vision encoders are essential.
- **Measurement ambiguity**: Some problems require reading approximate values from diagrams (e.g., "estimate the angle"). Ground truth may not perfectly match what the diagram shows, especially if the diagram is not drawn to scale (and many are explicitly marked "not to scale").
- **Cultural and language bias**: Many datasets originate from Chinese math education (GeoQA, GeoQA+). Translation quality varies. Problem styles differ across educational systems.
- **Limited dataset size**: Compared to text-only math datasets (millions of problems), visual math datasets are relatively small (tens of thousands). Data augmentation through diagram transformation can help.
- **Answer format sensitivity**: "12.0", "12", "12.00", and "twelve" should all be accepted. Normalization must be robust.

## Connections

- [[visual-question-answering]] — Visual math is a specialized, verifiable form of VQA
- [[chart-understanding]] — Chart questions with numeric answers overlap significantly
- [[spatial-reasoning]] — Geometry requires spatial reasoning
- [[multimodal-reasoning]] — Visual math is a core component of multimodal reasoning benchmarks
