---
domain: Pose Estimation
category: vision
verification_type: exact_match
dataset_scale: 250K+ images (COCO-Pose)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Pose Estimation

## Overview

Pose estimation detects the spatial positions of body keypoints (joints) in images or video: head, shoulders, elbows, wrists, hips, knees, ankles, etc. The task can be single-person (locate keypoints of one individual) or multi-person (detect and localize keypoints for all people in a scene). Outputs are (x, y) coordinates for each keypoint, optionally with confidence scores and visibility flags.

RLVR verification uses established metrics: Percentage of Correct Keypoints (PCK) at various thresholds, and Object Keypoint Similarity (OKS) which accounts for keypoint-specific scale and detection difficulty. Both metrics compare predicted coordinates against human-annotated ground truth.

## Verification Mechanism

```python
import numpy as np

def compute_oks(predicted_keypoints, gold_keypoints, bbox_area, sigmas):
    """
    Object Keypoint Similarity (OKS) - standard COCO metric.
    predicted_keypoints: [(x, y, visible), ...] for K keypoints
    gold_keypoints: same format
    sigmas: per-keypoint standard deviations (from COCO)
    bbox_area: area of the person bounding box
    """
    K = len(gold_keypoints)
    oks_sum = 0.0
    visible_count = 0

    for i in range(K):
        gx, gy, gv = gold_keypoints[i]
        px, py, pv = predicted_keypoints[i]
        if gv == 0:  # not labeled
            continue
        visible_count += 1
        dx = px - gx
        dy = py - gy
        dist_sq = dx**2 + dy**2
        s_sq = (2 * sigmas[i])**2 * bbox_area
        oks_sum += np.exp(-dist_sq / (2 * s_sq))

    return oks_sum / visible_count if visible_count > 0 else 0.0

def verify_pose(predicted, gold, bbox_area, sigmas, threshold=0.5) -> dict:
    """Compute OKS and AP-style metrics."""
    oks = compute_oks(predicted, gold, bbox_area, sigmas)

    # PCK: percentage of keypoints within threshold * bbox_diagonal
    bbox_diag = np.sqrt(bbox_area)
    correct = 0
    total = 0
    for i in range(len(gold)):
        gx, gy, gv = gold[i]
        if gv == 0:
            continue
        total += 1
        px, py, _ = predicted[i]
        dist = np.sqrt((px - gx)**2 + (py - gy)**2)
        if dist < threshold * bbox_diag:
            correct += 1

    pck = correct / total if total > 0 else 0.0

    return {
        "oks": oks,
        "pck": pck,
        "reward": oks  # OKS is the primary COCO metric
    }

# COCO keypoint sigmas (17 keypoints)
COCO_SIGMAS = [0.026, 0.025, 0.025, 0.035, 0.035, 0.079, 0.079,
               0.072, 0.072, 0.062, 0.062, 0.107, 0.107, 0.087,
               0.087, 0.089, 0.089]
```

## Dataset Sources

- **COCO Keypoints**: 250K+ person instances with 17 keypoints annotated (nose, eyes, ears, shoulders, elbows, wrists, hips, knees, ankles). Part of MS-COCO. The dominant benchmark.
- **MPII Human Pose**: 40K annotated images, 16 keypoints, 410 activities. Single-person crops with rich activity labels.
- **CrowdPose**: 20K images specifically selected for crowded scenes with occluded people. Tests multi-person pose estimation under occlusion.
- **OCHuman**: 13K images focusing on heavily occluded humans. Challenging subset.
- **AI Challenger (AIC)**: 300K images with 14 keypoints. Large-scale Chinese dataset.
- **PoseTrack**: 1,356 video sequences for multi-person pose tracking over time.
- **Human3.6M**: 3.6M frames of 11 subjects performing 15 activities with 3D joint annotations from motion capture. Gold standard for 3D pose.
- **MPI-INF-3DHP**: 3D pose in diverse indoor/outdoor settings.

## Task Format

- **Input**: An image containing one or more people, with optional bounding box specification.
```
Detect all body keypoints for the person in the bounding box [x1, y1, x2, y2]
in this image. Return 17 COCO keypoints as (x, y, confidence) tuples.
```
- **Output**: Keypoint coordinates.
```
{
  "nose": [234.5, 156.2, 0.95],
  "left_eye": [240.1, 148.3, 0.92],
  "right_eye": [228.9, 149.1, 0.91],
  "left_shoulder": [260.3, 195.8, 0.88],
  ...
}
```

## Difficulty Curriculum

- Level 1: Single person, frontal view, fully visible, simple background
- Level 2: Single person, slight rotation, most keypoints visible
- Level 3: Single person with partial occlusion (behind desk, holding objects)
- Level 4: Multi-person with no overlap, each person well-separated
- Level 5: Person in unusual poses (yoga, dance, sports action)
- Level 6: Multi-person with moderate overlap and partial occlusion
- Level 7: Crowded scenes with severe occlusion (CrowdPose-level)
- Level 8: Extreme poses, motion blur, low resolution, unusual viewpoints
- Level 9: 3D pose estimation from 2D images, cross-domain (artistic renderings, cartoons)

## Limitations & Risks

- **Annotation noise**: Human-annotated keypoints have inherent imprecision (2-5 pixel error). OKS accounts for this via per-keypoint sigmas, but the gold standard is still noisy.
- **Occlusion ambiguity**: When a keypoint is fully occluded, the "correct" position is genuinely uncertain. Different annotators place occluded keypoints differently.
- **Text-based models**: Pose estimation fundamentally requires visual input. For text-only LLMs, this domain is only accessible if keypoint coordinates are provided directly (from a vision system), which makes it a coordinate prediction task rather than true pose estimation.
- **Metric sensitivity**: OKS weighs keypoints by their annotation difficulty (sigmas). Eyes and nose have tight thresholds; hips and knees are more lenient. This can create counterintuitive reward signals.
- **Distribution bias**: COCO is biased toward Western contexts, standing/walking poses, and outdoor scenes. Performance degrades on underrepresented demographics and activities.
- **Real-time vs. accuracy tradeoff**: Many applications need real-time pose estimation. RLVR optimizes for accuracy without runtime constraints.

## Connections

- [[object-detection]] — Pose estimation often builds on person detection as a first stage
- [[medical-image-segmentation]] — Medical pose estimation (anatomical landmarks) uses similar techniques
- [[optical-flow]] — Temporal pose tracking leverages motion estimation
- [[video-understanding]] — Action recognition often uses pose as an intermediate representation
