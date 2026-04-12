---
domain: Image Segmentation & Object Detection
category: vision
verification_type: execution
dataset_scale: 10M+ (COCO, ADE20K, etc.)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Image Segmentation & Object Detection

## Overview
Given an image, detect objects (bounding boxes) or segment regions (pixel masks). Verification: IoU (Intersection over Union) against ground truth annotations. Well-established computer vision task with mature evaluation metrics.

## Verification Mechanism
```python
def verify(task_type: str, image: np.array, predictions: list, ground_truth: list) -> float:
    if task_type == "detection":
        # Compute mAP (mean Average Precision)
        matches = match_detections(predictions, ground_truth, iou_threshold=0.5)
        ap = compute_average_precision(matches)
        return ap
    
    elif task_type == "segmentation":
        # Compute mIoU
        pred_mask = predictions["mask"]
        gt_mask = ground_truth["mask"]
        intersection = np.logical_and(pred_mask, gt_mask).sum()
        union = np.logical_or(pred_mask, gt_mask).sum()
        iou = intersection / max(union, 1)
        return iou
    
    elif task_type == "counting":
        # Count objects in image
        pred_count = len(predictions)
        gt_count = len(ground_truth)
        return 1.0 if pred_count == gt_count else max(0, 1 - abs(pred_count - gt_count) / gt_count)
```

## Dataset Sources
- **COCO**: 330K images, 80 object categories, instance segmentation.
- **ADE20K**: 25K images, 150 semantic categories.
- **Pascal VOC**: 11K images, 20 categories.
- **LVIS**: 2M annotations, 1000+ categories (long-tail).
- **Open Images**: 9M images, 600 categories.
- **Cityscapes**: 5K urban scene images with pixel-level labels.
- **SA-1B (Segment Anything)**: 1B masks on 11M images.

## Task Format
- **Input**: Image + "Identify all people and their bounding boxes in this image"
- **Output**: List of (class, x1, y1, x2, y2, confidence) tuples

## Difficulty Curriculum
- Level 1: Single large object on clean background
- Level 3: Multiple objects, standard viewpoints
- Level 5: Crowded scenes, occlusion, small objects
- Level 7: Fine-grained categories, unusual viewpoints
- Level 9: Open-vocabulary detection, rare categories

## Limitations & Risks
- IoU thresholds are somewhat arbitrary (0.5 is standard, 0.75 is strict).
- Ground truth annotations have human error (~95% agreement).
- For RLVR, the agent would need to output bounding box coordinates or masks — this is naturally multimodal.

## Connections
- [[visual-question-answering]] — vision understanding
- [[visual-grounding]] — locating objects by description
- [[image-classification]] — classification without localization
