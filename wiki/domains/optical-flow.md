---
domain: Optical Flow
category: vision
verification_type: exact_match
dataset_scale: 1K-22K frame pairs (Sintel, FlyingChairs)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Optical Flow

## Overview

Optical flow estimation computes the per-pixel 2D motion field between consecutive video frames: for every pixel in frame t, where did it move to in frame t+1? The output is a dense displacement field (u, v) for each pixel. Optical flow is foundational for video understanding, action recognition, video stabilization, and autonomous driving.

RLVR verification is strong because ground truth flow can be computed exactly for synthetic scenes (rendering engines know the true 3D motion) and approximately for real scenes (via LiDAR, stereo, or interpolation). End-Point Error (EPE) -- the Euclidean distance between predicted and ground truth flow vectors -- is the standard metric.

## Verification Mechanism

```python
import numpy as np

def compute_epe(predicted_flow: np.ndarray, gold_flow: np.ndarray,
                valid_mask: np.ndarray = None) -> float:
    """
    End-Point Error: average Euclidean distance between predicted and gold flow vectors.
    predicted_flow: (H, W, 2) array of (u, v) displacements
    gold_flow: same shape
    valid_mask: (H, W) boolean mask for valid pixels (optional)
    """
    diff = predicted_flow - gold_flow
    epe_per_pixel = np.sqrt(diff[:, :, 0]**2 + diff[:, :, 1]**2)

    if valid_mask is not None:
        epe = epe_per_pixel[valid_mask].mean()
    else:
        epe = epe_per_pixel.mean()

    return epe

def verify_optical_flow(predicted_flow, gold_flow, valid_mask=None) -> dict:
    """Full evaluation with multiple metrics."""
    epe = compute_epe(predicted_flow, gold_flow, valid_mask)

    # Fl-all: percentage of pixels with EPE > 3px AND > 5% of gold magnitude
    diff = predicted_flow - gold_flow
    epe_map = np.sqrt(diff[:, :, 0]**2 + diff[:, :, 1]**2)
    gold_mag = np.sqrt(gold_flow[:, :, 0]**2 + gold_flow[:, :, 1]**2)

    outlier = (epe_map > 3.0) & (epe_map > 0.05 * gold_mag)
    if valid_mask is not None:
        fl_all = outlier[valid_mask].mean() * 100  # percentage
    else:
        fl_all = outlier.mean() * 100

    # Reward: inverse EPE (lower EPE = higher reward), capped
    reward = max(0, 1.0 - epe / 10.0)  # reward=0 when EPE>=10px

    return {
        "epe": epe,
        "fl_all": fl_all,
        "reward": reward
    }
```

## Dataset Sources

- **MPI Sintel**: 1,064 training frame pairs from an animated short film, with ground truth flow from rendering. Two versions: "clean" (no motion blur/fog) and "final" (with atmospheric effects). The standard academic benchmark.
- **KITTI Flow 2015**: 400 frame pairs from real-world driving scenes. Ground truth from LiDAR + camera calibration. Sparse ground truth (only ~30% of pixels have valid flow). The standard real-world benchmark.
- **KITTI Flow 2012**: 389 training pairs, earlier version.
- **FlyingChairs**: 22,872 synthetic image pairs of chairs composited on random backgrounds with known 2D motion. Used for pretraining.
- **FlyingThings3D**: 25K synthetic scene pairs with dense 3D motion. Part of the SceneFlow dataset family.
- **Spring**: 6K high-resolution frames from a realistic synthetic scene. Modern benchmark with challenging content.
- **HD1K**: 1,047 frames from real-world driving in varied conditions.
- **Middlebury Flow**: 72 pairs with very precise ground truth. Small but high quality.

## Task Format

- **Input**: Two consecutive video frames (as images or pixel arrays).
```
Given two consecutive video frames, estimate the optical flow field.
Return per-pixel (u, v) displacement vectors from frame 1 to frame 2.
Frame 1: [image_t.png]
Frame 2: [image_t+1.png]
```
- **Output**: Dense displacement field.
```
Flow field: (H, W, 2) array where flow[y, x] = (u, v) displacement in pixels
Example: flow[100, 200] = (3.5, -1.2) means pixel (200, 100) moved 3.5px right, 1.2px up
```

## Difficulty Curriculum

- Level 1: Synthetic scenes with simple translational motion (FlyingChairs-easy)
- Level 2: Synthetic scenes with rotation and scaling
- Level 3: Small displacements (<5px) in real-world scenes
- Level 4: Moderate motion (5-20px) with texture-rich regions
- Level 5: Large displacements (>20px), fast-moving objects
- Level 6: Occlusion boundaries (objects appearing/disappearing)
- Level 7: Motion blur, defocus, and atmospheric effects (Sintel "final")
- Level 8: Textureless/repetitive regions (aperture problem), transparent/reflective surfaces
- Level 9: Real-world scenes with extreme motion, dynamic weather, lighting changes (KITTI)

## Limitations & Risks

- **Synthetic-real domain gap**: Most training data is synthetic (FlyingChairs, Sintel). Models trained on synthetic data often perform poorly on real scenes. The domain gap is a fundamental challenge.
- **Sparse real-world ground truth**: KITTI provides ground truth for only ~30% of pixels (from LiDAR). Evaluation is incomplete and biased toward nearby surfaces.
- **Text-based model limitations**: Optical flow is inherently a pixel-level vision task. Text-only LLMs cannot perform optical flow estimation. This domain requires vision-capable models or is limited to flow analysis/interpretation tasks for text models.
- **Scale of output**: A single flow prediction is a (H, W, 2) tensor with millions of values. Representing this in text is impractical. RLVR for this domain likely operates on compressed representations or summary statistics.
- **Evaluation at occlusion boundaries**: Flow is undefined at occlusion boundaries (pixels that appear/disappear). Different benchmarks handle this differently, affecting metrics.
- **Brightness constancy assumption**: Classical flow assumes pixel intensity is preserved across frames. This breaks under lighting changes, shadows, and specular reflections.

## Connections

- [[stereo-depth]] — Both are dense correspondence problems; stereo is horizontal flow with epipolar constraint
- [[pose-estimation]] — Temporal pose tracking uses optical flow
- [[video-understanding]] — Flow is a key feature for action recognition and video analysis
- [[object-detection]] — Moving object detection can be derived from optical flow
