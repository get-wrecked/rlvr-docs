---
domain: config-generation
category: format-constrained
verification_type: execution
dataset_scale: ~large (open-source repos + synthetic)
difficulty_range: easy/medium/hard
modality: text | code
status: strong_hypothesis
---

# Configuration File Generation

## Overview

Configuration generation tasks require producing valid configuration files for real-world software systems: nginx, Apache, systemd, Docker, Kubernetes, Terraform, Ansible, GitHub Actions, and many others. Each system has its own syntax, semantics, and validation tools.

This is a high-value practical domain because (1) configuration is a major pain point in software engineering, (2) most config formats have built-in validators that provide exact verification, and (3) the task requires understanding both the format syntax and the semantic meaning of configuration directives.

## Verification Mechanism

**Type: Execution-based (config validation tools)**

Most configuration systems ship with validation commands:

| System | Validation Command |
|---|---|
| nginx | `nginx -t -c /path/to/nginx.conf` |
| Apache | `apachectl configtest` |
| systemd | `systemd-analyze verify service.service` |
| Docker Compose | `docker-compose -f compose.yml config` |
| Kubernetes | `kubectl apply --dry-run=client -f manifest.yaml` or `kubeconform` |
| Terraform | `terraform validate` |
| Ansible | `ansible-lint playbook.yml` + `ansible-playbook --syntax-check` |
| GitHub Actions | `actionlint workflow.yml` |
| ESLint config | Parse as JSON/YAML, validate schema |
| Prometheus | `promtool check config prometheus.yml` |
| Grafana | JSON schema validation for dashboards |

**Verification stages:**
1. **Syntax validity:** Does the config file parse? (0.3 reward)
2. **Semantic validity:** Does the validator accept it? (0.3 reward)
3. **Behavioral correctness:** Does the config achieve the specified behavior? This is harder — requires either running the service or checking specific directives. (0.4 reward)

**Behavioral verification approach:**
```python
def verify_nginx_config(config: str, requirements: dict) -> float:
    # Parse nginx config using crossplane or similar
    parsed = crossplane.parse(config)
    
    checks = []
    if 'listen_port' in requirements:
        checks.append(find_directive(parsed, 'listen') == str(requirements['listen_port']))
    if 'server_name' in requirements:
        checks.append(find_directive(parsed, 'server_name') == requirements['server_name'])
    if 'ssl_enabled' in requirements:
        checks.append(has_directive(parsed, 'ssl_certificate') == requirements['ssl_enabled'])
    if 'proxy_pass' in requirements:
        checks.append(find_directive(parsed, 'proxy_pass') == requirements['proxy_pass'])
    
    return sum(checks) / len(checks) if checks else 0.5
```

**Verification confidence: HIGH for syntax/validation, MEDIUM for behavioral correctness.** Validation tools are exact. Behavioral verification requires checking specific directives, which is doable but requires per-system implementation.

## Dataset Sources

- **GitHub open-source repos:** Millions of configuration files in public repositories. Search by filename (nginx.conf, docker-compose.yml, .github/workflows/*.yml).
- **Official documentation examples:** Every config system's docs contain example configurations.
- **Ansible Galaxy:** Thousands of Ansible roles with configuration templates.
- **Helm charts:** Kubernetes configuration packages with parameterized values.
- **Terraform Registry:** Thousands of Terraform modules with example configurations.
- **Docker Hub:** Dockerfiles and compose files from official images.
- **Infrastructure as Code datasets:** Various IaC datasets on GitHub.
- **Synthetic generation:** Generate requirements ("nginx reverse proxy on port 8080 forwarding to localhost:3000 with SSL") and pair with known-good configurations.

## Task Format

**Input:**
```
Generate an nginx configuration that:
- Listens on port 443 with SSL
- Uses certificate at /etc/ssl/cert.pem and key at /etc/ssl/key.pem
- Proxies all requests to http://localhost:3000
- Sets a 60-second proxy timeout
- Adds CORS headers for https://example.com
```

**Output:**
```nginx
server {
    listen 443 ssl;
    server_name _;
    
    ssl_certificate /etc/ssl/cert.pem;
    ssl_certificate_key /etc/ssl/key.pem;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_read_timeout 60s;
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        
        add_header Access-Control-Allow-Origin https://example.com;
        add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
        add_header Access-Control-Allow-Headers "Content-Type, Authorization";
    }
}
```

**Input (Kubernetes):**
```
Generate a Kubernetes Deployment and Service for a web app:
- Image: myapp:latest
- 3 replicas
- Container port 8080
- Resource limits: 256Mi memory, 500m CPU
- Liveness probe on /health every 10 seconds
- Service type: ClusterIP on port 80
```

## Difficulty Curriculum

1. **Easy:** Simple, single-purpose configs (basic nginx server block, simple Dockerfile, minimal systemd service).
2. **Medium:** Multi-directive configs with specific requirements (nginx with SSL + proxy + caching, Docker Compose with 3 services and volumes, GitHub Actions CI pipeline).
3. **Hard:** Complex configurations (Kubernetes multi-resource manifests, Terraform with modules and variables, Ansible playbooks with conditionals and handlers).
4. **Very hard:** Configuration troubleshooting (given a broken config and error message, fix it), migration between systems (Docker Compose to Kubernetes), security hardening.

## Limitations & Risks

- **Sandboxing required:** Running validation tools (nginx -t, kubectl, terraform) requires installing those tools in the verification environment. Docker containers can provide isolated validation.
- **Version sensitivity:** Configuration syntax changes between software versions (Kubernetes API versions, Terraform provider versions). Must pin versions.
- **Security implications:** Generated configs may have security vulnerabilities (open ports, weak SSL settings, excessive permissions). Verification checks syntax, not security.
- **Behavioral verification gap:** A config can be syntactically valid but not achieve the intended behavior. Full behavioral verification would require running the service — impractical at training scale.
- **System-specific knowledge:** Each config system is essentially its own domain. Breadth of systems adds implementation complexity for the verification pipeline.

## Connections

- [[data-formatting]] — Configuration files are structured data with format constraints.
- [[protocol-compliance]] — Config files must comply with system-specific schemas and rules.
- [[calendar-ical]] — Another domain requiring standards-compliant structured output.
- [[code-generation]] — Configuration generation is a specialized form of code generation.
