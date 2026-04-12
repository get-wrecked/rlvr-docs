---
domain: CI/CD Pipeline Configuration
category: devops
verification_type: execution
dataset_scale: 100K+ (from GitHub Actions marketplace)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# CI/CD Pipeline Configuration

## Overview
Generate CI/CD pipeline configurations (GitHub Actions YAML, GitLab CI, Jenkinsfile) that correctly define build/test/deploy workflows. Verification: parse YAML, validate schema, dry-run the pipeline, check that the dependency graph and stage ordering are correct.

## Verification Mechanism
```python
import yaml
import jsonschema

def verify(platform: str, pipeline_yaml: str, requirements: dict) -> float:
    # Parse YAML
    try:
        config = yaml.safe_load(pipeline_yaml)
    except yaml.YAMLError:
        return 0.0
    
    score = 0.0
    checks = 0
    
    # Schema validation
    checks += 1
    if platform == "github_actions":
        schema = load_github_actions_schema()
        try:
            jsonschema.validate(config, schema)
            score += 1
        except jsonschema.ValidationError:
            pass
    
    # Required stages present
    for stage in requirements.get("required_stages", []):
        checks += 1
        if stage_exists(config, stage, platform):
            score += 1
    
    # Correct triggers
    if "triggers" in requirements:
        checks += 1
        if triggers_match(config, requirements["triggers"], platform):
            score += 1
    
    # Dependency ordering (build before test before deploy)
    if "stage_order" in requirements:
        checks += 1
        if check_stage_order(config, requirements["stage_order"], platform):
            score += 1
    
    # Uses correct actions/images
    if "required_tools" in requirements:
        for tool in requirements["required_tools"]:
            checks += 1
            if uses_tool(config, tool):
                score += 1
    
    # Caching configured (if required)
    if requirements.get("requires_caching"):
        checks += 1
        if has_caching(config, platform):
            score += 1
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **GitHub Actions Marketplace**: 20K+ published actions.
- **Public GitHub repos**: Millions of `.github/workflows/*.yml` files.
- **GitLab CI templates**: Official template library.
- **Jenkins shared libraries**: Community Jenkinsfile examples.
- **CircleCI orbs**: Reusable config packages.
- **Procedural generation**: Generate project types + requirements, expected pipeline config.

## Task Format
- **Input**: "Create a GitHub Actions workflow for a Python project that: runs tests on push to main, lints with ruff, builds Docker image on tag, deploys to AWS ECR"
- **Output**: Valid `.github/workflows/ci.yml` YAML

## Difficulty Curriculum
- Level 1: Simple single-job workflow (checkout + test)
- Level 3: Multi-job with dependencies, caching
- Level 5: Matrix builds, conditional deploys, secrets management
- Level 7: Complex multi-environment deploy with approval gates
- Level 9: Full GitOps pipeline with rollback, canary deploys

## Connections
- [[infrastructure-as-code]] — infrastructure provisioning
- [[build-configuration]] — build system config
- [[config-generation]] — general YAML/config generation
- [[code-generation]] — pipeline code is code
