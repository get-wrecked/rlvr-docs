---
domain: Network Configuration & Troubleshooting
category: systems
verification_type: simulation
dataset_scale: 100K+ (generatable from network topologies)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Network Configuration & Troubleshooting

## Overview
Given a network topology and requirements (reachability, bandwidth, security policies), generate valid network configurations (routing tables, firewall rules, VLAN configs, BGP policies). Verification via network simulation.

## Verification Mechanism
```python
def verify(topology: NetworkTopology, requirements: list[Requirement], config: NetworkConfig) -> float:
    # Instantiate simulated network
    sim = Mininet(topology) if layer2_3 else GNS3(topology)
    sim.apply_config(config)
    
    satisfied = 0
    for req in requirements:
        if req.type == "reachability":
            if sim.ping(req.src, req.dst):
                satisfied += 1
        elif req.type == "isolation":
            if not sim.ping(req.src, req.dst):
                satisfied += 1
        elif req.type == "bandwidth":
            if sim.iperf(req.src, req.dst) >= req.min_bw:
                satisfied += 1
        elif req.type == "latency":
            if sim.measure_latency(req.src, req.dst) <= req.max_latency:
                satisfied += 1
    
    return satisfied / len(requirements)
```

Also: validate config syntax before simulation.

## Dataset Sources
- **Batfish**: Open-source network configuration analysis tool. Has example configs.
- **Cisco/Juniper config archives**: Public config examples from networking courses.
- **GNS3 lab exercises**: Thousands of networking lab exercises with solutions.
- **CCNA/CCNP study materials**: Structured problems with known correct configs.
- **Procedural generation**: Generate random topologies + requirements, compute valid configs algorithmically.

## Task Format
- **Input**: Network diagram (text description or image) + requirements ("Host A must reach Host B but not Host C")
- **Output**: Router/switch configuration commands

## Difficulty Curriculum
- Level 1: Single subnet, basic routing
- Level 3: VLANs, inter-VLAN routing
- Level 5: OSPF/BGP configuration
- Level 7: Complex security policies, NAT, VPN
- Level 9: Multi-domain, redundancy, failover

## Limitations & Risks
- Network simulation is slow (seconds to minutes per verification). Not ideal for rapid RL training.
- Config syntax varies by vendor (Cisco IOS, Juniper JUNOS, etc.). May need to standardize.
- Real-world networking has nuances that simulators miss.

## Connections
- [[infrastructure-as-code]] — related systems configuration
- [[config-generation]] — general config generation
- [[protocol-compliance]] — protocol-level correctness
