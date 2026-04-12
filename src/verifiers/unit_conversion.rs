//! Unit Conversion Verifier
//!
//! Verifies numerical unit conversions. Covers: unit-conversion, recipe-scaling,
//! and dimensional analysis tasks.
//!
//! Contains a lookup table of common conversion factors.
//! Verification: compute the correct answer and compare.

use std::collections::HashMap;
use super::VerifyResult;

/// Build the conversion factor table.
/// Key: (from_unit, to_unit), Value: multiply by this factor.
fn conversion_factors() -> HashMap<(&'static str, &'static str), f64> {
    let mut m = HashMap::new();

    // Length
    m.insert(("m", "cm"), 100.0);
    m.insert(("m", "mm"), 1000.0);
    m.insert(("m", "km"), 0.001);
    m.insert(("m", "in"), 39.3701);
    m.insert(("m", "ft"), 3.28084);
    m.insert(("m", "yd"), 1.09361);
    m.insert(("m", "mi"), 0.000621371);
    m.insert(("km", "mi"), 0.621371);
    m.insert(("mi", "km"), 1.60934);
    m.insert(("ft", "m"), 0.3048);
    m.insert(("in", "cm"), 2.54);
    m.insert(("cm", "in"), 0.393701);
    m.insert(("ft", "in"), 12.0);
    m.insert(("yd", "ft"), 3.0);
    m.insert(("mi", "ft"), 5280.0);

    // Mass
    m.insert(("kg", "g"), 1000.0);
    m.insert(("kg", "lb"), 2.20462);
    m.insert(("lb", "kg"), 0.453592);
    m.insert(("lb", "oz"), 16.0);
    m.insert(("oz", "g"), 28.3495);
    m.insert(("g", "oz"), 0.035274);
    m.insert(("kg", "oz"), 35.274);
    m.insert(("ton", "kg"), 1000.0);
    m.insert(("ton", "lb"), 2204.62);

    // Temperature (special — handled separately)

    // Volume
    m.insert(("l", "ml"), 1000.0);
    m.insert(("l", "gal"), 0.264172);
    m.insert(("gal", "l"), 3.78541);
    m.insert(("gal", "qt"), 4.0);
    m.insert(("qt", "pt"), 2.0);
    m.insert(("pt", "cup"), 2.0);
    m.insert(("cup", "ml"), 236.588);
    m.insert(("cup", "tbsp"), 16.0);
    m.insert(("tbsp", "tsp"), 3.0);
    m.insert(("l", "cup"), 4.22675);
    m.insert(("ml", "tsp"), 0.202884);

    // Time
    m.insert(("hr", "min"), 60.0);
    m.insert(("min", "s"), 60.0);
    m.insert(("hr", "s"), 3600.0);
    m.insert(("day", "hr"), 24.0);
    m.insert(("week", "day"), 7.0);
    m.insert(("year", "day"), 365.25);

    // Speed
    m.insert(("mph", "kph"), 1.60934);
    m.insert(("kph", "mph"), 0.621371);
    m.insert(("m/s", "kph"), 3.6);
    m.insert(("kph", "m/s"), 0.277778);

    // Area
    m.insert(("m2", "ft2"), 10.7639);
    m.insert(("ft2", "m2"), 0.092903);
    m.insert(("acre", "m2"), 4046.86);
    m.insert(("ha", "acre"), 2.47105);
    m.insert(("km2", "mi2"), 0.386102);

    m
}

/// Normalize unit name to canonical form.
fn normalize_unit(unit: &str) -> String {
    let lower = unit.to_lowercase().trim().to_string();
    match lower.as_str() {
        "meters" | "meter" | "metre" | "metres" => "m".into(),
        "centimeters" | "centimeter" | "centimetre" => "cm".into(),
        "millimeters" | "millimeter" | "millimetre" => "mm".into(),
        "kilometers" | "kilometer" | "kilometre" => "km".into(),
        "inches" | "inch" | "\"" => "in".into(),
        "feet" | "foot" | "'" => "ft".into(),
        "yards" | "yard" => "yd".into(),
        "miles" | "mile" => "mi".into(),
        "kilograms" | "kilogram" | "kgs" => "kg".into(),
        "grams" | "gram" => "g".into(),
        "pounds" | "pound" | "lbs" => "lb".into(),
        "ounces" | "ounce" => "oz".into(),
        "tons" | "tonnes" | "tonne" => "ton".into(),
        "liters" | "liter" | "litre" | "litres" => "l".into(),
        "milliliters" | "milliliter" | "millilitre" => "ml".into(),
        "gallons" | "gallon" => "gal".into(),
        "quarts" | "quart" => "qt".into(),
        "pints" | "pint" => "pt".into(),
        "cups" => "cup".into(),
        "tablespoons" | "tablespoon" => "tbsp".into(),
        "teaspoons" | "teaspoon" => "tsp".into(),
        "hours" | "hour" => "hr".into(),
        "minutes" | "minute" => "min".into(),
        "seconds" | "second" => "s".into(),
        "days" => "day".into(),
        "weeks" => "week".into(),
        "years" | "year" => "year".into(),
        "celsius" | "°c" | "c" => "c".into(),
        "fahrenheit" | "°f" | "f" => "f".into(),
        "kelvin" | "k" => "k".into(),
        other => other.into(),
    }
}

