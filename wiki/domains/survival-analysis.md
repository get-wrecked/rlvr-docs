---
domain: Survival Analysis & Reliability Engineering
category: applied-statistics
verification_type: execution
dataset_scale: 100K+ (from clinical + engineering datasets)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Survival Analysis & Reliability Engineering

## Overview
Compute survival/reliability metrics: Kaplan-Meier survival curves, Cox proportional hazards coefficients, hazard rates, mean time between failures (MTBF), censored data handling. Verification: compute against known statistical results.

## Verification Mechanism
```python
from lifelines import KaplanMeierFitter, CoxPHFitter

def verify(task_type: str, data: pd.DataFrame, answer: dict) -> float:
    if task_type == "kaplan_meier":
        kmf = KaplanMeierFitter()
        kmf.fit(data["duration"], event_observed=data["event"])
        
        # Check median survival time
        correct_median = kmf.median_survival_time_
        if abs(answer["median"] - correct_median) / max(correct_median, 0.001) < 0.01:
            return 1.0
        return 0.0
    
    elif task_type == "cox_ph":
        cph = CoxPHFitter()
        cph.fit(data, duration_col="duration", event_col="event")
        
        # Compare hazard ratios
        correct_hr = dict(zip(cph.summary.index, np.exp(cph.summary["coef"])))
        score = 0
        for var, hr in answer["hazard_ratios"].items():
            if var in correct_hr and abs(hr - correct_hr[var]) / correct_hr[var] < 0.05:
                score += 1
        return score / len(answer["hazard_ratios"])
    
    elif task_type == "reliability":
        # MTBF, failure rate, availability
        correct = compute_reliability_metrics(data, answer["model"])
        return 1.0 if abs(answer["mtbf"] - correct["mtbf"]) / correct["mtbf"] < 0.01 else 0.0
```

## Dataset Sources
- **SUPPORT**: 10K patient survival dataset.
- **METABRIC**: 2K breast cancer patients with survival.
- **PBC (Primary Biliary Cholangitis)**: Clinical survival dataset.
- **VA Lung Cancer**: Veteran Affairs cancer dataset.
- **NASA turbofan degradation**: Engine failure prediction.
- **Reliability datasets**: MIL-HDBK-217 reliability data.
- **lifelines package built-in datasets**: Standard survival examples.

## Task Format
- **Input**: Survival data (time, event, covariates) + "Compute the median survival time using Kaplan-Meier"
- **Output**: "Median survival time = 365 days (95% CI: 310-420)"

## Difficulty Curriculum
- Level 1: Simple Kaplan-Meier estimation
- Level 3: Cox proportional hazards, log-rank test
- Level 5: Time-varying covariates, competing risks
- Level 7: Frailty models, cure models
- Level 9: Causal survival analysis, dynamic treatment regimes

## Connections
- [[probability-statistics]] — statistical foundations
- [[medical-diagnosis]] — clinical application
- [[epidemiology-modeling]] — population-level survival
- [[mechanical-engineering]] — reliability engineering
