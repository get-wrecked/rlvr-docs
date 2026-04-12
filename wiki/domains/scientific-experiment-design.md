---
domain: Scientific Experiment Design
category: science-methodology
verification_type: constraint
dataset_scale: 50K+ (from published protocols + simulation)
difficulty_range: medium/hard/superhuman
modality: text
status: needs_validation
---

# Scientific Experiment Design

## Overview
Given a scientific hypothesis, design an experiment to test it — specifying variables, controls, sample sizes, measurements, and analysis plan. Verification: check that the design satisfies methodological constraints (has controls, adequate power, no confounds) and optionally simulate the experiment to check if it can distinguish the hypothesis from null.

## Verification Mechanism
```python
def verify(hypothesis: dict, experiment_design: dict) -> float:
    score = 0.0
    checks = 0
    
    # Has independent variable
    checks += 1
    if "independent_variable" in experiment_design:
        score += 1
    
    # Has dependent variable (measurable outcome)
    checks += 1
    if "dependent_variable" in experiment_design and \
       experiment_design["dependent_variable"]["measurable"]:
        score += 1
    
    # Has control group
    checks += 1
    if "control_group" in experiment_design:
        score += 1
    
    # Has randomization
    checks += 1
    if experiment_design.get("randomization", False):
        score += 1
    
    # Sample size adequate (power analysis)
    checks += 1
    if "sample_size" in experiment_design:
        expected_effect = hypothesis.get("expected_effect_size", 0.5)
        required_n = power_analysis(effect_size=expected_effect, alpha=0.05, power=0.8)
        if experiment_design["sample_size"] >= required_n:
            score += 1
    
    # Controls for confounds listed in hypothesis
    for confound in hypothesis.get("known_confounds", []):
        checks += 1
        if confound in experiment_design.get("controlled_variables", []):
            score += 1
    
    # Statistical test is appropriate
    checks += 1
    if is_appropriate_test(experiment_design["statistical_test"],
                          experiment_design["dependent_variable"]["type"],
                          experiment_design.get("groups", 2)):
        score += 1
    
    # Simulation: can the design distinguish hypothesis from null?
    if hypothesis.get("simulation_model"):
        checks += 1
        power = simulate_experiment(
            experiment_design, 
            hypothesis["simulation_model"],
            n_simulations=1000
        )
        if power >= 0.8:
            score += 1
    
    return score / checks
```

## Dataset Sources
- **Published protocols**: Protocol repositories (protocols.io) with structured experiment descriptions.
- **Clinical trial registries**: ClinicalTrials.gov — 400K+ registered trials with design details.
- **Methods sections of papers**: Extract from PubMed/arXiv. Semi-structured.
- **Textbook exercises**: Research methods textbooks (psychology, biology, etc.).
- **Procedural generation**: Generate hypotheses + correct experiment designs, also generate flawed designs for the agent to identify and fix.

## Task Format
- **Input**: "Hypothesis: A new fertilizer increases tomato yield by 20% compared to standard fertilizer. Design an experiment to test this."
- **Output**: Structured experiment design (variables, groups, sample size, protocol, analysis plan)

## Difficulty Curriculum
- Level 1: Simple two-group comparison (treatment vs. control)
- Level 3: Factorial designs, multiple dependent variables
- Level 5: Longitudinal designs, repeated measures, counterbalancing
- Level 7: Complex clinical trial designs (crossover, adaptive)
- Level 9: Multi-site, multi-factor with advanced statistics

## Limitations & Risks
- Experiment design quality has subjective aspects that constraints don't fully capture. A design can satisfy all checked constraints but still be poorly conceived.
- The constraint checker is only as good as its rule set. May miss subtle methodological flaws.
- Best used as supplementary training rather than primary RLVR signal.
- Simulation-based verification (can the design actually detect the effect?) is the strongest component.

## Connections
- [[probability-statistics]] — statistical foundations
- [[biology-sequence]] — biology experiment context
- [[chemistry-computation]] — chemistry experiment context
