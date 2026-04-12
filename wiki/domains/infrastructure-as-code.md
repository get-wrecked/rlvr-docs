---
domain: Infrastructure as Code
category: Code & Software
verification_type: execution-based (syntax validation + dry-run/plan + optional apply-and-test)
dataset_scale: 1K–20K configurations
difficulty_range: single-resource definitions → multi-service architectures with networking and IAM
modality: text-to-config (Terraform, Docker, Kubernetes, Ansible)
status: early-stage
---

## Overview

Infrastructure-as-Code (IaC) generation tasks the model with producing configuration files — Terraform HCL, Dockerfiles, Kubernetes YAML, Ansible playbooks, CloudFormation templates — that define cloud infrastructure meeting specified requirements. Verification is multi-layered: syntax validation, static analysis, dry-run planning (e.g., `terraform plan`), and optionally deploying to a test environment and running integration checks.

This domain is an attractive but challenging RLVR target: (a) syntax and schema validators provide a fast first-pass verifier, (b) dry-run tools like `terraform plan` verify resource correctness without actual deployment, (c) the practical value is enormous — IaC generation is a high-demand developer task. The main challenge is that full verification (actual deployment) is expensive and slow.

## Verification Mechanism

### Multi-stage verification pipeline

```
def verify_iac(task, generated_config, config_type):
    # Stage 1: Syntax validation (fast, ~1 second)
    if config_type == "terraform":
        syntax = run_in_sandbox("terraform fmt -check -", stdin=generated_config)
        if syntax.returncode != 0:
            return 0.0
        validate = run_in_sandbox("terraform validate", cwd=config_dir)
        if validate.returncode != 0:
            return 0.0

    elif config_type == "dockerfile":
        lint = run_in_sandbox(f"hadolint -", stdin=generated_config)
        if lint.returncode != 0 and has_errors(lint.stderr):
            return 0.0

    elif config_type == "kubernetes":
        validate = run_in_sandbox(
            f"kubectl apply --dry-run=client -f -",
            stdin=generated_config
        )
        if validate.returncode != 0:
            return 0.0

    elif config_type == "ansible":
        syntax = run_in_sandbox(
            f"ansible-playbook --syntax-check playbook.yml",
            cwd=config_dir
        )
        if syntax.returncode != 0:
            return 0.0

    # Stage 2: Static analysis / policy checks (~5 seconds)
    policy_result = run_in_sandbox(
        f"checkov -f {config_file} --quiet",  # or tflint, kube-linter
        cwd=config_dir
    )
    policy_score = parse_checkov_score(policy_result)

    # Stage 3: Dry-run / plan (for Terraform, ~10–30 seconds)
    if config_type == "terraform":
        plan = run_in_sandbox(
            "terraform plan -no-color -detailed-exitcode",
            cwd=config_dir,
            timeout=120,
            env={"AWS_REGION": "us-east-1"}  # use localstack or moto
        )
        if plan.returncode == 1:  # error
            return 0.0
        planned_resources = parse_plan(plan.stdout)
        # Verify expected resources are present
        for expected in task.expected_resources:
            if not resource_in_plan(expected, planned_resources):
                return 0.0

    # Stage 4 (optional): Deploy to test environment and verify
    if task.has_integration_test:
        deploy = run_in_sandbox(
            "terraform apply -auto-approve",
            cwd=config_dir,
            env=localstack_env,
            timeout=300
        )
        if deploy.returncode != 0:
            return 0.0
        # Run integration tests
        for test in task.integration_tests:
            result = run_test(test, env=localstack_env)
            if not result.passed:
                cleanup(config_dir, localstack_env)
                return 0.0
        cleanup(config_dir, localstack_env)

    # Composite reward
    return compute_reward(
        syntax_valid=True,
        policy_score=policy_score,
        plan_valid=True,
        integration_passed=True
    )
```

### Docker-specific verification

```
def verify_dockerfile(task, dockerfile_content, test_commands):
    # 1. Build the image
    build = run_in_sandbox(
        f"docker build -t test-image -f- .",
        stdin=dockerfile_content,
        timeout=300  # builds can be slow
    )
    if build.returncode != 0:
        return 0.0

    # 2. Run test commands inside container
    for test in test_commands:
        result = run_in_sandbox(
            f"docker run --rm test-image {test.command}",
            timeout=30
        )
        if result.stdout.strip() != test.expected_output.strip():
            return 0.0

    # 3. Check image properties
    inspect = run_in_sandbox("docker inspect test-image")
    props = json.loads(inspect.stdout)
    if task.max_image_size and props[0]["Size"] > task.max_image_size:
        return 0.5  # penalty for oversized image

    return 1.0
```

