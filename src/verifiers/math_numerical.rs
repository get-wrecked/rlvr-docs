//! Math Numerical Verifier
//!
//! Verifies numerical math answers (GSM8K, MATH, etc.).
//! Extracts the final numerical answer from model output and compares to gold.
//!
//! Supports: integers, decimals, fractions, negative numbers, scientific notation.
//! Normalization: strips whitespace, commas, dollar signs, percent signs,
//! converts fractions to decimals, handles equivalent representations.

use super::VerifyResult;

/// Extract the final numerical answer from a string.
/// Priority order (highest to lowest specificity):
/// 1. #### <number> (GSM8K format — most specific)
/// 2. \boxed{<number>} (MATH format — very specific)
/// 3. "The answer is <number>" / "Answer: <number>" (explicit markers)
/// 4. Last number in text (fallback — DANGEROUS, only if nothing else matches)
///
/// IMPORTANT: "= " is NOT used as a pattern — it causes false positives on
/// intermediate calculations like "5 + 3 = 8" in reasoning chains.
pub fn extract_answer(text: &str) -> Option<f64> {
    // GSM8K format: "#### <number>" — highest priority
    if let Some(val) = extract_gsm8k_format(text) {
        return Some(val);
    }

    // Boxed answer: \boxed{<number>} — second priority
    if let Some(val) = extract_boxed(text) {
        return Some(val);
    }

    // "The answer is <number>" / "Answer: <number>" — explicit markers only
    if let Some(val) = extract_answer_is_pattern(text) {
        return Some(val);
    }

    // Fall back to last number in text
    extract_last_number(text)
}

fn extract_gsm8k_format(text: &str) -> Option<f64> {
    // Search from the end for "####" — can be at start of line or after other text
    for line in text.lines().rev() {
        let trimmed = line.trim();
        if let Some(pos) = trimmed.find("####") {
            let rest = &trimmed[pos + 4..];
            if let Some(val) = parse_number(rest.trim()) {
                return Some(val);
            }
        }
    }
    None
}

fn extract_answer_is_pattern(text: &str) -> Option<f64> {
    let lower = text.to_lowercase();
    // Search from the end for explicit answer markers ONLY.
    // DO NOT include "= " — it matches intermediate calculations like "5 + 3 = 8"
    // and causes catastrophic false positives during training.
    for pattern in &["the answer is ", "the answer is: ", "answer: ", "answer is "] {
        if let Some(pos) = lower.rfind(pattern) {
            let after = &text[pos + pattern.len()..];
            // Take the first number after the pattern
            if let Some(val) = extract_first_number(after) {
                return Some(val);
            }
        }
    }
    None
}

fn extract_boxed(text: &str) -> Option<f64> {
    // Find \boxed{...} — take the last occurrence
    let mut result = None;
    let mut search_from = 0;
    while let Some(start) = text[search_from..].find("\\boxed{") {
        let abs_start = search_from + start + 7; // skip past \boxed{
        if let Some(end) = find_matching_brace(&text[abs_start..]) {
            let content = &text[abs_start..abs_start + end];
            if let Some(val) = parse_number(content.trim()) {
                result = Some(val);
            }
        }
        search_from = abs_start;
    }
    result
}

fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_first_number(text: &str) -> Option<f64> {
    let cleaned = text.trim();
    // Try to parse from the start, allowing common prefixes like $, -, etc.
    let start = cleaned.trim_start_matches(|c: char| !c.is_ascii_digit() && c != '-' && c != '.');
    if start.is_empty() {
        return None;
    }
    // Collect digits, dots, commas, slashes (for fractions), minus
    let num_str: String = start
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',' || *c == '/' || *c == '-')
        .collect();
    parse_number(&num_str)
}

