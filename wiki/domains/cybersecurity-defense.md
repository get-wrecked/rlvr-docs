---
domain: Cybersecurity Defense (Detection & Hardening)
category: security
verification_type: execution
dataset_scale: 100K+ (from security benchmarks + CVE databases)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Cybersecurity Defense (Detection & Hardening)

## Overview
Defensive security tasks: write firewall rules, detect intrusions in logs, analyze network traffic for anomalies, harden configurations, write security policies, identify vulnerabilities in code (SAST). Verification: test the defense against known attacks, run security scanners.

## Verification Mechanism
```python
def verify(task_type: str, defense: str, test_cases: list) -> float:
    if task_type == "firewall_rules":
        # Test firewall rules against allowed/blocked traffic
        passed = 0
        for tc in test_cases:
            result = simulate_firewall(defense, tc["packet"])
            if result == tc["expected_action"]:  # "allow" or "deny"
                passed += 1
        return passed / len(test_cases)
    
    elif task_type == "vulnerability_detection":
        # Check if agent identifies known vulnerabilities in code
        found_vulns = parse_vulnerabilities(defense)
        true_vulns = set(tc["vulnerability"] for tc in test_cases)
        precision = len(found_vulns & true_vulns) / max(len(found_vulns), 1)
        recall = len(found_vulns & true_vulns) / max(len(true_vulns), 1)
        return 2 * precision * recall / max(precision + recall, 1e-10)
    
    elif task_type == "config_hardening":
        # Run security scanner on hardened config
        scanner_results = run_security_scanner(defense)
        original_issues = run_security_scanner(test_cases["original_config"])
        
        issues_fixed = len(original_issues) - len(scanner_results)
        new_issues = len(set(scanner_results) - set(original_issues))
        
        if new_issues > 0:
            return 0.0  # Introduced new vulnerabilities
        return issues_fixed / len(original_issues) if original_issues else 1.0
    
    elif task_type == "log_analysis":
        # Identify malicious events in log data
        predictions = parse_predictions(defense)
        return f1_score(predictions, test_cases["ground_truth"])
```

## Dataset Sources
- **CIC-IDS datasets**: Network intrusion detection datasets.
- **UNSW-NB15**: Network traffic dataset with labeled attacks.
- **CVE/NVD database**: 200K+ vulnerabilities with descriptions and patches.
- **OWASP WebGoat/Juice Shop**: Intentionally vulnerable applications.
- **SecBench**: Security benchmark for LLMs.
- **CWE examples**: Common Weakness Enumeration examples.
- **Snort/Suricata rule sets**: IDS rules as training data.
- **AWS/GCP/Azure security best practices**: Configuration hardening guides.

## Task Format
- **Input**: "This Nginx configuration is serving a web app. Identify security issues and provide a hardened version." + config file
- **Output**: List of issues + hardened configuration

## Difficulty Curriculum
- Level 1: Identify common misconfigurations (default passwords, open ports)
- Level 3: Write firewall rules for specific network policies
- Level 5: Analyze logs for attack patterns (brute force, SQLi, etc.)
- Level 7: Code review for subtle vulnerabilities (timing attacks, race conditions)
- Level 9: Design defense-in-depth strategies for complex architectures

## Limitations & Risks
- Security is adversarial — defenses that work against known attacks may fail against novel attacks.
- Focus on KNOWN vulnerability patterns for RLVR (we need ground truth).
- Configuration hardening scanners (CIS benchmarks, Lynis, etc.) provide reliable automated verification.

## Connections
- [[cybersecurity-ctf]] — offensive counterpart
- [[config-generation]] — generating secure configurations
- [[code-repair]] — fixing security vulnerabilities
- [[protocol-compliance]] — secure protocol implementation