Key considerations:

- **LocalStack/moto for AWS simulation**: Real AWS deployment is expensive and slow. LocalStack provides a local mock of AWS services that supports Terraform apply, S3, Lambda, DynamoDB, and more.
- **Minikube/kind for Kubernetes**: Local Kubernetes clusters enable real deployment verification without cloud costs.
- **Security scanning**: Tools like Checkov, tfsec, and kube-bench verify that generated configs follow security best practices (no public S3 buckets, encrypted storage, least-privilege IAM).
- **Idempotency**: IaC should be idempotent. Running `terraform apply` twice should not cause errors or changes.

## Dataset Sources

| Dataset | Size | Type | Notes |
|---------|------|------|-------|
| **Terraform Registry modules** | 10,000+ modules | Terraform | Official and community modules on registry.terraform.io |
| **Awesome-Compose** | 90+ samples | Docker Compose | Docker's official multi-service examples |
| **Kubernetes examples** | 1,000+ | K8s YAML | From kubernetes/examples repo and Helm charts |
| **Ansible Galaxy** | 30,000+ roles | Ansible | Community-contributed automation roles |
| **Pulumi examples** | 500+ | Multiple IaC | Multi-language IaC examples |
| **CloudFormation templates** | 1,000+ | CFN JSON/YAML | AWS-provided reference architectures |
| **IaC-Eval** | Emerging | Multi-format | Academic benchmarks for IaC generation |
| **DevOps-Eval** | Emerging | Multi-tool | Evaluation suite for DevOps tasks |
| **GitHub IaC corpus** | 100K+ repos | All types | Mined Terraform/K8s/Docker configs from public repos |

**Synthetic data generation**: Take existing IaC modules, generate natural-language descriptions of what they provision, and create (description, config) training pairs. Variations can be generated by parametrizing resource counts, regions, naming conventions, etc.

## Task Format

**Input prompt**:
```
Write a Terraform configuration that provisions:
- An AWS VPC with CIDR block 10.0.0.0/16
- Two public subnets in different availability zones
- An internet gateway attached to the VPC
- A security group allowing inbound HTTP (80) and HTTPS (443)
  from anywhere, and all outbound traffic

Use the AWS provider for us-east-1.
```

**Expected output**: Complete Terraform HCL with `provider`, `resource` blocks.

**Verification**:
1. `terraform validate` passes
2. `terraform plan` shows the expected 6 resources (VPC, 2 subnets, IGW, route table, SG)
3. Checkov finds no critical security issues
4. (Optional) `terraform apply` against LocalStack succeeds

## Difficulty Curriculum

| Level | Scope | Example |
|-------|-------|---------|
| 1 | Single resource | One S3 bucket with versioning |
| 2 | Small stack (2–5 resources) | VPC + subnet + security group |
| 3 | Service with dependencies | ECS service with ALB, target group, IAM role |
| 4 | Multi-service architecture | Microservices with RDS, ElastiCache, SQS |
| 5 | Cross-account / multi-region | DR setup with replication across regions |
| 6 | Full platform | Complete platform with CI/CD, monitoring, IAM, networking |

## Limitations & Risks

- **Verification cost**: Full deployment verification (Stage 4) takes minutes per sample and requires LocalStack or similar. Most RL training will rely on Stages 1–3.
- **LocalStack fidelity**: LocalStack doesn't perfectly replicate AWS behavior. Some resources or edge cases may behave differently in real AWS.
- **Cloud provider specificity**: Terraform configs are provider-specific. A model trained on AWS may not generalize to GCP or Azure.
- **Security risks**: Generated IaC might create insecure infrastructure (open security groups, unencrypted data). Static analysis catches common issues but not all.
- **State management**: Terraform state can be complex. Tasks should be scoped to avoid state-related issues.
- **Version drift**: Provider versions and API changes make older configurations invalid. Datasets must be versioned with the provider versions they target.

## Connections

- **Build Configuration** is the local-development counterpart: Makefiles/CMake for building software vs. Terraform for provisioning infrastructure.
- **Shell Commands** are the imperative counterpart: running `aws cli` commands vs. declarative IaC.
- **API Usage** underlies IaC: Terraform ultimately calls cloud provider APIs.
- **Code Repair** applies: fixing broken Terraform configs that fail to apply.
- **Test Generation** for IaC: generating Terratest or InSpec tests to verify infrastructure.
