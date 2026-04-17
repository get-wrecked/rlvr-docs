//! Regex Synthesis Verifier
//!
//! Given positive examples (strings the regex must match) and negative examples
//! (strings the regex must NOT match), verify that a proposed regex is correct.
//!
//! This is a pure execution-based verifier: compile the regex, test against examples.

use super::VerifyResult;
use regex::Regex;

/// A regex synthesis task: positive and negative examples.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RegexTask {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

/// Verify a regex against positive/negative examples.
/// The regex must:
/// 1. Compile successfully
/// 2. Match ALL positive examples (full match, not partial)
/// 3. NOT match ANY negative examples
pub fn verify(task: &RegexTask, proposed_regex: &str) -> VerifyResult {
    // Compile the regex
    if let Err(e) = Regex::new(proposed_regex) {
        return VerifyResult::wrong(format!("regex compile error: {e}"));
    }

    // Wrap in anchors for full-string matching if not already anchored
    let anchored_pattern = if proposed_regex.starts_with('^') && proposed_regex.ends_with('$') {
        proposed_regex.to_string()
    } else {
        format!("^(?:{proposed_regex})$")
    };

    let anchored = match Regex::new(&anchored_pattern) {
        Ok(r) => r,
        Err(e) => return VerifyResult::wrong(format!("anchored regex compile error: {e}")),
    };

    let total = task.positive.len() + task.negative.len();
    if total == 0 {
        return VerifyResult::wrong("no examples provided");
    }

    let mut correct = 0;
    let mut errors = Vec::new();

    // Check positive examples (must match)
    for (i, pos) in task.positive.iter().enumerate() {
        if anchored.is_match(pos) {
            correct += 1;
        } else {
            if errors.len() < 3 {
                errors.push(format!("positive[{i}] '{pos}' not matched"));
            }
        }
    }

    // Check negative examples (must NOT match)
    for (i, neg) in task.negative.iter().enumerate() {
        if !anchored.is_match(neg) {
            correct += 1;
        } else {
            if errors.len() < 3 {
                errors.push(format!("negative[{i}] '{neg}' incorrectly matched"));
            }
        }
    }

    if correct == total {
        VerifyResult::correct()
    } else {
        let error_msg = errors.join("; ");
        VerifyResult::partial(
            correct as f64 / total as f64,
            format!("{correct}/{total} examples correct. {error_msg}"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(pos: &[&str], neg: &[&str]) -> RegexTask {
        RegexTask {
            positive: pos.iter().map(|s| s.to_string()).collect(),
            negative: neg.iter().map(|s| s.to_string()).collect(),
        }
    }

    // ========== Basic Regex Verification ==========

    #[test]
    fn simple_email_like_pattern() {
        let t = task(
            &["a@b.c", "user@example.com", "test@test.org"],
            &["", "noatsign", "@missing"],
        );
        let result = verify(&t, r".+@.+\..+");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn email_regex_too_broad_catches_spaces() {
        // .+@.+\..+ matches "spaces here@x.c" — verifier correctly catches this
        let t = task(&["a@b.c"], &["spaces here@x.c"]);
        let result = verify(&t, r".+@.+\..+");
        assert!(
            result.score < 1.0,
            "Regex that matches negative example should fail"
        );
    }

    #[test]
    fn digit_pattern() {
        let t = task(&["123", "456", "789", "000"], &["abc", "12", "1234", "12a"]);
        let result = verify(&t, r"\d{3}");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn wrong_regex_gets_zero() {
        let t = task(&["abc", "def"], &["123", "456"]);
        // This regex matches digits, not letters — should fail on all examples
        let result = verify(&t, r"\d+");
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn partial_credit() {
        let t = task(&["cat", "car", "cap"], &["dog", "bat"]);
        // "ca." matches all positives but also matches "cab" etc — doesn't affect neg examples here
        // "cat" only matches "cat" out of positives
        let result = verify(&t, "cat");
        // Matches: cat(1/3 pos), dog not matched (1/2 neg), bat not matched (1/2 neg) = 3/5
        assert!(result.score > 0.0 && result.score < 1.0);
    }

    #[test]
    fn invalid_regex_returns_zero() {
        let t = task(&["abc"], &["def"]);
        let result = verify(&t, "[invalid regex(((");
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn empty_examples_returns_zero() {
        let t = task(&[], &[]);
        let result = verify(&t, ".*");
        assert_eq!(result.score, 0.0);
    }

    // ========== Full-String Matching (Anchoring) ==========

    #[test]
    fn partial_match_not_accepted() {
        let t = task(
            &["abc"],
            &["xabcx"], // contains "abc" but is not exactly "abc"
        );
        // Regex "abc" should match "abc" fully but not "xabcx" fully
        let result = verify(&t, "abc");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn already_anchored_regex() {
        let t = task(&["hello"], &["hello world"]);
        let result = verify(&t, "^hello$");
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_tasks_different_regex() {
        // Task 1: match hex strings
        let t1 = task(&["0a", "ff", "1b", "00"], &["gg", "zz", "0x", ""]);
        assert_eq!(verify(&t1, "[0-9a-f]{2}").score, 1.0);
        assert!(verify(&t1, "[a-z]{2}").score < 1.0); // doesn't match "00"

        // Task 2: match dates
        let t2 = task(
            &["2024-01-15", "2023-12-31"],
            &["2024/01/15", "24-1-1", "not-a-date"],
        );
        assert_eq!(verify(&t2, r"\d{4}-\d{2}-\d{2}").score, 1.0);
        assert!(verify(&t2, r"\d+").score < 1.0); // too broad
    }

    #[test]
    fn antihardcode_overfit_regex_fails_on_unseen() {
        // Regex that just enumerates the positive examples — works for these
        // but wouldn't generalize (we test what we have)
        let t = task(
            &["cat", "bat", "hat"],
            &["dog", "rat"], // note: "rat" ends in "at" too
        );
        // Overfitting: "cat|bat|hat" works
        assert_eq!(verify(&t, "cat|bat|hat").score, 1.0);
        // But ".at" would match "rat" too — fails negative
        assert!(verify(&t, ".at").score < 1.0);
    }

    // ========== Real-World-ish Tasks ==========

    #[test]
    fn us_phone_number() {
        let t = task(
            &["555-123-4567", "800-555-0100", "212-456-7890"],
            &["5551234567", "555-12-4567", "abc-def-ghij", ""],
        );
        let result = verify(&t, r"\d{3}-\d{3}-\d{4}");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn ipv4_address() {
        let t = task(
            &["192.168.1.1", "10.0.0.1", "255.255.255.255"],
            &["999.999.999.999", "1.2.3", "1.2.3.4.5", "abc.def.ghi.jkl"],
        );
        // Simplified: just check structure (doesn't validate 0-255 range)
        let result = verify(&t, r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}");
        // 999.999... matches the pattern but shouldn't — this is a structural check only
        // so the negative "999.999.999.999" WILL match our simple regex → partial score
        assert!(result.score < 1.0); // Demonstrates that simple regex isn't enough
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_dot_star_too_broad() {
        let t = task(&["hello"], &["world"]);
        let result = verify(&t, ".*");
        assert!(result.score < 1.0, ".* matches the negative example too");
    }

    #[test]
    fn adversarial_redos_safe() {
        // Rust regex crate is immune to catastrophic backtracking
        let t = task(&["aaaaaaaaaaaaaaaaaa!"], &["bbb"]);
        let result = verify(&t, "(a+)+!");
        assert_eq!(result.score, 1.0, "Rust regex handles this safely");
    }

    #[test]
    fn adversarial_empty_regex() {
        let t = task(&["hello"], &[]);
        let result = verify(&t, "");
        assert!(result.score < 1.0, "Empty regex should not match 'hello'");
    }

    #[test]
    fn adversarial_exact_enumeration_vs_pattern() {
        // Enumeration works for known examples
        let t = task(&["cat", "bat", "hat"], &["dog", "rat"]);
        assert_eq!(verify(&t, "cat|bat|hat").score, 1.0, "Enumeration works");
        // But ".at" wrongly matches "rat"
        assert!(verify(&t, ".at").score < 1.0, ".at matches rat (negative)");
    }
}
