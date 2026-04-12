---
domain: spatial-reasoning
category: vision/reasoning
verification_type: exact_match
dataset_scale: >100K examples
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Spatial Reasoning

## Overview

Spatial reasoning tasks require understanding and reasoning about spatial relationships, configurations, and transformations in visual scenes. This includes: determining relative positions of objects (left/right, above/below, in front/behind), understanding containment and occlusion, reasoning about geometric arrangements, matching visual patterns, and predicting spatial transformations. Unlike simple VQA that asks factual questions about image content, spatial reasoning tasks specifically test the ability to encode and manipulate spatial representations — a core cognitive capability that is notoriously difficult for language models. Verification is clean: answers are typically binary, categorical, or exact-match.

## Verification Mechanism

1. **Exact answer match** (exact match): Most spatial reasoning benchmarks have short, unambiguous answers (yes/no, true/false, left/right, or a specific count). Verification: direct string comparison.
2. **Binary classification** (exact match): NLVR/NLVR2 format: given an image (or pair of images) and a statement, determine if the statement is true or false. Perfectly unambiguous.
3. **Pattern matching** (exact match): Bongard problems: given two sets of images (positive and negative examples), classify a test image. Binary output.
4. **Spatial relationship extraction** (exact match): Given an image, identify the spatial relationship between two specified objects (e.g., "left of", "above", "inside"). Categorical output from fixed vocabulary.

## Dataset Sources

- **NLVR (Natural Language Visual Reasoning)**: https://lil.nlp.cornell.edu/nlvr/ — 92K sentence-image pairs. Each example: a synthetic image of colored shapes + a statement ("There is a yellow square above a blue circle"). Label: true/false.
- **NLVR2**: https://lil.nlp.cornell.edu/nlvr/nlvr2.html — 107K sentence-image pairs using real photographs. Harder than NLVR due to natural image complexity. Pairs of images with statements comparing them.
- **Bongard-LOGO**: https://github.com/NVlabs/Bongard-LOGO — 12K Bongard problems using abstract shapes. Each problem: 6 positive examples + 6 negative examples + 1 test image. Binary classification.
- **Bongard-HOI**: Bongard problems based on human-object interactions in real images.
- **Bongard-OpenWorld**: Extended Bongard problems with open-world concepts.
- **VSR (Visual Spatial Reasoning)**: https://github.com/cambridgeltl/visual-spatial-reasoning — 10K true/false questions about spatial relationships in natural images (COCO-based).
- **What'sUp**: https://github.com/amitakamath/whatsup_vlms — Benchmark testing spatial relationship understanding across multiple VLMs.
- **SpartQA**: Spatial reasoning QA in 3D scenes described in text + rendered images.
- **CLEVR (compositional questions)**: Spatial subset of CLEVR: questions about relative positions, sizes, distances between objects.
- **PTR (Perception Test for Real images)**: Spatial reasoning component of Google's Perception Test benchmark.
- **Spatial-MM**: Recent spatial reasoning benchmark for multimodal models.

## Task Format

**True/False spatial statements (NLVR-style)**:
- Input: Image (or image pair) + spatial statement (e.g., "There are at least three objects in the same row").
- Output: "true" or "false".
- Verification: Exact match.

**Spatial relationship questions**:
- Input: Image + question (e.g., "What is the spatial relationship between the cat and the box?").
- Output: Relationship label (e.g., "inside", "on top of", "to the left of").
- Verification: Exact match.

**Bongard-style pattern recognition**:
- Input: Set of positive examples + set of negative examples + test image.
- Output: "positive" or "negative".
- Verification: Exact match.

**Spatial comparison**:
- Input: Image pair + question (e.g., "Which image has more red objects?").
- Output: "left" / "right" / "same".
- Verification: Exact match.

**Transformation prediction**:
- Input: Image + transformation description (e.g., "Rotate 90 degrees clockwise. How many objects are now on the left?").
- Output: Count or spatial description.
- Verification: Exact match.

## Difficulty Curriculum

1. **Single object presence** (trivial): "Is there a red object?" Binary, no spatial reasoning needed.
2. **Basic relative position** (easy): "Is the circle above the square?" Two objects, one relation.
3. **Multi-object counting with spatial filter** (medium): "How many blue objects are to the left of the red line?"
4. **Comparative spatial reasoning** (medium): "Are there more circles above the line than below?"
5. **NLVR-style compositional statements** (medium-hard): Complex sentences with quantifiers and spatial terms.
6. **3D spatial reasoning** (hard): Reasoning about depth, occlusion, and 3D arrangement from 2D images.
7. **Bongard concept learning** (hard): Infer the spatial concept from examples and apply it.
8. **Multi-image comparison** (hard): Compare spatial arrangements across two images.
9. **Abstract spatial rule induction** (very hard): Bongard-LOGO with novel abstract concepts. Requires visual abstraction.
10. **Physical spatial reasoning** (very hard): Predict physical outcomes (will this stack of objects fall? will the ball hit the target?).

## Limitations & Risks

- **Ambiguous spatial terms**: "Next to", "near", "by" have no precise geometric definitions. Ground truth depends on annotator judgment. Verification is exact match, but the ground truth itself may be debatable.
- **Viewpoint dependence**: "Left" and "right" depend on frame of reference (viewer-centric vs object-centric). Datasets must be consistent but not all are.
- **Synthetic-to-real gap**: NLVR and CLEVR use synthetic images; real-world spatial reasoning involves occlusion, clutter, and ambiguous boundaries. NLVR2 uses real images but has limited diversity.
- **2D projection of 3D world**: All image-based spatial reasoning operates on 2D projections of 3D scenes. Depth relationships are inherently ambiguous from a single view.
- **Language shortcut exploitation**: Models may exploit surface-level language patterns in spatial statements without truly understanding spatial relationships. E.g., certain words may be statistically associated with true/false.
- **Limited benchmark diversity**: The number of dedicated spatial reasoning benchmarks is small compared to VQA or classification. Most are relatively small-scale.

## Connections

- [[visual-question-answering]] — Spatial VQA questions form a subset of general VQA
- [[visual-grounding]] — Grounding requires spatial reasoning to locate objects
- [[math-word-problems-visual]] — Geometry problems are a form of spatial reasoning
- [[puzzle-games]] — Many puzzles (Sokoban, sliding puzzles) exercise spatial reasoning
- [[map-navigation]] — Navigation requires spatial reasoning in map contexts