/// Convert a value between units.
pub fn convert(value: f64, from: &str, to: &str) -> Option<f64> {
    let from_n = normalize_unit(from);
    let to_n = normalize_unit(to);

    if from_n == to_n {
        return Some(value);
    }

    // Temperature (special cases)
    match (from_n.as_str(), to_n.as_str()) {
        ("c", "f") => return Some(value * 9.0 / 5.0 + 32.0),
        ("f", "c") => return Some((value - 32.0) * 5.0 / 9.0),
        ("c", "k") => return Some(value + 273.15),
        ("k", "c") => return Some(value - 273.15),
        ("f", "k") => return Some((value - 32.0) * 5.0 / 9.0 + 273.15),
        ("k", "f") => return Some((value - 273.15) * 9.0 / 5.0 + 32.0),
        _ => {}
    }

    let factors = conversion_factors();

    // Direct lookup
    if let Some(&factor) = factors.get(&(from_n.as_str(), to_n.as_str())) {
        return Some(value * factor);
    }

    // Try reverse
    if let Some(&factor) = factors.get(&(to_n.as_str(), from_n.as_str())) {
        return Some(value / factor);
    }

    None
}

/// Verify a unit conversion answer.
pub fn verify(value: f64, from: &str, to: &str, proposed_answer: f64, rel_tol: f64) -> VerifyResult {
    match convert(value, from, to) {
        Some(correct) => {
            let diff = (proposed_answer - correct).abs();
            let rel = diff / correct.abs().max(1e-10);
            if rel <= rel_tol {
                VerifyResult::correct()
            } else {
                VerifyResult::wrong(format!(
                    "{value} {from} = {correct:.6} {to}, got {proposed_answer}"
                ))
            }
        }
        None => VerifyResult::wrong(format!("unknown conversion: {from} → {to}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Direct Conversions ==========

    #[test]
    fn meters_to_feet() {
        let result = convert(1.0, "meters", "feet").unwrap();
        assert!((result - 3.28084).abs() < 0.001);
    }

    #[test]
    fn km_to_miles() {
        let result = convert(10.0, "km", "miles").unwrap();
        assert!((result - 6.21371).abs() < 0.001);
    }

    #[test]
    fn pounds_to_kg() {
        let result = convert(100.0, "pounds", "kilograms").unwrap();
        assert!((result - 45.3592).abs() < 0.01);
    }

    #[test]
    fn celsius_to_fahrenheit() {
        assert!((convert(0.0, "celsius", "fahrenheit").unwrap() - 32.0).abs() < 0.01);
        assert!((convert(100.0, "celsius", "fahrenheit").unwrap() - 212.0).abs() < 0.01);
        assert!((convert(37.0, "celsius", "fahrenheit").unwrap() - 98.6).abs() < 0.1);
    }

    #[test]
    fn fahrenheit_to_celsius() {
        assert!((convert(32.0, "fahrenheit", "celsius").unwrap() - 0.0).abs() < 0.01);
        assert!((convert(212.0, "fahrenheit", "celsius").unwrap() - 100.0).abs() < 0.01);
    }

    #[test]
    fn cups_to_ml() {
        let result = convert(1.0, "cups", "ml").unwrap();
        assert!((result - 236.588).abs() < 1.0);
    }

    #[test]
    fn hours_to_seconds() {
        assert_eq!(convert(1.0, "hours", "seconds").unwrap(), 3600.0);
    }

    // ========== Verification ==========

    #[test]
    fn verify_correct() {
        let result = verify(1.0, "km", "mi", 0.621371, 0.001);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_wrong() {
        let result = verify(1.0, "km", "mi", 1.0, 0.001);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_within_tolerance() {
        let result = verify(1.0, "km", "mi", 0.621, 0.01);
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_conversions() {
        assert_eq!(verify(100.0, "cm", "in", 39.3701, 0.001).score, 1.0);
        assert_eq!(verify(100.0, "cm", "in", 50.0, 0.001).score, 0.0);

        assert_eq!(verify(5.0, "gallons", "liters", 18.927, 0.01).score, 1.0);
        assert_eq!(verify(5.0, "gallons", "liters", 5.0, 0.01).score, 0.0);
    }

    // ========== Identity Conversion ==========

    #[test]
    fn same_unit_is_identity() {
        assert_eq!(convert(42.0, "kg", "kg").unwrap(), 42.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_absolute_zero() {
        let result = convert(-273.15, "celsius", "kelvin").unwrap();
        assert!((result - 0.0).abs() < 0.01, "Absolute zero = 0K");
    }

    #[test]
    fn adversarial_minus_40_crossover() {
        let result = convert(-40.0, "fahrenheit", "celsius").unwrap();
        assert!((result - (-40.0)).abs() < 0.01, "-40°F = -40°C");
    }

    #[test]
    fn adversarial_wrong_answer_rejected() {
        let result = verify(1.0, "miles", "km", 1.0, 0.01);
        assert_eq!(result.score, 0.0, "1 mile ≠ 1 km");
    }

    #[test]
    fn adversarial_unknown_unit() {
        let result = verify(1.0, "parsecs", "lightyears", 1.0, 0.01);
        assert_eq!(result.score, 0.0, "Unknown units should fail");
    }
}
