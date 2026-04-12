---
domain: Time Series Forecasting
category: data-science
verification_type: execution
dataset_scale: 1M+ (from Monash, M-competitions, industry)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Time Series Forecasting

## Overview
Predict future values of time series: point forecasts, probabilistic forecasts, anomaly detection, change point detection. Verification: compare predictions to held-out actuals using standard metrics (MAE, RMSE, MAPE, CRPS).

## Verification Mechanism
```python
import numpy as np
from scipy.stats import norm

def verify(task_type: str, predictions: Any, actuals: np.array) -> float:
    if task_type == "point_forecast":
        mae = np.mean(np.abs(predictions - actuals))
        # Normalize by series scale
        naive_mae = np.mean(np.abs(actuals[1:] - actuals[:-1]))  # Naive baseline
        mase = mae / naive_mae if naive_mae > 0 else mae
        return max(0, 1 - mase)  # MASE < 1 = better than naive
    
    elif task_type == "probabilistic_forecast":
        # CRPS (Continuous Ranked Probability Score)
        crps = compute_crps(predictions["mean"], predictions["std"], actuals)
        return max(0, 1 - crps / np.std(actuals))
    
    elif task_type == "anomaly_detection":
        # F1 score on anomaly labels
        pred_labels = predictions["anomalies"]
        true_labels = actuals["anomalies"]
        return f1_score(true_labels, pred_labels)
    
    elif task_type == "changepoint":
        # Distance between predicted and actual changepoints
        pred_cps = set(predictions["changepoints"])
        true_cps = set(actuals["changepoints"])
        precision = len(pred_cps & true_cps) / max(len(pred_cps), 1)
        recall = len(pred_cps & true_cps) / max(len(true_cps), 1)
        return 2 * precision * recall / max(precision + recall, 1e-10)
```

## Dataset Sources
- **Monash Time Series Forecasting Archive**: 30+ datasets across domains.
- **M4 Competition**: 100K time series from multiple domains.
- **M5 Competition**: Walmart sales data (42K time series).
- **ETTh/ETTm**: Electricity transformer temperature datasets.
- **Weather datasets**: Global hourly weather data.
- **Electricity Load**: UCI electricity consumption (370 series).
- **Traffic**: California traffic flow data.
- **Exchange rates**: Daily exchange rates.
- **Yahoo Anomaly Detection benchmark**: Labeled anomaly datasets.
- **NAB (Numenta Anomaly Benchmark)**: 58 labeled time series.

## Task Format
- **Input**: Historical time series + "Forecast the next 24 values"
- **Output**: Point predictions (and optionally confidence intervals)

## Difficulty Curriculum
- Level 1: Stationary series, simple trend/seasonality
- Level 3: Multiple seasonality, holiday effects
- Level 5: Non-stationary, structural breaks
- Level 7: Multivariate, cross-series forecasting
- Level 9: Long-horizon forecasting, distribution shift

## Limitations & Risks
- Time series are inherently stochastic — even perfect models can't achieve 0 error.
- MASE normalizes by naive baseline, making comparison fair across series.
- For RLVR, reward should be relative to baseline (beat naive/seasonal naive), not absolute accuracy.

## Connections
- [[probability-statistics]] — statistical foundations
- [[data-science-eda]] — time series analysis
- [[epidemiology-modeling]] — epidemic forecasting
- [[financial-calculation]] — financial time series
- [[climate-science]] — climate time series
