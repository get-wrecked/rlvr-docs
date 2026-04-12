---
domain: Computational Geometry
category: Math
verification_type: execution
dataset_scale: ~10K (competition) + unlimited (procedural)
difficulty_range: easy/medium/hard/superhuman
modality: text (potentially multimodal with diagrams)
status: strong_hypothesis
---

## Overview

Computational geometry problems ask the model to compute geometric quantities — distances, areas, angles, volumes, intersection points, convex hull properties, etc. — or to determine geometric relationships (collinearity, concurrency, tangency). The key RLVR advantage: geometric answers are typically exact numbers or expressions that can be verified by independent computation.

Geometry is prominent in math competitions (AMC, AIME, Olympiad) and has a strong computational tradition (computational geometry algorithms). The domain spans from simple coordinate geometry ("find the distance between two points") to deep Olympiad geometry ("prove that four points are concyclic") with the latter requiring either exact numerical computation or formal proof.

This domain is partially validated. Geometry problems appear in MATH, competition benchmarks, and some specialized geometry datasets. However, dedicated RLVR on geometry is underexplored. The main challenge is that many geometry problems involve diagrams, which requires multimodal input.

## Verification Mechanism

**Primary method: Numerical/symbolic computation with tolerance.**

```python
import sympy as sp
from sympy.geometry import Point, Line, Circle, Triangle, Polygon, Segment
import math

def verify_geometry_numeric(problem_type: str, model_answer, gold_answer,
                            tolerance=1e-6) -> float:
    """
    Verify a geometry answer by numerical comparison.
    
    Works for: distance, area, angle, volume, coordinate computations.
    """
    try:
        model_val = parse_numeric(model_answer)
        gold_val = parse_numeric(gold_answer)
    except:
        return 0.0
    
    # Handle exact answers (fractions, surds)
    if isinstance(model_val, sp.Expr) and isinstance(gold_val, sp.Expr):
        if sp.simplify(model_val - gold_val) == 0:
            return 1.0
        # Fall through to numerical comparison
        model_val = float(model_val.evalf())
        gold_val = float(gold_val.evalf())
    
    if abs(model_val - gold_val) < tolerance + tolerance * abs(gold_val):
        return 1.0
    return 0.0


def verify_geometry_construction(problem: dict, model_answer: dict) -> float:
    """
    Verify a geometric construction answer using SymPy geometry.
    
    Example: "Find the circumcenter of triangle with vertices A(0,0), B(4,0), C(0,3)"
    """
    try:
        if problem["type"] == "circumcenter":
            A = Point(*problem["vertices"][0])
            B = Point(*problem["vertices"][1])
            C = Point(*problem["vertices"][2])
            tri = Triangle(A, B, C)
            
            gold = tri.circumcenter
            model_point = Point(*model_answer["point"])
            
            return 1.0 if gold.distance(model_point) < 1e-6 else 0.0
        
        elif problem["type"] == "area":
            vertices = [Point(*v) for v in problem["vertices"]]
            poly = Polygon(*vertices)
            gold_area = float(poly.area)
            model_area = float(model_answer["area"])
            
            return 1.0 if abs(gold_area - model_area) < 1e-6 else 0.0
        
        elif problem["type"] == "intersection":
            # Verify intersection point lies on both objects
            obj1 = construct_object(problem["object1"])
            obj2 = construct_object(problem["object2"])
            model_point = Point(*model_answer["point"])
            
            on_obj1 = obj1.distance(model_point) < 1e-6
            on_obj2 = obj2.distance(model_point) < 1e-6
            
            return 1.0 if on_obj1 and on_obj2 else 0.0
        
        elif problem["type"] == "collinear":
            points = [Point(*p) for p in model_answer["points"]]
            return 1.0 if Point.is_collinear(*points) else 0.0
        
        elif problem["type"] == "concyclic":
            points = [Point(*p) for p in model_answer["points"]]
            return 1.0 if Point.is_concyclic(*points) else 0.0
    
    except Exception:
        return 0.0


def verify_convex_hull(points: list[tuple], model_hull: list[int]) -> float:
    """
    Verify a convex hull computation.
    
    Args:
        points: List of (x, y) points
        model_hull: Indices of hull vertices in CCW order
    """
    from scipy.spatial import ConvexHull
    
    hull = ConvexHull(points)
    gold_hull_set = set(hull.vertices)
    model_hull_set = set(model_hull)
    
    if gold_hull_set == model_hull_set:
        # Also check ordering (CCW)
        hull_points = [points[i] for i in model_hull]
        if is_ccw(hull_points):
            return 1.0
        return 0.5  # Correct vertices, wrong order
    
    return 0.0


def verify_with_geogebra_or_desmos(construction_steps: list, 
                                     expected_property: str) -> float:
    """
    For complex constructions, verify using a geometry engine.
    
    Example: Construct the nine-point circle and verify it passes through
    the midpoints of the sides, the feet of the altitudes, and the
    midpoints of the segments from vertices to orthocenter.
    """
    # Build the construction
    engine = GeometryEngine()
    for step in construction_steps:
        engine.execute(step)
    
    # Check the property
    return 1.0 if engine.check_property(expected_property) else 0.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **MATH** (geometry subset) | ~1,500 | hendrycks/math | Mix of coordinate and synthetic geometry |
| **AMC/AIME geometry** | ~800 | AoPS | Competition geometry with numerical answers |
| **GeoQA** (Chen et al., 2021) | 5,010 | [github.com/chen-judge/GeoQA](https://github.com/chen-judge/GeoQA) | Geometry with diagrams (multimodal) |
| **GeoQA+** | 12,054 | GitHub | Extended version |
| **Geometry3K** (Lu et al., 2021) | 3,002 | [github.com/lupantech/InterGPS](https://github.com/lupantech/InterGPS) | Geometry problems with formal language |
| **UniGeo** (Chen et al., 2022) | 14,541 | GitHub | Unified geometry benchmark |
| **PGPS9K** | 9,022 | GitHub | Plane geometry |
| **OlympiadBench** (geometry subset) | ~1,000 | GitHub | Hard Olympiad geometry |
| **Formalized IMO geometry** | ~200 | Various | Geometry olympiad in Lean4 |
| **Computational geometry textbooks** | ~2K problems | Various | Algorithms-focused problems |
| **Project Euler** (geometry subset) | ~50 | projecteuler.net | Computational geometry challenges |

### Procedural Generation

```python
def generate_triangle_problem(difficulty="medium"):
    """Generate a random triangle geometry problem."""
    # Generate random triangle with nice coordinates
    if difficulty == "easy":
        # Right triangle with integer sides
        a, b = random.choice([(3,4), (5,12), (8,15), (7,24)])
        A, B, C = Point(0, 0), Point(a, 0), Point(0, b)
    elif difficulty == "medium":
        # Random triangle with rational coordinates
        coords = [(random.randint(-10, 10), random.randint(-10, 10)) for _ in range(3)]
        A, B, C = [Point(*c) for c in coords]
    
    tri = Triangle(A, B, C)
    
    # Choose a random question
    questions = [
        ("area", float(tri.area)),
        ("perimeter", float(tri.perimeter.evalf())),
        ("circumradius", float(tri.circumradius.evalf())),
        ("inradius", float(tri.inradius.evalf())),
        ("circumcenter", (float(tri.circumcenter.x), float(tri.circumcenter.y))),
        ("incenter", (float(tri.incenter.x), float(tri.incenter.y))),
        ("orthocenter", (float(tri.orthocenter.x), float(tri.orthocenter.y))),
        ("centroid", (float(tri.centroid.x), float(tri.centroid.y))),
    ]
    
    q_type, answer = random.choice(questions)
    
    return {
        "problem": f"Triangle has vertices A{tuple(A)}, B{tuple(B)}, C{tuple(C)}. Find the {q_type}.",
        "answer": answer,
        "type": q_type
    }

