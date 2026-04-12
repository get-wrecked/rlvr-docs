---
domain: multimodal-reasoning
category: multimodal/reasoning
verification_type: exact_match
dataset_scale: >50K problems
difficulty_range: medium/hard/superhuman
modality: multimodal
status: verified
---

# Multimodal Reasoning

## Overview

Multimodal reasoning tasks require multi-step reasoning that integrates information from both images and text to arrive at a verifiable answer. Unlike simple VQA (one-step perception -> answer), these tasks demand chaining multiple reasoning steps: interpreting a diagram, applying domain knowledge, performing computation, and synthesizing across modalities. The domain is critical for RLVR because it tests the highest-level cognitive capabilities of multimodal models while still maintaining verifiable answers (typically exact match against ground truth). Benchmarks like MMMU, ScienceQA, and M3Exam draw from real educational and professional assessments where answers are unambiguous.

## Verification Mechanism

1. **Multiple choice selection** (exact match): The majority of multimodal reasoning benchmarks are multiple choice. Verification: predicted option matches correct option. Completely unambiguous.
2. **Exact answer match** (exact match): For free-form answer benchmarks, compare agent's answer to ground truth after normalization. Number matching with tolerance, string normalization for text answers.
3. **Expression equivalence** (exact match): For mathematical answers, check symbolic equivalence via SymPy or similar. "2x + 4" = "4 + 2x" = "2(x+2)".
4. **Binary classification** (exact match): True/false questions about combined image-text claims.

## Dataset Sources

- **MMMU (Massive Multi-discipline Multimodal Understanding)**: https://mmmu-benchmark.github.io/ — 11.5K problems from 30 subjects across 6 disciplines (Art, Business, Science, Health, Humanities, Tech). College-level. Images include diagrams, charts, photos, maps, sheet music. Multiple choice + open-ended. THE benchmark for advanced multimodal reasoning.
- **ScienceQA**: https://scienceqa.github.io/ — 21K multiple-choice science questions with image context and explanations. Elementary to high school level. Covers natural science, social science, language science. Each question has a lecture + explanation annotation.
- **M3Exam**: https://github.com/DAMO-NLP-SG/M3Exam — 12.3K multiple-choice questions from real exams in 9 languages. Multilingual + multimodal. Diverse subject areas.
- **AI2D (Allen AI Diagrams)**: https://allenai.org/data/diagrams — 5K grade-school science diagrams with 15K multiple-choice questions. Tests diagram interpretation + scientific knowledge.
- **MathVista**: https://mathvista.github.io/ — 6,141 multimodal math problems. Overlaps with visual math but includes multi-step reasoning problems.
- **SEED-Bench**: https://github.com/AILab-CVC/SEED-Bench — 19K multiple-choice questions evaluating 12 dimensions of multimodal understanding including spatial, temporal, and compositional reasoning.
- **MMBench**: https://opencompass.org.cn/mmbench — 3K questions organized into fine-grained ability dimensions.
- **MME (Multimodal Evaluation)**: Yes/no format benchmark covering perception and cognition subtasks.
- **MM-Vet**: https://github.com/yuweihao/MM-Vet — 218 complex problems requiring integrated vision-language capabilities.
- **CMMMU**: Chinese version of MMMU. 12K problems from Chinese college exams.
- **RealWorldQA**: https://x.ai/blog/grok-1.5v — 765 real-world multimodal reasoning questions.

## Task Format

**Multiple choice multimodal reasoning**:
- Input: Image(s) + question text + answer options. Image might be a scientific diagram, medical scan, architectural drawing, music score, map, or historical photograph. Question requires understanding the image AND applying domain knowledge.
- Example: [Image of a circuit diagram] "If the voltage source is 12V and R1=4 ohms, R2=6 ohms in parallel, what is the total current? (A) 1.2A (B) 2.0A (C) 3.0A (D) 5.0A"
- Output: Selected option (e.g., "D").
- Verification: Exact match.

**Open-ended multimodal reasoning**:
- Input: Image + complex question requiring multi-step reasoning.
- Output: Short answer (number, term, name).
- Verification: Exact match after normalization.

**Multi-image reasoning**:
- Input: Multiple images + question requiring comparison or synthesis across images.
- Output: Answer.
- Verification: Exact match.

**Diagram interpretation + computation**:
- Input: Technical diagram (circuit, flow chart, molecular structure) + computational question.
- Output: Computed answer.
- Verification: Numeric match with tolerance.

## Difficulty Curriculum

1. **Simple diagram reading** (easy): Read a labeled diagram. AI2D basic questions.
2. **Single-step science** (easy-medium): Answer a science question using one piece of visual evidence. ScienceQA elementary level.
3. **Multi-step science** (medium): Chain two reasoning steps. ScienceQA middle school level.
4. **Chart + computation** (medium-hard): Read data from a chart and perform calculations.
5. **College-level STEM** (hard): MMMU science/engineering questions. Requires domain expertise + visual reasoning.
6. **Medical image interpretation** (hard): MMMU health/medical questions with clinical images.
7. **Cross-domain reasoning** (very hard): Questions requiring knowledge from multiple domains applied to complex visual inputs.
8. **Professional-level** (superhuman): MMMU questions from graduate-level exams. Specialized professional knowledge + complex diagram interpretation.
9. **Multi-image synthesis** (superhuman): Integrate information across multiple complex images to answer a question.

## Limitations & Risks

- **Domain knowledge conflation**: It is hard to separate "multimodal reasoning ability" from "domain knowledge." A model might fail a medical question because it does not know medicine, not because it cannot reason multimodally. RLVR training on these tasks teaches domain knowledge as much as reasoning.
- **Dataset contamination**: Many benchmark problems come from published exams and textbooks. Large language models may have seen these problems during pre-training. Verification of novel reasoning (vs memorization) is difficult.
- **Multiple choice format limitations**: Multiple choice allows guessing (25% for 4 options). Models can exploit option elimination rather than truly solving the problem. Also, the correct answer must be among the options, which provides information a free-form answer would not.
- **Image quality dependency**: Problems with low-resolution, poorly scanned, or ambiguously drawn images are harder not because of reasoning difficulty but because of perceptual difficulty. This confounds reasoning evaluation.
- **Cultural and language bias**: Exams from different countries test different curricula. M3Exam helps with diversity but most benchmarks are English-centric.
- **Limited scale**: MMMU has 11.5K problems, ScienceQA has 21K. These are small compared to text-only reasoning datasets. Generating new high-quality multimodal reasoning problems at scale is expensive.
- **Explanation gap**: Correct answers do not guarantee correct reasoning. A model might arrive at the right multiple-choice answer through flawed reasoning or guessing. ScienceQA's explanation annotations help but are not used in standard verification.

## Connections

- [[math-word-problems-visual]] — Visual math is a core component of multimodal reasoning
- [[chart-understanding]] — Chart questions are a common multimodal reasoning subtype
- [[visual-question-answering]] — VQA is the simpler, single-step predecessor
- [[spatial-reasoning]] — Spatial reasoning is often required in multimodal reasoning tasks
- [[document-ocr-extraction]] — Reading text from images is a prerequisite skill