fn extract_last_number(text: &str) -> Option<f64> {
    // Find all number-like substrings, return the last one
    let mut last = None;
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let is_negative_start = bytes[i] == b'-'
            && i + 1 < bytes.len()
            && bytes[i + 1].is_ascii_digit();
        if bytes[i].is_ascii_digit() || is_negative_start {
            let start = i;
            // Skip the leading minus if present
            if is_negative_start {
                i += 1;
            }
            // Collect digits, dots, commas, slashes
            while i < bytes.len()
                && (bytes[i].is_ascii_digit()
                    || bytes[i] == b'.'
                    || bytes[i] == b','
                    || bytes[i] == b'/')
            {
                i += 1;
            }
            let candidate = &text[start..i];
            if let Some(val) = parse_number(candidate) {
                last = Some(val);
            }
            // Ensure progress even if parse failed
            if i == start {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    last
}

/// Parse a number string, handling:
/// - Plain integers: "42"
/// - Decimals: "3.14"
/// - Commas: "1,000,000"
/// - Fractions: "3/4"
/// - Negative: "-5"
/// - Percent cleanup: remove trailing %
pub fn parse_number(s: &str) -> Option<f64> {
    let s = s.trim().trim_end_matches('%').trim_end_matches('$').trim();
    if s.is_empty() {
        return None;
    }

    // Handle fractions: "3/4", "-1/3"
    if let Some(slash_pos) = s.find('/') {
        let numer = s[..slash_pos].trim().replace(',', "").parse::<f64>().ok()?;
        let denom = s[slash_pos + 1..].trim().replace(',', "").parse::<f64>().ok()?;
        if denom == 0.0 {
            return None;
        }
        return Some(numer / denom);
    }

    // Remove commas and parse
    let cleaned = s.replace(',', "");
    cleaned.parse::<f64>().ok()
}

/// Compare two numerical answers with tolerance.
/// Returns 1.0 if they match within relative or absolute tolerance.
pub fn numbers_match(predicted: f64, gold: f64, rel_tol: f64, abs_tol: f64) -> bool {
    if predicted == gold {
        return true;
    }
    let abs_diff = (predicted - gold).abs();
    if abs_diff <= abs_tol {
        return true;
    }
    let denom = gold.abs().max(1e-10);
    abs_diff / denom <= rel_tol
}

/// Verify a model's output against a gold answer.
/// Extracts numbers from both, compares with tolerance.
pub fn verify(model_output: &str, gold_answer: &str) -> VerifyResult {
    let gold = match extract_answer(gold_answer) {
        Some(v) => v,
        None => return VerifyResult::wrong(format!("could not parse gold answer: {gold_answer}")),
    };

    let predicted = match extract_answer(model_output) {
        Some(v) => v,
        None => return VerifyResult::wrong("no number found in model output".to_string()),
    };

    // For integers, require the predicted value to ALSO be an integer (or very close).
    // This prevents 47.6 from rounding to match gold=48.
    let is_integer_gold = gold == gold.round() && gold.abs() < 1e12;
    if is_integer_gold {
        // Predicted must be within 1e-4 of an integer, and that integer must match gold
        let predicted_rounded = predicted.round();
        let is_predicted_integer = (predicted - predicted_rounded).abs() < 1e-3;
        if is_predicted_integer && (predicted_rounded - gold).abs() < 0.5 {
            VerifyResult::correct()
        } else {
            VerifyResult::wrong(format!("predicted {predicted}, expected {gold}"))
        }
    } else if numbers_match(predicted, gold, 1e-4, 1e-6) {
        VerifyResult::correct()
    } else {
        VerifyResult::wrong(format!("predicted {predicted}, expected {gold}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Number Parsing Tests ==========

    #[test]
    fn parse_integers() {
        assert_eq!(parse_number("42"), Some(42.0));
        assert_eq!(parse_number("-7"), Some(-7.0));
        assert_eq!(parse_number("0"), Some(0.0));
        assert_eq!(parse_number("1000000"), Some(1_000_000.0));
    }

    #[test]
    fn parse_with_commas() {
        assert_eq!(parse_number("1,000"), Some(1000.0));
        assert_eq!(parse_number("1,234,567"), Some(1_234_567.0));
    }

    #[test]
    fn parse_decimals() {
        assert_eq!(parse_number("3.14"), Some(3.14));
        assert_eq!(parse_number("-0.5"), Some(-0.5));
        assert_eq!(parse_number("1,234.56"), Some(1234.56));
    }

    #[test]
    fn parse_fractions() {
        assert_eq!(parse_number("3/4"), Some(0.75));
        assert_eq!(parse_number("1/3"), Some(1.0 / 3.0));
        assert_eq!(parse_number("-1/2"), Some(-0.5));
    }

    #[test]
    fn parse_with_dollar_percent() {
        assert_eq!(parse_number("$42"), None); // $ at start isn't stripped by parse_number alone
        assert_eq!(parse_number("42%"), Some(42.0));
    }

    #[test]
    fn parse_empty_and_garbage() {
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("abc"), None);
        assert_eq!(parse_number("/"), None);
    }

    // ========== Answer Extraction Tests ==========

    #[test]
    fn extract_gsm8k_format_answer() {
        let text = "Let me work through this step by step.\nFirst, 5 + 3 = 8.\nThen 8 * 2 = 16.\n#### 16";
        assert_eq!(extract_answer(text), Some(16.0));
    }

    #[test]
    fn extract_gsm8k_with_comma() {
        let text = "The total is 1,000 + 500 = 1,500.\n#### 1,500";
        assert_eq!(extract_answer(text), Some(1500.0));
    }

    #[test]
    fn extract_answer_is_pattern_test() {
        let text = "We calculate step by step and the answer is 42.";
        assert_eq!(extract_answer(text), Some(42.0));
    }

    #[test]
    fn extract_boxed_answer() {
        let text = "Therefore, $x = \\boxed{7}$.";
        assert_eq!(extract_answer(text), Some(7.0));
    }

    #[test]
    fn extract_boxed_fraction() {
        let text = "The probability is \\boxed{3/4}.";
        assert_eq!(extract_answer(text), Some(0.75));
    }

    #[test]
    fn extract_last_number_fallback() {
        let text = "We get 5 then 10 then 15.";
        assert_eq!(extract_answer(text), Some(15.0));
    }

    #[test]
    fn extract_negative_answer() {
        let text = "#### -3";
        assert_eq!(extract_answer(text), Some(-3.0));
    }

    // ========== Verification Tests ==========
    // These test the VERIFIER, not the math. We use known GSM8K-style problems.

    #[test]
    fn verify_correct_integer() {
        let result = verify(
            "Step 1: 5 + 3 = 8\nStep 2: 8 * 6 = 48\n#### 48",
            "#### 48",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_wrong_integer() {
        let result = verify(
            "Step 1: 5 + 3 = 8\nStep 2: 8 * 6 = 47\n#### 47",
            "#### 48",
        );
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_correct_decimal() {
        let result = verify(
            "The answer is 3.14159",
            "3.14159",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_decimal_tolerance() {
        // 3.14159 vs 3.14160 — within 1e-4 relative tolerance
        let result = verify("The answer is 3.14160", "3.14159");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_fraction_vs_decimal() {
        // Model outputs fraction, gold is decimal
        let result = verify("The answer is 3/4", "0.75");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_no_number_in_output() {
        let result = verify("I don't know the answer", "#### 42");
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_comma_formatted_answer() {
        let result = verify("#### 1,500", "#### 1500");
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding Tests ==========
    // These verify the verifier actually does comparison, not just returning 1.0

    #[test]
    fn antihardcode_different_numbers_always_fail() {
        // Verify that DIFFERENT answers produce score 0.0
        for (pred, gold) in &[
            ("#### 1", "#### 2"),
            ("#### 100", "#### 200"),
            ("#### -5", "#### 5"),
            ("#### 0", "#### 1"),
            ("#### 999", "#### 1000"),
        ] {
            let result = verify(pred, gold);
            assert_eq!(result.score, 0.0, "Expected 0.0 for pred={pred}, gold={gold}");
        }
    }

    #[test]
    fn antihardcode_same_numbers_always_pass() {
        for n in &["0", "1", "42", "-7", "3.14", "1000000", "1/3"] {
            let gold = format!("#### {n}");
            let pred = format!("The answer is {n}");
            let result = verify(&pred, &gold);
            assert_eq!(result.score, 1.0, "Expected 1.0 for n={n}");
        }
    }

    // ========== Real GSM8K Examples ==========
    // These are actual GSM8K problems with known answers.

    #[test]
    fn gsm8k_extract_gold_18() {
        // Isolate: can we extract 18 from this gold answer?
        let gold = "She eats 3 and bakes 4 so uses 3+4=7. 16-7=9 eggs sold. 9*2=$18. #### 18";
        assert_eq!(extract_answer(gold), Some(18.0));
    }

    #[test]
    fn gsm8k_extract_model_18() {
        let model = "Janet has 16 eggs. She uses 3+4=7. She sells 16-7=9 eggs at $2 each = $18.\n#### 18";
        assert_eq!(extract_answer(model), Some(18.0));
    }

    #[test]
    fn gsm8k_real_example_1() {
        let gold = "#### 18";
        let model = "#### 18";
        assert_eq!(verify(model, gold).score, 1.0);

        let bad_model = "#### 32";
        assert_eq!(verify(bad_model, gold).score, 0.0);
    }

    #[test]
    fn gsm8k_real_example_2() {
        let gold = "Half of 2 is 1. 2+1=3. #### 3";
        let model = "Blue: 2 bolts. White: 2/2 = 1 bolt. Total: 2+1 = 3 bolts. The answer is 3";
        assert_eq!(verify(model, gold).score, 1.0);
    }

    // ========== ADVERSARIAL TESTS — False Positive Prevention ==========
    // These test scenarios where a naive verifier would give credit but shouldn't.

    #[test]
    fn adversarial_correct_intermediate_wrong_final() {
        // Model's reasoning mentions the correct answer (48) but gives wrong final answer
        // A naive "= " matcher would extract 48 from "8 * 6 = 48" in the reasoning
        let gold = "#### 48";
        let model = "Step 1: 5 + 3 = 8. Step 2: 8 * 6 = 48. But wait, I need to subtract 10.\n#### 38";
        assert_eq!(verify(model, gold).score, 0.0, "Should use #### answer, not intermediate = 48");
    }

    #[test]
    fn adversarial_rounding_exploitation() {
        // 47.6 should NOT round to match gold=48
        let gold = "#### 48";
        let model = "The answer is 47.6";
        assert_eq!(verify(model, gold).score, 0.0, "47.6 should not match integer gold 48");

        // 47.9999 SHOULD match 48 (within integer tolerance)
        let model2 = "The answer is 47.9999";
        assert_eq!(verify(model2, gold).score, 1.0, "47.9999 is close enough to 48");
    }

    #[test]
    fn adversarial_off_by_one() {
        // Common model error — off by one
        let gold = "#### 100";
        assert_eq!(verify("#### 99", gold).score, 0.0);
        assert_eq!(verify("#### 101", gold).score, 0.0);
        assert_eq!(verify("#### 100", gold).score, 1.0);
    }

    #[test]
    fn adversarial_sign_error() {
        // Model gets the magnitude right but wrong sign
        let gold = "#### -15";
        assert_eq!(verify("#### 15", gold).score, 0.0, "Wrong sign must fail");
        assert_eq!(verify("#### -15", gold).score, 1.0);

        let gold2 = "#### 15";
        assert_eq!(verify("#### -15", gold2).score, 0.0, "Wrong sign must fail");
    }

    #[test]
    fn adversarial_zero_vs_nonzero() {
        let gold = "#### 0";
        assert_eq!(verify("#### 0", gold).score, 1.0);
        assert_eq!(verify("#### 1", gold).score, 0.0);
        assert_eq!(verify("#### -1", gold).score, 0.0);
        assert_eq!(verify("#### 0.001", gold).score, 0.0, "0.001 is not 0 for integer gold");
    }

    #[test]
    fn adversarial_large_numbers() {
        let gold = "#### 1000000";
        assert_eq!(verify("#### 1,000,000", gold).score, 1.0);
        assert_eq!(verify("#### 999999", gold).score, 0.0);
        assert_eq!(verify("#### 1000001", gold).score, 0.0);
    }

    #[test]
    fn adversarial_multiple_hash_lines() {
        // Multiple #### in output — should take the LAST one
        let gold = "#### 42";
        let model = "First attempt: #### 30\nWait, let me recalculate.\n#### 42";
        assert_eq!(verify(model, gold).score, 1.0, "Should use last #### line");

        let model2 = "#### 42\nActually, I made an error.\n#### 30";
        assert_eq!(verify(model2, gold).score, 0.0, "Should use last #### (30), not first (42)");
    }

    #[test]
    fn adversarial_fraction_in_reasoning() {
        // "2/2 = 1" in reasoning should not be extracted as the answer
        let gold = "#### 5";
        let model = "We compute 2/2 = 1, then add 4.\n#### 5";
        assert_eq!(verify(model, gold).score, 1.0, "#### takes priority over fraction in reasoning");
    }

    #[test]
    fn adversarial_division_by_zero() {
        assert_eq!(parse_number("1/0"), None, "Division by zero should return None");
        assert_eq!(parse_number("0/0"), None);
    }

    #[test]
    fn adversarial_nan_inf() {
        // Model outputs NaN or Infinity — should never match any gold
        let gold = "#### 42";
        assert_eq!(verify("NaN", gold).score, 0.0);
        assert_eq!(verify("Infinity", gold).score, 0.0);
        assert_eq!(verify("inf", gold).score, 0.0);
    }

    #[test]
    fn adversarial_empty_output() {
        let gold = "#### 42";
        assert_eq!(verify("", gold).score, 0.0);
        assert_eq!(verify("   ", gold).score, 0.0);
        assert_eq!(verify("\n\n\n", gold).score, 0.0);
    }

    #[test]
    fn adversarial_model_echoes_question() {
        // Model repeats the question which contains numbers, but doesn't answer
        let gold = "#### 18";
        let model = "Janet's ducks lay 16 eggs per day. She eats 3 and bakes 4.";
        // Falls back to last number = 4. Should NOT match 18.
        assert_eq!(verify(model, gold).score, 0.0);
    }

    #[test]
    fn adversarial_decimal_gold_integer_pred() {
        // Gold is 3.5, model says 4 (rounded up) or 3 (truncated)
        let gold = "3.5";
        assert_eq!(verify("#### 4", gold).score, 0.0, "4 != 3.5");
        assert_eq!(verify("#### 3", gold).score, 0.0, "3 != 3.5");
        assert_eq!(verify("#### 3.5", gold).score, 1.0);
    }

    #[test]
    fn adversarial_very_close_but_wrong() {
        // For integer gold, even 47.5 should not match 48
        let gold = "#### 48";
        assert_eq!(verify("#### 47.5", gold).score, 0.0, "47.5 is not close enough to integer 48");
    }

    #[test]
    fn adversarial_boxed_vs_hash_conflict() {
        // If both #### and \boxed{} are present, #### takes priority (more specific)
        let gold = "#### 42";
        let model = "I think \\boxed{30} but actually #### 42";
        assert_eq!(verify(model, gold).score, 1.0, "#### should take priority");
    }

    // ========== GSM8K Full Corpus Validation ==========

    #[test]
    fn gsm8k_corpus_no_extraction_failures() {
        let path = "raw/datasets/math/gsm8k_test.jsonl";
        if !std::path::Path::new(path).exists() { return; }

        let problems = crate::datasets::gsm8k::load(path).unwrap();
        let mut failures = Vec::new();
        for (i, p) in problems.iter().enumerate() {
            let gold = extract_answer(&p.answer);
            if gold.is_none() {
                failures.push(format!("#{i}: {}", &p.answer[..p.answer.len().min(80)]));
            }
        }
        assert!(failures.is_empty(), "Failed to extract from {} problems:\n{}",
            failures.len(), failures.join("\n"));
    }

    #[test]
    fn gsm8k_corpus_self_consistency() {
        // Verify that every GSM8K gold answer matches itself
        let path = "raw/datasets/math/gsm8k_test.jsonl";
        if !std::path::Path::new(path).exists() { return; }

        let problems = crate::datasets::gsm8k::load(path).unwrap();
        let mut failures = Vec::new();
        for (i, p) in problems.iter().enumerate() {
            let result = verify(&p.answer, &p.answer);
            if result.score != 1.0 {
                failures.push(format!("#{i}: self-verify failed: {}", result.reason));
            }
        }
        assert!(failures.is_empty(), "{} self-consistency failures:\n{}",
            failures.len(), failures.join("\n"));
    }
}
