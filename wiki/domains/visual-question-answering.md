---
domain: visual-question-answering
category: vision/vqa
verification_type: exact_match
dataset_scale: >5M question-answer pairs
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Visual Question Answering (VQA)

## Overview

Visual Question Answering requires answering natural-language questions about images. The RLVR appeal is straightforward: when questions have short, factual, verifiable answers, verification is simple exact-match comparison. VQA is one of the most well-studied multimodal AI tasks, with massive datasets and well-understood evaluation protocols. It tests visual perception (object recognition, counting, color, spatial relationships), scene understanding, and the ability to ground language in visual content. The key RLVR challenge is that not all VQA questions have cleanly verifiable answers — the domain works best when restricted to factual, closed-ended questions.

## Verification Mechanism

1. **Exact string match** (exact match): Compare agent's answer to ground truth answer after normalization (lowercasing, stripping articles/punctuation, number word-to-digit conversion). The VQAv2 evaluation protocol uses soft accuracy: `min(count_of_humans_who_gave_this_answer / 3, 1)` over 10 human annotations, accounting for answer ambiguity.
2. **Multiple choice selection** (exact match): For datasets with fixed answer choices (A-OKVQA, ScienceQA), verification is trivial.
3. **Numeric answer match** (exact match): For counting questions or quantitative questions, compare numbers directly.
4. **Set match** (exact match): For questions asking to list multiple items, compare sets. E.g., "What colors are the cars?" — {red, blue} vs {blue, red}.

## Dataset Sources

- **VQAv2**: https://visualqa.org/ — 1.1M questions on 200K COCO images. 10 human answers per question. The foundational VQA benchmark. Open-ended and yes/no questions.
- **GQA**: https://cs.stanford.edu/people/dorarad/gqa/ — 22M questions generated from scene graphs. Highly structured, compositional questions. Strong ground truth from scene graph annotations.
- **CLEVR**: https://cs.stanford.edu/people/jcjohns/clevr/ — 1M questions on synthetic 3D-rendered scenes. Tests compositional reasoning (counting, comparing, spatial relations). Perfect ground truth.
- **OK-VQA**: https://okvqa.allenai.org/ — 14K questions requiring external knowledge beyond the image. Harder, but still exact-match verifiable.
- **A-OKVQA**: https://allenai.org/project/a-okvqa — Augmented version of OK-VQA with rationales and multiple choice format. ~25K questions.
- **TextVQA**: https://textvqa.org/ — 45K questions requiring reading text in images (OCR + reasoning). Answers are text strings visible in the image.
- **VizWiz**: https://vizwiz.org/ — 31K VQA pairs from blind users. Real-world, noisy, sometimes unanswerable. Tests practical utility.
- **TallyQA**: https://github.com/manoja328/TallyQA — 287K counting questions. Simple and complex counts.
- **Visual7W**: 47K QA pairs with pointing answers (which object in the image?).
- **COCO-QA**: Automatically generated QA pairs from COCO captions. 78K train, 38K test.

## Task Format

**Open-ended VQA**:
- Input: Image + question (e.g., "How many people are sitting?" over an image of a park bench).
- Output: Short text answer (e.g., "3" or "three").
- Verification: VQAv2 soft accuracy or exact match after normalization.

**Multiple choice VQA**:
- Input: Image + question + answer choices.
- Output: Selected choice.
- Verification: Exact match.

**Yes/No VQA**:
- Input: Image + yes/no question (e.g., "Is the dog wearing a hat?").
- Output: "yes" or "no".
- Verification: Exact match.

**Compositional VQA (CLEVR-style)**:
- Input: Synthetic image + compositional question (e.g., "Are there more red cubes than blue spheres?").
- Output: Answer.
- Verification: Exact match against programmatically generated ground truth.

## Difficulty Curriculum

1. **Yes/No questions** (easy): Binary answers. VQAv2 yes/no subset.
2. **Color/attribute questions** (easy): "What color is the car?" Single-attribute recognition.
3. **Object identification** (easy-medium): "What is the person holding?" Requires recognition.
4. **Counting (simple)** (medium): "How many dogs?" Requires detection + counting. TallyQA simple subset.
5. **Spatial reasoning** (medium): "What is to the left of the lamp?" GQA spatial questions.
6. **Compositional questions** (medium-hard): "How many red objects are smaller than the blue cylinder?" CLEVR-style.
7. **Counting (complex)** (hard): Counting objects with multiple conditions. TallyQA complex subset.
8. **Knowledge-requiring VQA** (hard): "What sport is being played?" requires world knowledge. OK-VQA.
9. **Text reading + reasoning** (hard): "What time does the store close?" from a photo of a sign. TextVQA.
10. **Multi-hop visual reasoning** (very hard): Questions requiring multiple steps of inference over image content.

## Limitations & Risks

- **Answer ambiguity**: Many VQA questions have legitimately multiple correct answers. "What is the man doing?" could be "standing", "waiting", "looking at phone" — all correct. The VQAv2 soft accuracy metric partially addresses this, but it is imperfect.
- **Short answer bias**: VQA models are biased toward short, frequent answers. Training with exact-match reward reinforces this bias. Rare but correct answers may be penalized.
- **Language priors**: Models can exploit dataset biases (e.g., answering "2" for any "how many" question achieves surprisingly high accuracy). This is a reward hacking vector — the model gets reward without truly understanding the image.
- **Annotation noise**: Human annotations contain errors, ambiguities, and inconsistencies. These become noise in the reward signal.
- **Not all VQA is verifiable**: Open-ended, subjective, or explanation-requiring questions ("Why is this funny?") are not cleanly verifiable. The RLVR-suitable subset is primarily factual, short-answer questions.
- **Image quality variance**: Real-world images have huge quality variation (blur, occlusion, unusual viewpoints). This affects answer verifiability when the ground truth was annotated on a clear view.

## Connections

- [[math-word-problems-visual]] — Visual math is a specialized VQA subtype with numeric verification
- [[chart-understanding]] — Chart QA is VQA applied to chart images
- [[document-ocr-extraction]] — Document QA is VQA applied to document images
- [[spatial-reasoning]] — Spatial VQA questions test spatial reasoning
- [[visual-grounding]] — VQA asks "what?" while grounding asks "where?"
- [[multimodal-reasoning]] — VQA is a building block for complex multimodal reasoning
