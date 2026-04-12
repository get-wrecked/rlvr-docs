---
domain: Cryptographic Protocol Analysis
category: security-formal
verification_type: rule
dataset_scale: 10K+ (from protocol verification benchmarks)
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Cryptographic Protocol Analysis

## Overview
Analyze security protocols (key exchange, authentication, e-voting) for vulnerabilities, or verify that a protocol satisfies security properties (secrecy, authentication, forward secrecy). Verification: use formal protocol verification tools (ProVerif, Tamarin, AVISPA).

## Verification Mechanism
```python
def verify(protocol_spec: str, claimed_properties: list[str], tool: str = "proverif") -> float:
    if tool == "proverif":
        result = run_proverif(protocol_spec, claimed_properties, timeout=300)
        verified = sum(1 for p in result.properties if p.status == "verified")
        return verified / len(claimed_properties)
    
    elif tool == "tamarin":
        result = run_tamarin(protocol_spec, claimed_properties, timeout=600)
        proven = sum(1 for p in result.lemmas if p.status == "proven")
        return proven / len(result.lemmas)
```

For attack finding:
```python
def verify_attack(protocol_spec: str, attack_trace: list[Message]) -> float:
    """Verify that an attack trace actually breaks the protocol."""
    result = simulate_protocol(protocol_spec, attack_trace)
    if result.security_property_violated:
        return 1.0
    return 0.0
```

## Dataset Sources
- **AVISPA library**: Protocol verification benchmarks.
- **Tamarin examples**: Verified protocol examples.
- **ProVerif examples**: Protocol analysis benchmarks.
- **Clark-Jacob library**: Survey of security protocol attacks.
- **Real-world protocols**: TLS 1.3, Signal, WireGuard — with formal analyses.

## Task Format
- **Input**: Protocol description (Alice sends nonce, Bob responds with encrypted key, etc.) + "Does this protocol satisfy forward secrecy?"
- **Output**: "Yes" (with formal verification proof) or "No" (with attack trace)

## Difficulty Curriculum
- Level 3: Analyze Needham-Schroeder-style protocols
- Level 5: TLS handshake variants
- Level 7: Complex multi-party protocols
- Level 9: Novel protocol design with formal security proofs

## Limitations & Risks
- Formal protocol verification tools can be slow (minutes to hours).
- The abstraction gap: protocols verified in formal models may have implementation bugs.
- ProVerif/Tamarin have specific input languages that the agent must learn.

## Connections
- [[cryptography-challenges]] — practical crypto
- [[logic-formal-specification]] — formal specification
- [[smt-solving]] — underlying verification technology
