//! Date/Time Computation Verifier
//!
//! Verifies date/time calculations: days between dates, day of week,
//! timezone conversions, date arithmetic.

use chrono::{NaiveDate, Datelike, Weekday};
use super::VerifyResult;

/// Compute days between two dates.
pub fn days_between(date1: &str, date2: &str) -> Option<i64> {
    let d1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").ok()?;
    let d2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").ok()?;
    Some((d2 - d1).num_days())
}

/// Get the day of week for a date.
pub fn day_of_week(date: &str) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    Some(format!("{}", d.weekday()))
}

/// Add days to a date.
pub fn add_days(date: &str, days: i64) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let result = d + chrono::Duration::days(days);
    Some(result.format("%Y-%m-%d").to_string())
}

/// Check if a year is a leap year.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Verify a date/time computation answer.
pub fn verify(task_type: &str, params: &serde_json::Value, proposed_answer: &str) -> VerifyResult {
    match task_type {
        "days_between" => {
            let d1 = params["date1"].as_str().unwrap_or("");
            let d2 = params["date2"].as_str().unwrap_or("");
            match days_between(d1, d2) {
                Some(correct) => {
                    let proposed: i64 = match proposed_answer.trim().parse() {
                        Ok(v) => v,
                        Err(_) => return VerifyResult::wrong("could not parse answer as integer"),
                    };
                    if proposed == correct {
                        VerifyResult::correct()
                    } else {
                        VerifyResult::wrong(format!("expected {correct}, got {proposed}"))
                    }
                }
                None => VerifyResult::wrong("could not parse dates"),
            }
        }
        "day_of_week" => {
            let date = params["date"].as_str().unwrap_or("");
            match day_of_week(date) {
                Some(correct) => {
                    let proposed = proposed_answer.trim().to_lowercase();
                    let correct_lower = correct.to_lowercase();
                    // Accept full name or abbreviation
                    if proposed == correct_lower
                        || proposed == &correct_lower[..3]
                        || proposed.starts_with(&correct_lower[..3])
                    {
                        VerifyResult::correct()
                    } else {
                        VerifyResult::wrong(format!("expected {correct}, got {proposed_answer}"))
                    }
                }
                None => VerifyResult::wrong("could not parse date"),
            }
        }
        "add_days" => {
            let date = params["date"].as_str().unwrap_or("");
            let days = params["days"].as_i64().unwrap_or(0);
            match add_days(date, days) {
                Some(correct) => {
                    if proposed_answer.trim() == correct {
                        VerifyResult::correct()
                    } else {
                        VerifyResult::wrong(format!("expected {correct}, got {proposed_answer}"))
                    }
                }
                None => VerifyResult::wrong("could not parse date"),
            }
        }
        "leap_year" => {
            let year = params["year"].as_i64().unwrap_or(0) as i32;
            let correct = is_leap_year(year);
            let proposed = match proposed_answer.trim().to_lowercase().as_str() {
                "true" | "yes" | "1" => true,
                "false" | "no" | "0" => false,
                _ => return VerifyResult::wrong("answer must be true/false"),
            };
            if proposed == correct {
                VerifyResult::correct()
            } else {
                VerifyResult::wrong(format!("expected {correct}, got {proposed}"))
            }
        }
        _ => VerifyResult::wrong(format!("unknown task type: {task_type}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn days_between_same_day() {
        assert_eq!(days_between("2024-01-01", "2024-01-01"), Some(0));
    }

    #[test]
    fn days_between_one_week() {
        assert_eq!(days_between("2024-01-01", "2024-01-08"), Some(7));
    }

    #[test]
    fn days_between_across_year() {
        assert_eq!(days_between("2023-12-31", "2024-01-01"), Some(1));
    }

    #[test]
    fn days_between_leap_year() {
        // 2024 is a leap year — Feb has 29 days
        assert_eq!(days_between("2024-02-28", "2024-03-01"), Some(2));
        // 2023 is not — Feb has 28 days
        assert_eq!(days_between("2023-02-28", "2023-03-01"), Some(1));
    }

    #[test]
    fn day_of_week_known() {
        // 2024-01-01 was a Monday
        assert_eq!(day_of_week("2024-01-01"), Some("Mon".to_string()));
        // 2024-07-04 was a Thursday
        assert_eq!(day_of_week("2024-07-04"), Some("Thu".to_string()));
    }

    #[test]
    fn add_days_basic() {
        assert_eq!(add_days("2024-01-01", 30), Some("2024-01-31".to_string()));
        assert_eq!(add_days("2024-01-31", 1), Some("2024-02-01".to_string()));
    }

    #[test]
    fn add_days_negative() {
        assert_eq!(add_days("2024-01-15", -14), Some("2024-01-01".to_string()));
    }

    #[test]
    fn leap_year_checks() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(1900)); // divisible by 100 but not 400
        assert!(is_leap_year(2000)); // divisible by 400
    }

    // ========== Verification ==========

    #[test]
    fn verify_days_between() {
        let params = json!({"date1": "2024-01-01", "date2": "2024-02-01"});
        assert_eq!(verify("days_between", &params, "31").score, 1.0);
        assert_eq!(verify("days_between", &params, "30").score, 0.0);
    }

    #[test]
    fn verify_day_of_week() {
        let params = json!({"date": "2024-01-01"});
        assert_eq!(verify("day_of_week", &params, "Monday").score, 1.0);
        assert_eq!(verify("day_of_week", &params, "Mon").score, 1.0);
        assert_eq!(verify("day_of_week", &params, "Tuesday").score, 0.0);
    }

    #[test]
    fn verify_add_days() {
        let params = json!({"date": "2024-03-01", "days": 365});
        assert_eq!(verify("add_days", &params, "2025-03-01").score, 1.0);
    }

    #[test]
    fn verify_leap_year() {
        assert_eq!(verify("leap_year", &json!({"year": 2024}), "true").score, 1.0);
        assert_eq!(verify("leap_year", &json!({"year": 2023}), "false").score, 1.0);
        assert_eq!(verify("leap_year", &json!({"year": 2024}), "false").score, 0.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_dates() {
        let p1 = json!({"date1": "2020-01-01", "date2": "2020-12-31"});
        assert_eq!(verify("days_between", &p1, "365").score, 1.0); // 2020 is leap
        assert_eq!(verify("days_between", &p1, "364").score, 0.0);

        let p2 = json!({"date1": "2021-01-01", "date2": "2021-12-31"});
        assert_eq!(verify("days_between", &p2, "364").score, 1.0); // 2021 is not leap
        assert_eq!(verify("days_between", &p2, "365").score, 0.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_century_leap_years() {
        assert!(!is_leap_year(1900), "1900 is NOT leap (div by 100, not 400)");
        assert!(is_leap_year(2000), "2000 IS leap (div by 400)");
        assert!(!is_leap_year(2100), "2100 is NOT leap");
    }

    #[test]
    fn adversarial_negative_days() {
        assert_eq!(days_between("2024-01-10", "2024-01-01"), Some(-9));
    }

    #[test]
    fn adversarial_wrong_day_of_week() {
        let params = json!({"date": "2024-01-01"});
        assert_eq!(verify("day_of_week", &params, "Tuesday").score, 0.0, "Jan 1 2024 was Monday");
        assert_eq!(verify("day_of_week", &params, "Friday").score, 0.0);
    }

    #[test]
    fn adversarial_invalid_date() {
        assert_eq!(days_between("2024-02-30", "2024-03-01"), None, "Feb 30 doesn't exist");
    }
}
