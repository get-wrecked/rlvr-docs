---
domain: Astronomy Computation
category: Science
verification_type: Kepler's laws, orbital mechanics equations, catalog value lookup
dataset_scale: Medium (10K-50K problems from catalogs and textbooks)
difficulty_range: Orbital period calculation to multi-body dynamics and stellar evolution
modality: Text-in, text-out (numerical predictions, classifications)
status: Strong for classical mechanics; weaker for complex astrophysical systems
---

## Overview

Astronomy computation tasks ask the model to solve quantitative astronomy and astrophysics problems: orbital mechanics (periods, velocities, trajectories), stellar properties (luminosity, temperature, spectral classification), distance calculations (parallax, redshift, standard candles), and celestial event prediction (eclipses, conjunctions, transit timing). The RLVR strength is that much of astronomy is governed by well-understood, deterministic physics (Kepler's laws, gravitational dynamics, Stefan-Boltzmann law) with extensive observational catalogs providing ground truth.

This domain covers orbital mechanics (Kepler's laws, transfer orbits, escape velocity), stellar astrophysics (HR diagram classification, mass-luminosity relation, stellar evolution stages), cosmology calculations (Hubble's law, redshift-distance), and observational astronomy (coordinate transforms, visibility calculations, magnitude conversions).

## Verification Mechanism

**Primary approach:** Compute answers from established physical laws or look up verified catalog values.

- **Orbital mechanics verification:** Kepler's laws and Newtonian gravity give exact analytical solutions for two-body problems. Compute period from semi-major axis and masses (T^2 = 4π^2 a^3 / GM). Exact within the two-body approximation.
- **Stellar property verification:** Stefan-Boltzmann law (L = 4πR^2σT^4), mass-luminosity relations, and spectral classification criteria are deterministic computations. Compare to catalog values (SIMBAD, Hipparcos, Gaia).
- **Distance calculation verification:** Parallax-based distance (d = 1/p arcsec in parsecs), redshift-distance via Hubble's law (v = H_0 d) are exact formulas.
- **Catalog value lookup:** For known objects, verify predictions against established catalogs (NASA Exoplanet Archive, SIMBAD, Gaia DR3). Stellar magnitudes, positions, proper motions are measured to high precision.
- **N-body simulation:** For multi-body problems, verify with numerical integration (Runge-Kutta, symplectic integrators). Deterministic given initial conditions, but chaotic over long timescales.

**Verification reliability: VERY HIGH for two-body orbital mechanics.** Kepler's laws are exact for two-body problems. Analytical solutions exist.

**Verification reliability: HIGH for stellar property calculations.** Physical laws are well-established. Catalog values are measured to high precision.

**Verification reliability: MODERATE for N-body dynamics.** Numerical integration is deterministic but chaotic systems amplify small errors. Long-term predictions become unreliable.

**Verification reliability: MODERATE for cosmological calculations.** Results depend on assumed cosmological parameters (H_0, Omega_m, etc.). Different parameter choices give different answers. The problem must specify parameters.

## Dataset Sources

- **NASA Exoplanet Archive:** ~5,500 confirmed exoplanets with orbital parameters, host star properties. Rich source for orbital mechanics problems.
- **Gaia DR3:** ~1.8 billion sources with parallaxes, proper motions, photometry. Vast but most useful as a lookup table.
- **SIMBAD:** Astronomical object database with comprehensive property listings.
- **Hipparcos catalog:** ~118K stars with precise parallaxes. Classic reference.
- **Textbook problems:** Carroll/Ostlie, Zeilik, Kutner provide hundreds of computational astronomy problems.
- **Synthetic generation:** Parameterize orbital systems (masses, semi-major axes, eccentricities) and generate mechanics problems. Very scalable.
- **IAU Astronomical Almanac:** Ephemeris data for solar system objects. Gold standard for positional astronomy.

**Realistic scale:** 20K-50K problems achievable. Orbital mechanics problems are highly scalable through synthetic generation. Stellar property problems are bounded by the number of well-characterized stars (~100K with complete data).

## Task Format

**Input:** Astronomical scenario with specific parameters and a quantitative question.

Example 1 (orbital mechanics):
```
A satellite orbits Earth at an altitude of 400 km. Given Earth's mass
(5.972 x 10^24 kg) and radius (6371 km), calculate the orbital period
in minutes.
```

Example 2 (stellar properties):
```
A star has an effective temperature of 5,800 K and a luminosity of
3.83 x 10^26 W. What is its radius in solar radii?
(Solar luminosity = 3.828 x 10^26 W, Solar radius = 6.957 x 10^8 m)
```

Example 3 (distance):
```
A galaxy has a measured redshift of z = 0.05. Using Hubble's law with
H_0 = 70 km/s/Mpc, what is its distance in Mpc?
```

**Output:** Numerical values with units.

## Difficulty Curriculum

1. **Level 1 — Direct formula application:** Kepler's third law, parallax distance, magnitude-distance relation. Single formula, plug in values.
2. **Level 2 — Multi-step calculations:** Hohmann transfer orbits (requires combining multiple orbital mechanics equations), HR diagram placement from multiple observables.
3. **Level 3 — System analysis:** Binary star mass determination from observational data, tidal force calculations, Roche lobe analysis.
4. **Level 4 — Multi-body and perturbation problems:** Three-body dynamics, orbital perturbations, Lagrange points. Requires numerical methods or perturbation theory.
5. **Level 5 — Astrophysical modeling:** Stellar evolution timescale estimation, galaxy rotation curve analysis, cosmological distance ladder. Requires synthesizing multiple physical models.

## Limitations & Risks

- **Assumed constants:** Results depend on physical constants and their precision (G, solar mass, Hubble constant). Problems must specify which values to use.
- **Cosmological model dependence:** Distance calculations beyond ~100 Mpc depend on the cosmological model (flat vs. open, dark energy equation of state). The problem must specify assumptions.
- **Catalog data vintage:** Astronomical measurements improve over time. A model trained on older data may give answers that conflict with current best measurements. Verification should use a fixed, specified catalog version.
- **Idealization vs. reality:** Textbook problems assume point masses, circular orbits, no perturbations. Real astronomical systems are more complex. The verification must match the problem's idealizations.
- **N-body chaos:** For problems involving three or more bodies over many orbits, the system is chaotic and predictions diverge. These problems should be flagged and have relaxed tolerances or shortened prediction horizons.
- **Memorization of catalog values:** The model might memorize properties of famous objects (Sirius, Alpha Centauri) rather than learn to compute. Synthetic problems with arbitrary parameters mitigate this.

## Connections

- **physics-simulation.md** provides the gravitational mechanics underlying orbital calculations
- **climate-weather.md** shares challenges with predicting complex physical systems from observations
- **engineering-optimization.md** connects for spacecraft trajectory optimization
- **control-systems.md** relates to spacecraft attitude control and orbit maintenance
- Catalog-based verification parallels **materials-science.md** (database lookup as ground truth)
