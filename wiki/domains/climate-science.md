---
domain: Climate Science & Earth System Modeling
category: science-earth
verification_type: execution
dataset_scale: massive (petabytes of earth observation)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Climate Science & Earth System Modeling

## Overview
Answer verifiable questions about climate/earth science: predict temperature from radiative forcing, compute carbon budgets, classify satellite imagery, analyze climate time series. Verification: compare to observational data or physics-based computation.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "radiative_forcing":
        # Compute radiative forcing from CO2 concentration
        correct = 5.35 * np.log(answer["co2"] / 280)  # W/m²
        return 1.0 if abs(answer["forcing"] - correct) / correct < 0.05 else 0.0
    
    elif task_type == "carbon_budget":
        # Compute remaining carbon budget for temperature target
        correct = compute_carbon_budget(answer["target_temp"], problem["current_emissions"])
        return 1.0 if abs(answer["budget"] - correct) / correct < 0.1 else 0.0
    
    elif task_type == "satellite_classification":
        # Classify land cover from satellite image
        pred_class = answer["class"]
        gold_class = problem["ground_truth_class"]
        return 1.0 if pred_class == gold_class else 0.0
    
    elif task_type == "time_series_analysis":
        # Trend detection, anomaly identification
        correct_trend = compute_trend(problem["data"])
        return 1.0 if abs(answer["trend"] - correct_trend) < 0.01 else 0.0
```

## Dataset Sources
- **ERA5**: ECMWF global reanalysis (hourly, 1940-present).
- **CMIP6**: Climate model intercomparison project outputs.
- **Copernicus Climate Data Store**: Free climate datasets.
- **NOAA NCDC**: US climate data.
- **Sentinel-2**: Satellite imagery for land cover.
- **NASA GISS**: Temperature records.
- **GlobalForestWatch**: Deforestation data.
- **ClimateNet**: Labeled climate events in simulation data.
- **EuroSAT**: 27K satellite images classified into 10 categories.
- **UC Merced Land Use**: 2K satellite images in 21 classes.

## Task Format
- **Input**: "Given atmospheric CO2 has risen from 280ppm to 420ppm, compute the approximate radiative forcing change."
- **Output**: "ΔF ≈ 5.35 × ln(420/280) ≈ 2.16 W/m²"

## Difficulty Curriculum
- Level 1: Basic climate calculations (radiative forcing, albedo)
- Level 3: Time series trend analysis, satellite image classification
- Level 5: Climate model parameter estimation
- Level 7: Multi-model ensemble analysis, extreme event attribution
- Level 9: Full Earth system model integration

## Limitations & Risks
- Climate science involves deep uncertainties. Focus on well-established physics-based calculations for verification.
- Satellite image classification has ground truth labels — reliable.
- Long-term predictions are inherently uncertain — use hindcasting for verification.

## Connections
- [[physics-simulation]] — physical climate models
- [[data-science-eda]] — climate data analysis
- [[image-classification]] — satellite image classification
- [[mathematical-modeling]] — climate modeling
