---
domain: Dockerfile Optimization
category: Code & Software
verification_type: execution
dataset_scale: ~4M+ Dockerfiles on GitHub
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Dockerfile Optimization

## Overview
Optimize Dockerfiles for image size, build time, layer caching, and security. Verification by building both versions and comparing metrics while maintaining functional equivalence.

## Verification Mechanism
1. Both images build successfully
2. Identical test suites pass against containers from both images (functional equivalence)
3. Optimized image is smaller (measured in bytes)
4. Optionally: faster build time, fewer layers, no root user

## Dataset Sources
- Binnacle dataset (900K Dockerfiles)
- Docker official images
- Hadolint rule database
- GitHub public Dockerfiles (4M+)

## Task Format
**Input**: Dockerfile (unoptimized)
**Output**: Optimized Dockerfile

## Difficulty Curriculum
1. Basic layer ordering optimization
2. Multi-stage builds
3. .dockerignore and cache optimization
4. Security hardening (non-root, minimal base)
5. Complex multi-service builds with shared layers

## Connections
[[infrastructure-as-code]], [[build-configuration]], [[code-optimization]]
