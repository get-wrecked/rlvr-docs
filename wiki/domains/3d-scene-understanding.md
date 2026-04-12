---
domain: 3D Scene Understanding
category: vision-3d
verification_type: execution
dataset_scale: 1M+ (from 3D scene datasets)
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# 3D Scene Understanding

## Overview
Understand 3D scenes from images or point clouds: depth estimation, 3D object detection, room layout estimation, point cloud segmentation. Verification: compare predictions to ground truth 3D annotations.

## Verification Mechanism
```python
def verify(task_type: str, prediction: Any, ground_truth: Any) -> float:
    if task_type == "depth_estimation":
        pred_depth = np.array(prediction)
        gt_depth = np.array(ground_truth)
        # Absolute relative error
        abs_rel = np.mean(np.abs(pred_depth - gt_depth) / gt_depth)
        return max(0, 1 - abs_rel)
    
    elif task_type == "3d_detection":
        # 3D IoU for each detected object
        ious = []
        for pred_box in prediction:
            best_iou = max(iou_3d(pred_box, gt_box) for gt_box in ground_truth)
            ious.append(best_iou)
        return np.mean(ious) if ious else 0.0
    
    elif task_type == "point_cloud_segmentation":
        pred_labels = np.array(prediction)
        gt_labels = np.array(ground_truth)
        # Mean IoU across classes
        classes = np.unique(gt_labels)
        ious = []
        for c in classes:
            intersection = np.sum((pred_labels == c) & (gt_labels == c))
            union = np.sum((pred_labels == c) | (gt_labels == c))
            ious.append(intersection / max(union, 1))
        return np.mean(ious)
```

## Dataset Sources
- **ScanNet**: 1513 3D scans of indoor scenes with annotations.
- **NYU Depth V2**: 1449 RGBD images with depth and segmentation.
- **KITTI 3D**: Autonomous driving 3D object detection.
- **SUN RGB-D**: 10K RGBD images with 3D annotations.
- **Matterport3D**: 10K panoramic views of indoor environments.
- **S3DIS**: Stanford 3D Indoor Scenes.
- **nuScenes**: 1.4M 3D boxes in autonomous driving.
- **Waymo Open Dataset**: 12M 3D boxes in driving scenes.
- **ShapeNet**: 51K 3D models for shape understanding.

## Task Format
- **Input**: RGB image or point cloud + "Detect all chairs in this room and provide their 3D bounding boxes"
- **Output**: List of 3D bounding boxes (center, dimensions, rotation)

## Difficulty Curriculum
- Level 1: Monocular depth estimation on clean scenes
- Level 3: 3D object detection from single image
- Level 5: Full scene reconstruction from multiple views
- Level 7: Dynamic scene understanding (moving objects)
- Level 9: Novel view synthesis from sparse observations

## Connections
- [[image-segmentation]] — 2D segmentation
- [[spatial-reasoning]] — spatial understanding
- [[cad-modeling]] — 3D representation
- [[robotics-planning]] — robot perception
