---
domain: Medical Image Segmentation
category: medical-vision
verification_type: exact_match
dataset_scale: 1K-10K volumes (MSD, BraTS, LiTS)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Medical Image Segmentation

## Overview

Medical image segmentation delineates anatomical structures (organs, vessels, bones) and pathologies (tumors, lesions, hemorrhages) in CT, MRI, X-ray, and ultrasound images. The output is a per-pixel (or per-voxel in 3D) label map assigning each pixel to an anatomical class or background.

RLVR verification is strong: the Dice coefficient (and its variants) provides a robust, well-understood metric comparing predicted segmentations against expert-annotated ground truth. Medical image segmentation is one of the most active areas in applied AI, with abundant benchmark datasets and standardized evaluation protocols.

## Verification Mechanism

```python
import numpy as np

def dice_coefficient(predicted: np.ndarray, gold: np.ndarray, class_id: int) -> float:
    """
    Dice coefficient for a single class.
    predicted, gold: (H, W) or (D, H, W) label maps
    """
    pred_mask = (predicted == class_id)
    gold_mask = (gold == class_id)

    intersection = np.sum(pred_mask & gold_mask)
    sum_masks = np.sum(pred_mask) + np.sum(gold_mask)

    if sum_masks == 0:
        return 1.0  # both empty = perfect agreement
    return 2.0 * intersection / sum_masks

def hausdorff_distance_95(pred_mask: np.ndarray, gold_mask: np.ndarray,
                           voxel_spacing: tuple = (1, 1, 1)) -> float:
    """95th percentile Hausdorff distance for surface accuracy."""
    from scipy.ndimage import distance_transform_edt
    pred_border = get_surface_points(pred_mask)
    gold_border = get_surface_points(gold_mask)
    # Compute directed distances and take 95th percentile
    # (implementation details omitted for brevity)
    return hd95

def verify_segmentation(predicted: np.ndarray, gold: np.ndarray,
                         class_ids: list) -> dict:
    """Multi-class segmentation verification."""
    dice_scores = {}
    for cid in class_ids:
        dice_scores[cid] = dice_coefficient(predicted, gold, cid)

    mean_dice = np.mean(list(dice_scores.values()))

    return {
        "per_class_dice": dice_scores,
        "mean_dice": mean_dice,
        "reward": mean_dice
    }
```

## Dataset Sources

- **Medical Segmentation Decathlon (MSD)**: 10 tasks across 10 organs/pathologies (brain tumors, heart, liver, hippocampus, prostate, lung, pancreas, hepatic vessel, spleen, colon). ~2,600 total training volumes. The standard multi-organ benchmark.
- **BraTS (Brain Tumor Segmentation)**: ~2,000 multi-parametric MRI brain scans with tumor annotations (enhancing tumor, edema, necrotic core). Annual challenge since 2012. 4 MRI modalities per case.
- **ACDC (Automated Cardiac Diagnosis)**: 150 cardiac MRI exams with annotations of left/right ventricle and myocardium across cardiac cycle.
- **LiTS (Liver Tumor Segmentation)**: 201 CT scans with liver and tumor annotations. Part of ISBI challenge.
- **KiTS (Kidney Tumor Segmentation)**: 300 CT scans with kidney and tumor annotations.
- **AMOS**: 500 CT + 100 MRI scans with 15-organ annotations. Large multi-organ dataset.
- **TotalSegmentator**: 1,204 CT scans with annotations for 104 anatomical structures. Most comprehensive single dataset.
- **BTCV (Beyond the Cranial Vault)**: 50 CT scans with 13 abdominal organ annotations.
- **SegTHOR**: 60 CT scans with thoracic organ annotations (heart, trachea, aorta, esophagus).

## Task Format

- **Input**: Medical image (2D slice or 3D volume) with segmentation target specified.
```
Segment the liver and liver tumors in this abdominal CT scan.
Label each voxel as: 0=background, 1=liver, 2=tumor.

Image: [ct_volume.nii.gz] (512x512x200 voxels)
```
- **Output**: Label map of the same dimensions.
```
Segmentation mask: (512, 512, 200) array with values {0, 1, 2}
Liver Dice: 0.95, Tumor Dice: 0.72
```

## Difficulty Curriculum

- Level 1: Large, well-defined organs with high contrast (liver, spleen in CT)
- Level 2: Smaller but regular structures (kidneys, heart chambers)
- Level 3: Organs with variable shape (pancreas, stomach)
- Level 4: Tumors with clear boundaries and strong enhancement
- Level 5: Tumors with diffuse boundaries, infiltrating surrounding tissue
- Level 6: Small structures (<1cm) or thin structures (blood vessels, nerve fibers)
- Level 7: Multi-organ segmentation (15+ classes simultaneously)
- Level 8: Cross-modality (train on CT, test on MRI), domain shift between scanners
- Level 9: Rare pathologies with minimal training data, extremely heterogeneous tumors, pediatric anatomy

## Limitations & Risks

- **Annotation variability**: Inter-expert agreement is often only 0.85-0.90 Dice even for "clear" structures. Tumor boundaries are especially subjective. The gold standard itself has significant noise.
- **Vision-only task**: Medical image segmentation requires processing actual images. Text-only models cannot perform this task. This domain is for vision-capable models or text-based reasoning about segmentation results.
- **Clinical risk**: Incorrect segmentations can lead to wrong treatment decisions. RLVR-trained models for clinical use require extensive validation beyond benchmark metrics.
- **Class imbalance**: Tumors may occupy <1% of the image volume. Dice handles this better than pixel accuracy, but training signal for rare/small structures is weak.
- **Data access restrictions**: Many medical datasets require IRB approval, data use agreements, or institutional access. MIMIC, BTCV, and challenge datasets are relatively accessible; hospital-scale data is not.
- **3D processing**: Medical images are volumetric (3D). Processing full volumes requires substantial memory and computation that 2D-only architectures cannot handle.
- **Domain specificity**: A model trained on abdominal CT transfers poorly to brain MRI. Cross-anatomy, cross-modality generalization is a major open challenge.

## Connections

- [[ecg-biosignal]] — Both are medical AI domains requiring expert annotations as ground truth
- [[medical-coding]] — Segmentation findings inform clinical coding decisions
- [[stereo-depth]] — Similar dense prediction formulation (per-pixel output)
- [[pose-estimation]] — Anatomical landmark detection is the medical analogue of body keypoint detection