def generate_coordinate_geometry_problem():
    """Generate coordinate geometry problems."""
    problem_type = random.choice([
        "line_intersection", "circle_tangent", "distance_point_line",
        "area_polygon", "reflection", "rotation"
    ])
    
    if problem_type == "line_intersection":
        # Two random lines
        m1, b1 = random.uniform(-5, 5), random.uniform(-10, 10)
        m2, b2 = random.uniform(-5, 5), random.uniform(-10, 10)
        while abs(m1 - m2) < 0.01:  # Ensure not parallel
            m2 = random.uniform(-5, 5)
        
        x = (b2 - b1) / (m1 - m2)
        y = m1 * x + b1
        
        return {
            "problem": f"Find the intersection of y = {m1:.2f}x + {b1:.2f} and y = {m2:.2f}x + {b2:.2f}.",
            "answer": (round(x, 6), round(y, 6))
        }
    # ... other types
```

## Task Format

**Input**: Problem statement, possibly with coordinates or diagram.

```
Problem: Triangle ABC has vertices A(1, 2), B(5, 2), and C(3, 6). 
Find the area of the triangle.

Expected answer: 8
```

```
Problem: A circle passes through points (0, 0), (4, 0), and (0, 3). 
Find the radius of the circle.

Expected answer: 5/2 = 2.5
```

**Output**: Numerical answer with optional work.

## Difficulty Curriculum

| Level | Type | Example | Scale |
|-------|------|---------|-------|
| Easy | Basic coordinate geometry | Distance between two points, midpoint | Unlimited |
| Medium | Triangle/circle properties | Circumradius, area via Heron's, tangent lines | Unlimited |
| Hard | Olympiad-style numerical | AIME geometry (compute a numerical answer) | ~1K |
| Very Hard | 3D geometry, advanced constructions | Volume of intersection of solids | ~500 |
| Superhuman | Olympiad geometry (proof-based) | IMO geometry problems | ~200 (needs formal proof) |

## Limitations & Risks

1. **Diagram dependence**: Many geometry problems are impossible without the accompanying diagram. Text-only input loses critical spatial information. Multimodal models help but add complexity.
2. **Coordinate bashing vs. synthetic geometry**: LLMs may learn to convert everything to coordinates and compute (coordinate bashing), which works but misses deeper geometric understanding. This is actually a valid strategy for RLVR (correct answers are correct), but may limit transfer.
3. **Floating-point precision**: Geometry computations can accumulate floating-point errors. The tolerance must be set carefully, and exact symbolic computation is preferred when possible.
4. **Proof-based problems excluded**: The hardest geometry (Olympiad) requires synthetic proofs, not computation. These cannot be verified by exact match. Formal proof (in Lean4) is the alternative.
5. **3D and higher dimensions**: Most geometry datasets are 2D. 3D geometry is harder for LLMs to reason about textually and has fewer benchmarks.

## Connections

- **math-competition.md**: Geometry is one of the four pillars of competition math (along with algebra, combinatorics, number theory).
- **math-numerical.md**: Coordinate geometry computations are numerical math in geometric clothing.
- **math-formal-proofs.md**: Olympiad geometry proofs can be formalized in Lean4 (e.g., via the `Mathlib.Geometry` library).
- **math-symbolic.md**: Symbolic computation (simplify expressions involving angles, trig identities) is a common subroutine.
- **probability-statistics.md**: Geometric probability problems (e.g., "probability that random point in square is within distance r of center") bridge both domains.
