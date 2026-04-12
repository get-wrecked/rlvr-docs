---
domain: Stereo Depth Estimation
category: vision-3d
verification_type: exact_match
dataset_scale: 400-39K pairs (KITTI, SceneFlow)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Stereo Depth Estimation

## Overview

Stereo depth estimation computes per-pixel depth (or disparity) from a pair of images captured by horizontally offset cameras. Given the left and right images, the system must find corresponding pixels and compute the horizontal displacement (disparity), which is inversely proportional to depth. This is the computational analogue of human binocular vision.

RLVR verification is solid: predicted disparity maps are compared pixel-by-pixel against ground truth from LiDAR sensors (real scenes) or rendering engines (synthetic scenes). Error metrics are well-established and deterministic.

## Verification Mechanism

```python
import numpy as np

def verify_stereo_depth(predicted_disp: np.ndarray, gold_disp: np.ndarray,
                         valid_mask: np.ndarray = None) -> dict:
    """
    predicted_disp: (H, W) disparity map in pixels
    gold_disp: ground truth disparity
    valid_mask: (H, W) boolean mask for pixels with valid ground truth
    """
    if valid_mask is None:
        valid_mask = gold_disp > 0  # disparity=0 usually means invalid

    pred = predicted_disp[valid_mask]
    gold = gold_disp[valid_mask]

    # Absolute error
    abs_err = np.abs(pred - gold)

    # D1-all: percentage of pixels with error > 3px AND > 5% of true disparity
    outlier = (abs_err > 3.0) & (abs_err > 0.05 * gold)
    d1_all = outlier.mean() * 100  # percentage

    # EPE (End-Point Error): mean absolute disparity error
    epe = abs_err.mean()

    # Threshold accuracies
    acc_1px = (abs_err < 1.0).mean() * 100
    acc_3px = (abs_err < 3.0).mean() * 100

    reward = max(0, 1.0 - d1_all / 100.0)  # 1 - outlier rate

    return {
        "d1_all": d1_all,
        "epe": epe,
        "acc_1px": acc_1px,
        "acc_3px": acc_3px,
        "reward": reward
    }
```

## Dataset Sources

- **KITTI Stereo 2015**: 400 training pairs with sparse LiDAR ground truth from real driving scenes. 200 test pairs (evaluation server). The primary real-world benchmark.
- **KITTI Stereo 2012**: 389 training pairs, earlier version with different evaluation protocol.
- **SceneFlow**: 39,000+ synthetic stereo pairs across three sub-datasets:
  - FlyingThings3D: random objects in flight (22K pairs)
  - Monkaa: animated short film scenes (8.6K pairs)
  - Driving: realistic driving simulation (4.4K pairs)
  Dense ground truth from rendering. Standard pretraining dataset.
- **Middlebury Stereo**: 33 high-resolution pairs with very precise ground truth from structured light. Small but gold-standard quality.
- **ETH3D**: 47 stereo pairs (training + test) with high-accuracy LiDAR ground truth in indoor/outdoor scenes.
- **DrivingStereo**: 182K real-world driving stereo pairs with LiDAR ground truth across weather conditions.
- **Tartanair**: 1M+ stereo pairs from diverse simulated environments (indoor, outdoor, nature, urban).

## Task Format

- **Input**: Left and right stereo images.
```
Given a stereo image pair (left and right cameras with known baseline),
compute the disparity map for the left image.
Camera baseline: 0.54m, focal length: 721px.

Left image: [left.png]
Right image: [right.png]
```
- **Output**: Dense disparity map (pixels) or depth map (meters).
```
Disparity map: (H, W) array where disp[y, x] = horizontal displacement in pixels
Depth = baseline * focal_length / disparity
Example: disp[200, 300] = 45.2 pixels -> depth = 0.54 * 721 / 45.2 = 8.6 meters
```

## Difficulty Curriculum

- Level 1: Synthetic scenes with strong texture, no occlusion, small disparity range
- Level 2: Synthetic scenes with moderate complexity (SceneFlow)
- Level 3: Real-world scenes with textured surfaces, good lighting (easy KITTI frames)
- Level 4: Scenes with thin structures (poles, fences) and fine details
- Level 5: Occlusion boundaries, half-occluded regions
- Level 6: Textureless regions (walls, sky), repetitive patterns (brick, tile)
- Level 7: Reflective/transparent surfaces (glass, water, car paint)
- Level 8: Adverse conditions (rain, fog, low light, overexposure)
- Level 9: Very large disparity range (near+far objects), cross-domain generalization (indoor trained, outdoor tested)

## Limitations & Risks

- **Sparse real ground truth**: KITTI LiDAR provides ground truth for only ~30% of pixels, biased toward surfaces at LiDAR-visible depths. Sky, transparent objects, and distant surfaces have no ground truth.
- **Vision-only task**: Like optical flow, stereo depth is fundamentally a pixel-level vision task. Text-only LLMs cannot perform it. The domain requires vision-capable models.
- **Domain gap**: Models trained on SceneFlow (synthetic) lose significant accuracy on KITTI (real). Domain adaptation or real-world finetuning is essential.
- **Ill-posed regions**: Textureless surfaces, reflections, and repetitive patterns create ambiguous stereo matching. Multiple disparities are equally plausible, making the "correct" answer undefined.
- **Computational cost**: Stereo matching at high resolution is computationally expensive. Cost volume construction scales as O(H * W * D_max).
- **Rectification assumption**: Standard stereo methods assume epipolar-rectified images. Real cameras may have calibration errors that degrade ground truth quality.

## Connections

- [[optical-flow]] — Optical flow is a generalization of stereo matching (2D motion vs. horizontal-only disparity)
- [[pose-estimation]] — 3D pose estimation benefits from depth maps
- [[3d-scene-understanding]] — Stereo depth is a building block for 3D scene reconstruction
- [[medical-image-segmentation]] — Stereo/depth techniques apply to 3D medical imaging
