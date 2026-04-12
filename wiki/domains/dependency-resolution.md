---
domain: Dependency Resolution & Package Management
category: systems
verification_type: constraint
dataset_scale: 10M+ (from npm, PyPI, cargo registries)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Dependency Resolution & Package Management

## Overview
Given a set of package requirements with version constraints, find a compatible set of package versions that satisfies all constraints. This is a real-world constraint satisfaction problem that every developer faces. Verification: check that all version constraints are satisfied.

## Verification Mechanism
```python
from packaging.version import Version
from packaging.specifiers import SpecifierSet

def verify(requirements: list[dict], resolution: dict[str, str]) -> float:
    # Check each requirement is satisfied
    for req in requirements:
        package = req["package"]
        version_spec = SpecifierSet(req["version_constraint"])
        
        if package not in resolution:
            return 0.0  # Missing package
        
        resolved_version = Version(resolution[package])
        if resolved_version not in version_spec:
            return 0.0  # Version doesn't match constraint
    
    # Check transitive dependencies
    for package, version in resolution.items():
        deps = get_package_deps(package, version)
        for dep in deps:
            dep_spec = SpecifierSet(dep["version_constraint"])
            if dep["package"] not in resolution:
                return 0.0
            if Version(resolution[dep["package"]]) not in dep_spec:
                return 0.0
    
    return 1.0  # All constraints satisfied
```

## Dataset Sources
- **npm registry**: 2M+ packages with dependency trees.
- **PyPI**: 500K+ packages with version constraints.
- **crates.io**: Rust packages with semver constraints.
- **Maven Central**: Java packages.
- **Procedural generation**: Generate synthetic package registries with controlled conflicts.
- **Real-world lockfile diffs**: From GitHub repos showing dependency resolution changes.

## Task Format
- **Input**: `{"requirements": [{"package": "A", "version": ">=1.0,<2.0"}, {"package": "B", "version": ">=2.0"}], "registry": {...}}`
- **Output**: `{"A": "1.5.3", "B": "2.1.0", "C": "3.0.1", ...}` (complete resolved set)

## Difficulty Curriculum
- Level 1: Linear dependency chains, no conflicts
- Level 3: Diamond dependencies with compatible versions
- Level 5: Conflicts requiring backtracking
- Level 7: Complex constraint graphs with multiple valid solutions (optimize for freshness)
- Level 9: Real-world dependency hell scenarios (npm left-pad style)

## Limitations & Risks
- Multiple valid resolutions may exist. Accept any valid one (or optimize for a criterion like freshness/minimality).
- Package registries are large. Need to provide relevant subset.
- This is essentially a CSP/SAT problem — connects well to formal methods domains.

## Connections
- [[logic-propositional]] — dependency resolution is a SAT problem
- [[combinatorics-optimization]] — finding optimal resolution
- [[build-configuration]] — build systems depend on this
