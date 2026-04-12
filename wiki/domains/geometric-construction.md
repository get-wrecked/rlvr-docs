---
domain: Geometric Construction (Euclidean)
category: math-geometry
verification_type: simulation
dataset_scale: 10K+ (construction puzzles + procedural)
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Geometric Construction (Euclidean)

## Overview
Given a geometric goal (construct the incircle of a triangle, bisect an angle, construct a regular pentagon), produce a sequence of compass-and-straightedge operations. Verification: simulate the construction, check the result matches the specification.

## Verification Mechanism
```python
def verify(goal: dict, construction: list[Step]) -> float:
    # Simulate compass-and-straightedge operations
    state = GeometricState()
    for step in construction:
        if step.type == "line_through":
            state.add_line(step.point1, step.point2)
        elif step.type == "circle":
            state.add_circle(step.center, step.radius_point)
        elif step.type == "intersection":
            state.mark_intersection(step.obj1, step.obj2, step.which)
    
    # Check goal conditions
    if goal["type"] == "bisect_angle":
        ray = state.get_last_line()
        angle1 = angle_between(goal["ray1"], ray)
        angle2 = angle_between(ray, goal["ray2"])
        return 1.0 if abs(angle1 - angle2) < 1e-6 else 0.0
    
    elif goal["type"] == "construct_point":
        target = compute_target_point(goal)
        constructed = state.get_last_point()
        return 1.0 if distance(target, constructed) < 1e-6 else 0.0
    
    elif goal["type"] == "construct_circle":
        target = compute_target_circle(goal)
        constructed = state.get_last_circle()
        return 1.0 if circles_match(target, constructed, tol=1e-6) else 0.0
```

## Dataset Sources
- **Euclidea app**: 127 geometric construction puzzles with optimal solutions.
- **GeoGebra exercises**: Construction problems.
- **Geometry textbook exercises**: Compass-and-straightedge problems.
- **AlphaGeometry benchmark**: Geometry problems from Olympiad competitions.
- **Procedural generation**: Generate random point configurations, define construction goals.

## Task Format
- **Input**: "Given triangle ABC, construct the circumcircle" (+ coordinates or diagram)
- **Output**: Sequence of compass/straightedge steps

## Difficulty Curriculum
- Level 1: Midpoint, perpendicular bisector
- Level 3: Angle bisector, parallel lines
- Level 5: Regular polygons, tangent lines to circles
- Level 7: Advanced constructions (inversion, radical axis)
- Level 9: Olympiad-level constructions, minimal-step challenges (Euclidea-style)

## Limitations & Risks
- Numerical precision in simulation. Use exact arithmetic (rational/algebraic numbers) to avoid floating-point issues.
- Some constructions are impossible with compass-and-straightedge (trisecting arbitrary angles). The agent should recognize impossibility.
- Visual input (diagrams) is important for multimodal training.

## Connections
- [[geometry-computation]] — computational geometry
- [[math-competition]] — competition geometry
- [[planning-classical]] — construction as planning
