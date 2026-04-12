---
domain: Climate and Weather Prediction
category: Science
verification_type: Comparison to observed outcomes (reanalysis data, station measurements)
dataset_scale: Massive (petabytes of ERA5, NOAA data; practical subsets much smaller)
difficulty_range: Climatology lookup to skillful multi-day forecasting
modality: Text-in, text-out (numerical predictions, categorical forecasts)
status: Problematic for RLVR — stochasticity makes single-prediction verification unreliable
---

## Overview

Climate and weather tasks ask the model to make predictions about atmospheric, oceanic, and climate systems: temperature forecasts, precipitation predictions, extreme event likelihood, climate trend estimation, and seasonal outlooks. The motivation is enormous — weather prediction is one of the most consequential scientific prediction tasks. However, this domain has a fundamental tension with RLVR: weather is inherently stochastic, and a "correct" forecast is a probability distribution, not a point prediction. Verifying a single deterministic prediction against a single observed outcome is statistically unsound.

This domain covers nowcasting (0-6 hours), short-range forecasting (1-3 days), medium-range forecasting (3-15 days), seasonal prediction, climate trend estimation, and extreme event prediction. The verification challenge grows with forecast horizon.

## Verification Mechanism

**Primary approach:** Compare model predictions to observed outcomes from reanalysis or station data.

- **Point prediction comparison:** Compare predicted temperature, pressure, wind speed to observed values at a location and time. Compute error metrics (MAE, RMSE, bias).
- **Categorical verification:** For categorical forecasts (rain/no rain, above/below normal), compare to observed category. Binary accuracy or Heidke Skill Score.
- **Climatology baseline comparison:** A prediction must beat climatology (historical average for that location and date) to demonstrate skill. Reward = 1 if better than climatology, 0 otherwise.
- **Ensemble-based verification:** If the model produces probabilistic forecasts, verify calibration (predicted 70% chance of rain; did it rain ~70% of the time across similar forecasts?). Requires many samples.
- **Physical consistency checks:** Verify that predictions are physically consistent (e.g., geostrophic balance, hydrostatic consistency, energy conservation). Necessary but not sufficient.

**Verification reliability: WARNING — fundamentally problematic for RLVR.**

The core issue: weather is chaotic. A forecast of 72F that verifies at 74F might have been an excellent forecast (if the forecast distribution was centered at 72F with 3F spread), or a terrible one (if the true distribution was centered at 68F). A single observation cannot distinguish forecast quality from luck. This makes binary reward signals extremely noisy.

**Verification reliability: MODERATE for short-range, aggregated metrics.** If we evaluate many forecasts and reward based on aggregate skill scores (rather than individual predictions), the signal becomes meaningful. But this requires evaluating the model on hundreds of forecasts before providing a reward signal, which conflicts with standard RLVR per-example rewards.

**Verification reliability: LOW for individual forecasts beyond 3 days.** Forecast skill drops rapidly with lead time. By day 10, even the best models have limited skill for specific weather events. Single-forecast verification is dominated by noise.

**Verification reliability: MODERATE for climate statistics.** Predicting long-term averages, trends, or return periods is more verifiable because we are predicting statistical properties, not specific events.

## Dataset Sources

- **ERA5 (ECMWF):** Global reanalysis dataset, 0.25° resolution, hourly, from 1940 to present. The gold standard for "observed" weather.
- **NOAA/NCEP GFS analysis:** Operational weather analysis data. Global coverage.
- **ISD (Integrated Surface Database):** Station-level observations worldwide. Ground truth for point verification.
- **WeatherBench / WeatherBench2:** Standardized benchmarks for ML weather prediction with defined evaluation protocols.
- **CMIP6:** Climate model output for climate projection tasks. Not observations, but provides scenarios for climate-scale prediction.
- **GHCN (Global Historical Climatology Network):** Long-term station records for climatology tasks.

**Realistic scale:** The data is massive, but for RLVR we need well-defined prediction tasks. WeatherBench provides ~10K+ standardized forecast-verification pairs. Custom tasks from ERA5 can provide 100K+ instances.

## Task Format

**Input:** Current conditions or date/location and forecast query.

Example 1 (short-range):
```
Given the following atmospheric state at 2024-03-15 00Z
[temperature, pressure, wind fields at multiple levels],
predict the 2-meter temperature at London Heathrow 24 hours later.
```

Example 2 (climatology):
```
What is the average July maximum temperature in Phoenix, Arizona,
based on the 1991-2020 climatology?
```

Example 3 (trend):
```
Based on the global mean surface temperature record from 1980-2020,
what is the linear warming trend in degrees C per decade?
```

**Output:** Temperature, precipitation amount, categorical forecast, or trend value.

## Difficulty Curriculum

1. **Level 1 — Climatology retrieval:** Average temperature for location/month, record high/low, typical precipitation. Database lookup.
2. **Level 2 — Trend computation:** Compute warming trends, precipitation trends from historical data. Statistical computation.
3. **Level 3 — Short-range forecasting:** Predict next-day temperature given current conditions. Persistence and simple pattern recognition.
4. **Level 4 — Medium-range forecasting:** 3-7 day forecasts. Requires understanding synoptic weather patterns.
5. **Level 5 — Extreme event prediction and seasonal outlooks:** Predict probability of extreme events or seasonal anomalies. Highest skill required, lowest verification reliability.

## Limitations & Risks

- **STOCHASTICITY IS THE FUNDAMENTAL PROBLEM.** This cannot be overstated. Weather is a chaotic system. Verifying point predictions with single observations produces extremely noisy reward signals. This may make standard RLVR ineffective for this domain.
- **Probabilistic verification requires many samples.** Proper scoring rules (Brier score, CRPS) require evaluating many forecasts to assess calibration. This conflicts with per-example RLVR rewards.
- **Temporal data leakage:** If the model was trained on data up to 2025, it may have "memorized" weather events. Verification must use temporally held-out data that postdates training.
- **Spatial resolution mismatch:** ERA5 at 0.25° cannot capture local effects (urban heat islands, mountain valleys). Station data is sparse. Both create verification noise.
- **Climate change non-stationarity:** Historical climatology becomes less relevant as the climate changes. A model that learns historical patterns may be systematically biased for future predictions.
- **Potential misuse:** Climate prediction models could be used to mislead about climate change. The verification framework does not prevent this.
- **HONEST ASSESSMENT:** This domain is one of the weakest candidates for standard RLVR among all domains in this wiki. It could work for climatology retrieval and trend computation, but for genuine weather prediction, the stochastic verification problem is severe. Consider using aggregate skill scores (requiring batched evaluation) rather than per-example binary rewards.

## Connections

- **physics-simulation.md** provides the underlying fluid dynamics and thermodynamics
- **fluid-dynamics.md** covers the computational methods used in weather models (Navier-Stokes solvers)
- **signal-processing.md** connects via spectral analysis of climate time series
- **astronomy-computation.md** shares the challenge of predicting complex physical systems from observations
- Verification challenges here are a warning for any domain with stochastic outcomes
