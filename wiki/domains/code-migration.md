---
domain: Code Migration
category: Code & Software
verification_type: execution
dataset_scale: ~100K+ migration examples from open-source projects
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Code Migration

## Overview
Upgrade code across framework/library versions (React class→hooks, Python 2→3, Java 8→17, Angular.js→Angular). Distinct from code-translation (cross-language) — this is within the same language across versions.

## Verification Mechanism
1. Migrated code compiles/parses with new version's toolchain
2. Test suite passes against migrated code
3. Deprecated API calls are eliminated (grep/AST check)
4. No version-specific warnings/errors remain

## Dataset Sources
- Python 2to3 corpus
- React class-to-hooks examples
- Android Java-to-Kotlin migrations
- jQuery to vanilla JS
- Major framework migration guides with before/after examples

## Task Format
**Input**: Source code + current version + target version
**Output**: Migrated source code

## Difficulty Curriculum
1. Simple API renames (Python 2 print → print())
2. Pattern transformations (class components → hooks)
3. Breaking change adaptation (async/await patterns)
4. Architecture-level migration (monolith → microservice patterns)

## Connections
[[code-translation]], [[code-refactoring]], [[dependency-resolution]]
