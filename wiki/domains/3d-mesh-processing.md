---
domain: 3D Mesh & Point Cloud Processing
category: computational-geometry
verification_type: execution
dataset_scale: 1M+ (ShapeNet, ModelNet, ABC)
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# 3D Mesh & Point Cloud Processing

## Overview
Process and transform 3D geometric data: mesh simplification, remeshing, Boolean operations, surface reconstruction from point clouds, mesh repair, watertight conversion. Verification: geometric metrics (Hausdorff distance, chamfer distance, mesh quality measures).

## Verification Mechanism
```python
import trimesh
import numpy as np

def verify(task_type: str, input_mesh: trimesh.Trimesh, 
           output_mesh: trimesh.Trimesh, specs: dict) -> float:
    if task_type == "simplification":
        # Check face count matches target
        if abs(output_mesh.faces.shape[0] - specs["target_faces"]) > specs["target_faces"] * 0.05:
            return 0.0
        # Check geometric fidelity (Hausdorff distance)
        hausdorff = compute_hausdorff(input_mesh, output_mesh)
        max_allowed = specs.get("max_hausdorff", input_mesh.bounding_box.extents.max() * 0.01)
        return max(0, 1 - hausdorff / max_allowed)
    
    elif task_type == "repair":
        # Check mesh is manifold, watertight, no degenerate faces
        score = 0.0
        checks = 0
        checks += 1
        if output_mesh.is_watertight:
            score += 1
        checks += 1
        if output_mesh.is_volume:
            score += 1
        checks += 1
        if len(trimesh.repair.broken_faces(output_mesh)) == 0:
            score += 1
        return score / checks
    
    elif task_type == "surface_reconstruction":
        # Chamfer distance between reconstructed surface and ground truth
        pts_pred = output_mesh.sample(10000)
        pts_gt = specs["ground_truth_mesh"].sample(10000)
        cd = chamfer_distance(pts_pred, pts_gt)
        return max(0, 1 - cd / specs["cd_threshold"])
    
    elif task_type == "boolean_operation":
        # Check result matches expected (computed by reference implementation)
        expected = specs["expected_mesh"]
        iou = mesh_iou(output_mesh, expected)
        return iou
```

## Dataset Sources
- **ShapeNet**: 51K 3D models across 55 categories.
- **ModelNet**: 12K CAD models in 40 categories.
- **ABC Dataset**: 1M CAD models (B-rep format).
- **Thingiverse**: 2M+ user-created 3D models.
- **ScanNet**: Real-world 3D scans.
- **Open3D test data**: Standard test meshes.
- **Stanford 3D scanning repository**: Classic 3D scan datasets (Bunny, Dragon).

## Task Format
- **Input**: 3D mesh file + "Simplify this mesh to 5000 faces while preserving sharp edges"
- **Output**: Simplified mesh (or code that produces it)

## Difficulty Curriculum
- Level 1: Basic mesh queries (vertex count, bounding box, volume)
- Level 3: Mesh simplification, smoothing
- Level 5: Boolean operations, surface reconstruction
- Level 7: Remeshing with quality constraints, mesh parameterization
- Level 9: Topology-preserving operations, quad meshing

## Connections
- [[cad-modeling]] — 3D model generation
- [[3d-scene-understanding]] — 3D perception
- [[topology-computation]] — topological properties of meshes
- [[geometry-computation]] — computational geometry
