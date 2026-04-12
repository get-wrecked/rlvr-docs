---
domain: Face Detection & Recognition
category: vision
verification_type: exact_match
dataset_scale: 13K-32K images (LFW, WIDER Face)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Face Detection & Recognition

## Overview

Face detection locates face bounding boxes in images; face recognition identifies whose face it is. Detection is measured by mean Average Precision (mAP) at various IoU thresholds. Recognition is measured by True Accept Rate at a given False Accept Rate (TAR@FAR), or verification accuracy on pair-matching tasks.

Both tasks have well-established benchmarks with deterministic evaluation. Detection is fully verifiable (bounding box overlap with ground truth). Recognition verification depends on identity labels, which are reliable in curated datasets but noisy in web-scraped ones.

## Verification Mechanism

```python
import numpy as np

def iou(box_a, box_b):
    """Intersection over Union for two bounding boxes [x1, y1, x2, y2]."""
    x1 = max(box_a[0], box_b[0])
    y1 = max(box_a[1], box_b[1])
    x2 = min(box_a[2], box_b[2])
    y2 = min(box_a[3], box_b[3])
    inter = max(0, x2 - x1) * max(0, y2 - y1)
    area_a = (box_a[2] - box_a[0]) * (box_a[3] - box_a[1])
    area_b = (box_b[2] - box_b[0]) * (box_b[3] - box_b[1])
    return inter / (area_a + area_b - inter + 1e-6)

def compute_map(predicted_boxes, gold_boxes, iou_threshold=0.5) -> float:
    """mAP for face detection."""
    # Sort predictions by confidence
    predicted_boxes.sort(key=lambda x: x["confidence"], reverse=True)
    matched = [False] * len(gold_boxes)
    tp, fp = [], []

    for pred in predicted_boxes:
        best_iou, best_idx = 0, -1
        for i, gold in enumerate(gold_boxes):
            cur_iou = iou(pred["box"], gold["box"])
            if cur_iou > best_iou:
                best_iou, best_idx = cur_iou, i
        if best_iou >= iou_threshold and not matched[best_idx]:
            tp.append(1)
            fp.append(0)
            matched[best_idx] = True
        else:
            tp.append(0)
            fp.append(1)

    tp_cum = np.cumsum(tp)
    fp_cum = np.cumsum(fp)
    recall = tp_cum / len(gold_boxes)
    precision = tp_cum / (tp_cum + fp_cum)
    ap = np.trapz(precision, recall) if len(recall) > 0 else 0.0
    return ap

def verify_face_recognition(pred_identity: str, gold_identity: str) -> dict:
    """Verification: does the predicted identity match?"""
    match = pred_identity.strip().lower() == gold_identity.strip().lower()
    return {"correct": match, "reward": 1.0 if match else 0.0}

def verify_face_verification(similarity: float, is_same: bool, threshold: float = 0.5) -> dict:
    """Pair verification: are these two faces the same person?"""
    predicted_same = similarity >= threshold
    correct = predicted_same == is_same
    return {"correct": correct, "reward": 1.0 if correct else 0.0}
```

## Dataset Sources

- **WIDER Face**: 32,203 images with 393,703 annotated faces across 61 event categories. Three difficulty levels (easy, medium, hard based on face size and occlusion). The standard detection benchmark.
- **LFW (Labeled Faces in the Wild)**: 13,233 face images of 5,749 people. Standard verification benchmark: 6,000 pairs (3,000 matched, 3,000 mismatched). Near-saturated (~99.8% accuracy) for modern models.
- **MegaFace**: 1M distractors + 100K probe images for large-scale face recognition. Tests identification at scale.
- **FDDB (Face Detection Data Set and Benchmark)**: 2,845 images with 5,171 faces. Annotated with ellipses.
- **IJB-B/IJB-C (IARPA Janus Benchmark)**: 21K+ images, 7K+ videos of 3,531 subjects. Mixed media (still + video). Current state-of-the-art benchmark.
- **AgeDB**: 16,488 images of 568 subjects with age labels for cross-age recognition.
- **CFP (Celebrities in Frontal-Profile)**: 7,000 images testing frontal-to-profile face verification.
- **VGGFace2**: 3.31M images of 9,131 subjects. Large-scale training set.
- **MS-Celeb-1M**: ~10M images of 100K celebrities. Large but with known label noise.

## Task Format

- **Input (detection)**: An image, return all face bounding boxes.
```
Detect all faces in this image. Return bounding boxes as [x1, y1, x2, y2] with confidence scores.
Image: [crowd_photo.jpg]
```
- **Output (detection)**:
```
[
  {"box": [120, 45, 185, 130], "confidence": 0.98},
  {"box": [300, 80, 350, 145], "confidence": 0.95},
  {"box": [50, 200, 78, 232], "confidence": 0.72}
]
```
- **Input (recognition)**: Face image + gallery of known identities.
```
Identify the person in this face image from the following gallery:
[face_query.jpg]
Gallery: {person_001: [ref1.jpg, ref2.jpg], person_002: [ref1.jpg], ...}
```
- **Output (recognition)**: Identity label.
```
person_001 (confidence: 0.94)
```

## Difficulty Curriculum

- Level 1: Frontal faces, good lighting, no occlusion, FDDB-easy
- Level 2: Slight pose variation (up to 30 degrees), minor occlusion
- Level 3: WIDER Face "easy" subset, multiple faces per image
- Level 4: Small faces (<30px), partial occlusion by objects
- Level 5: WIDER Face "medium", cross-age recognition
- Level 6: Profile faces, extreme lighting, heavy makeup/disguise
- Level 7: WIDER Face "hard", tiny faces in crowds
- Level 8: Cross-modality (NIR to visible, sketch to photo), heavily occluded faces
- Level 9: Large-scale identification (1M+ gallery), adversarial perturbations, deepfake detection

## Limitations & Risks

- **Privacy and ethics**: Face recognition raises serious privacy concerns. Datasets scraped from the web (MS-Celeb, MegaFace) have been criticized for lack of consent. Some datasets have been retracted.
- **Demographic bias**: Recognition accuracy varies significantly by race, gender, and age. Models trained primarily on light-skinned faces perform poorly on darker skin tones. This is a well-documented and serious limitation.
- **Vision-only task**: Requires actual image processing. Text-only models cannot perform face detection or recognition.
- **Dataset noise**: Web-scraped identity labels are noisy (~5-10% label errors in MS-Celeb-1M). Training on noisy labels degrades the RLVR reward signal.
- **Near-saturation on LFW**: LFW accuracy exceeds 99.8% for state-of-the-art models, providing almost no discriminative signal. More challenging benchmarks (IJB-C, cross-age) are needed.
- **Adversarial vulnerability**: Face recognition systems are vulnerable to adversarial patches, glasses, and makeup that can evade or impersonate. This is a security concern beyond RLVR scope.
- **Legal restrictions**: Face recognition is regulated or banned in some jurisdictions. Dataset usage must comply with GDPR, CCPA, and institutional policies.

## Connections

- [[object-detection]] — Face detection is a special case of object detection
- [[pose-estimation]] — Face landmarks (eyes, nose, mouth) are a face-specific pose estimation task
- [[emotion-recognition]] — Emotion recognition from faces builds on face detection
- [[medical-image-segmentation]] — Similar localization and classification in images
