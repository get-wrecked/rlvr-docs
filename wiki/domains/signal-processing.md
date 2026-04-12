---
domain: Signal Processing
category: Engineering
verification_type: Numerical computation of frequency response, SNR, error metrics
dataset_scale: Medium (10K-50K, primarily synthetic)
difficulty_range: Basic filtering to advanced compression and adaptive algorithms
modality: Text-in, structured-out (filter coefficients, spectral values, algorithm parameters)
status: Strong — DSP verification is mathematically exact for most tasks
---

## Overview

Signal processing tasks ask the model to design filters, analyze spectra, compress signals, detect features, and solve other DSP problems. The RLVR strength here is that digital signal processing is grounded in exact mathematics — a filter's frequency response is a deterministic function of its coefficients, SNR is a computable quantity, and compression ratios are measurable. There is no ambiguity in whether a designed filter meets its specifications.

This domain covers FIR/IIR filter design, spectral analysis (DFT, FFT interpretation), sampling theory (Nyquist, aliasing), modulation/demodulation, signal compression, noise reduction, and adaptive filtering. These span communications, audio processing, image processing, and radar.

## Verification Mechanism

**Primary approach:** Compute the designed system's actual performance characteristics mathematically and compare to specifications.

- **Frequency response verification:** Given filter coefficients, compute the frequency response H(e^{jw}) and check passband ripple, stopband attenuation, cutoff frequency, transition bandwidth against specs.
- **Impulse/step response verification:** Compute and check for desired properties (linear phase, minimum overshoot, specific decay rate).
- **SNR computation:** Apply the model's filter/algorithm to a test signal with known noise, measure output SNR. Compare to requirement.
- **Compression metrics:** Apply compression algorithm, measure compression ratio and distortion (MSE, PSNR). Verify both meet specs simultaneously.
- **Sampling/reconstruction verification:** Check whether the model correctly identifies Nyquist rate, aliasing frequency, or reconstructed signal properties.
- **Spectral analysis verification:** Given a signal, verify the model correctly identifies frequency components, their magnitudes, and phases.

**Verification reliability: VERY HIGH for filter design tasks.** Given filter coefficients, the frequency response is an exact mathematical computation. No simulation uncertainty.

**Verification reliability: HIGH for spectral analysis and sampling theory.** DFT is a deterministic computation. Nyquist theorem gives exact results.

**Verification reliability: HIGH for compression metrics.** MSE, PSNR, compression ratio are exactly computable.

**Verification reliability: MODERATE for adaptive and statistical signal processing.** Algorithms like LMS/RLS have convergence behavior that depends on input statistics. Verification requires running on specific test signals, which may not capture general performance.

## Dataset Sources

- **Textbook problems:** Oppenheim/Willsky, Haykin, Proakis/Manolakis provide hundreds of well-defined problems.
- **IEEE Signal Processing Cup:** Competition problems with defined metrics.
- **Synthetic generation:** Extremely scalable. Parameterize filter specifications (cutoff, order, ripple, attenuation) to generate unlimited design problems. Create test signals with known spectral content.
- **Standard test signals:** Well-known signals for benchmarking (chirps, multi-tone, speech, image test patterns like Lena/Barbara).
- **MATLAB Signal Processing Toolbox examples:** Standard design problems with reference solutions.

**Realistic scale:** 20K-50K problems easily achievable through synthetic generation. The mathematical nature of DSP makes it one of the most scalable domains for RLVR.

## Task Format

**Input:** Signal processing problem with specifications.

Example 1 (filter design):
```
Design a lowpass FIR filter with the following specifications:
- Passband edge: 0.2π rad/sample
- Stopband edge: 0.3π rad/sample
- Maximum passband ripple: 0.1 dB
- Minimum stopband attenuation: 40 dB
What is the minimum filter order, and give the filter coefficients?
```

Example 2 (spectral analysis):
```
A continuous-time signal x(t) = 3cos(200πt) + 5sin(600πt) is sampled
at fs = 1000 Hz. If we compute a 1000-point DFT of one second of data,
at which DFT bin indices will we see peaks?
```

Example 3 (sampling):
```
A signal has frequency components at 100 Hz, 250 Hz, and 400 Hz.
What is the minimum sampling frequency to avoid aliasing?
```

**Output:** Filter coefficients, frequency values, numerical answers.

## Difficulty Curriculum

1. **Level 1 — Sampling and frequency basics:** Nyquist rate computation, frequency identification in DFT output, basic spectral properties.
2. **Level 2 — Simple filter design:** Design FIR/IIR filters of specified order using windowing or bilinear transform. Direct formula application.
3. **Level 3 — Optimal filter design:** Meet multi-constraint specifications (passband ripple + stopband attenuation + transition width). Requires understanding design trade-offs.
4. **Level 4 — System design:** Multi-rate processing (decimation, interpolation), modulation/demodulation schemes, filter banks.
5. **Level 5 — Adaptive and statistical:** Adaptive filter design (LMS step size selection, convergence analysis), optimal detection/estimation, compressed sensing.

## Limitations & Risks

- **Coefficient precision:** Filter coefficients are continuous-valued. The model's output precision affects whether the designed filter actually meets specs. Verification must account for finite-precision effects.
- **Multiple valid designs:** Many filter designs can meet the same specs (different orders, window types, optimization methods). Verification must check spec compliance, not coefficient matching.
- **Finite wordlength effects:** In practice, filters are implemented with finite-precision arithmetic. Textbook analysis assumes infinite precision. This gap is usually small but can matter for high-order IIR filters.
- **Perceptual quality:** For audio/image tasks, mathematical metrics (MSE, PSNR) don't perfectly correlate with perceptual quality. But for RLVR purposes, we must use computable metrics.
- **Problem statement ambiguity:** Filter specifications must be precisely stated (passband/stopband edges, ripple/attenuation in dB vs. linear, one-sided vs. two-sided spectra). Ambiguity in the problem statement makes verification unreliable.

## Connections

- **circuit-design.md** overlaps for analog filter design and implementation
- **control-systems.md** shares frequency-domain analysis (Bode plots, transfer functions)
- **electrical-engineering.md** covers the broader EE context where DSP is applied
- **physics-simulation.md** provides signals to process (sensor data, measurement data)
- Compression tasks connect to information theory / coding theory domains
