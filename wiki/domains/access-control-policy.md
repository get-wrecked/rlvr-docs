---
domain: Access Control Policy Generation
category: Code & Software
verification_type: execution
dataset_scale: ~10K+ from AWS IAM + OPA + K8s RBAC examples
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Access Control Policy Generation

## Overview
Generate access control policies (AWS IAM, OPA Rego, Kubernetes RBAC, Casbin) from natural language requirements. Verification via policy evaluation against decision matrices.

## Verification Mechanism
1. Policy parses without errors
2. Evaluate against test matrix of (principal, action, resource) tuples
3. Compare allow/deny decisions to expected outcomes
4. 100% match required for correctness

## Dataset Sources
- AWS IAM policy examples
- Open Policy Agent test suites
- Kubernetes RBAC examples
- Casbin policy examples
- Azure/GCP IAM documentation

## Task Format
**Input**: Natural language access requirements
**Output**: Policy in target format (IAM JSON, Rego, K8s YAML, Casbin model)

## Difficulty Curriculum
1. Simple RBAC (admin/user roles)
2. Resource-specific permissions
3. Conditional access (time-based, IP-based)
4. ABAC (attribute-based)
5. Cross-account/cross-service delegation

## Connections
[[infrastructure-as-code]], [[config-generation]], [[constraint-programming]]
