---
domain: CAD Modeling / 3D Geometry Generation
category: engineering
verification_type: diff
dataset_scale: 1M+ (Thingiverse, GrabCAD, ABC dataset)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# CAD Modeling / 3D Geometry Generation

## Overview
Generate 3D CAD models (OpenSCAD code, STEP files, mesh descriptions) from natural language or image specifications. Verification: render the model and compare geometrically to reference, or check dimensional/topological constraints.

## Verification Mechanism
```python
def verify(spec: dict, model_code: str) -> float:
    # Parse and render the model
    try:
        mesh = render_openscad(model_code)  # or parse STEP/STL
    except RenderError:
        return 0.0
    
    score = 0.0
    checks = 0
    
    # Dimensional checks
    if "dimensions" in spec:
        bbox = mesh.bounding_box()
        for dim, (expected, tolerance) in spec["dimensions"].items():
            checks += 1
            actual = getattr(bbox, dim)
            if abs(actual - expected) <= tolerance:
                score += 1
    
    # Topological checks
    if "num_holes" in spec:
        checks += 1
        if mesh.genus() == spec["num_holes"]:
            score += 1
    
    # Volume/mass check
    if "volume" in spec:
        checks += 1
        if abs(mesh.volume() - spec["volume"]) / spec["volume"] < 0.05:
            score += 1
    
    # Visual similarity to reference
    if "reference_mesh" in spec:
        checks += 1
        iou = mesh_iou(mesh, spec["reference_mesh"])
        score += iou  # 0.0 to 1.0
    
    # Chamfer distance (point cloud comparison)
    if "reference_points" in spec:
        checks += 1
        cd = chamfer_distance(mesh.sample_points(10000), spec["reference_points"])
        score += max(0, 1 - cd / spec["cd_threshold"])
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **Thingiverse**: 2M+ 3D models with descriptions.
- **GrabCAD**: 4M+ CAD files from engineers.
- **ABC Dataset**: 1M CAD models (mechanical parts).
- **ShapeNet**: 51K 3D models across 55 categories.
- **Fusion 360 Gallery**: Parametric CAD models with construction history.
- **OpenSCAD examples**: Parametric code-based 3D models.
- **Text2CAD**: Benchmark for text-to-CAD generation.

## Task Format
- **Input**: "Create a cylindrical container with an inner diameter of 50mm, wall thickness of 3mm, and height of 100mm, with a threaded top"
- **Output**: OpenSCAD code or STEP file

## Difficulty Curriculum
- Level 1: Basic primitives (box, cylinder, sphere with dimensions)
- Level 3: Boolean operations (union, difference, intersection)
- Level 5: Parametric designs, threads, fillets
- Level 7: Assemblies, moving parts, tolerances
- Level 9: Complex organic shapes, topology optimization

## Limitations & Risks
- Geometric comparison is approximate. Chamfer distance and mesh IoU are good but not perfect.
- OpenSCAD rendering can be slow for complex models.
- Natural language specifications are often ambiguous — need precise dimensional specs for reliable verification.

## Connections
- [[svg-generation]] — 2D equivalent
- [[engineering-optimization]] — structural optimization of 3D models
- [[html-css-generation]] — code-to-visual generation
