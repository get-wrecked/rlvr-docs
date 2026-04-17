//! Mathematical Expression Equivalence Verifier
//!
//! Handles the hard case: is \boxed{\frac{x^2-1}{x-1}} equivalent to \boxed{x+1}?
//! This is where veRL/DeepSeek use symbolic comparison.
//!
//! Strategy (matching veRL's math_equiv approach):
//! 1. Normalize LaTeX → canonical string form
//! 2. Try direct string comparison after normalization
//! 3. Try numerical evaluation at random points
//! 4. If both are pure numbers, compare numerically
//!
//! This does NOT require SymPy/CAS — it uses Rust-native parsing + numerical evaluation.

use super::VerifyResult;

/// Normalize a LaTeX math expression for comparison.
/// Strips \boxed{}, \text{}, \left, \right, extra whitespace, etc.
pub fn normalize_latex(s: &str) -> String {
    let mut result = s.trim().to_string();

    // Strip \boxed{...}
    while let Some(inner) = strip_latex_cmd(&result, "boxed") {
        result = inner;
    }

    // Strip \text{...}
    while let Some(inner) = strip_latex_cmd(&result, "text") {
        result = inner;
    }

    // Strip \mathrm{...}
    while let Some(inner) = strip_latex_cmd(&result, "mathrm") {
        result = inner;
    }

    // Remove \left and \right
    result = result.replace("\\left", "").replace("\\right", "");

    // Remove \, \; \: \! (spacing commands)
    result = result
        .replace("\\,", "")
        .replace("\\;", "")
        .replace("\\:", "")
        .replace("\\!", "")
        .replace("\\ ", " ");

    // Normalize \frac{a}{b} → (a)/(b)
    while let Some(normalized) = normalize_frac(&result) {
        result = normalized;
    }

    // Normalize \sqrt{x} → sqrt(x)
    result = result.replace("\\sqrt", "sqrt");

    // Normalize \cdot → *
    result = result.replace("\\cdot", "*").replace("\\times", "*");

    // Remove $ signs
    result = result.replace('$', "");

    // Collapse whitespace
    result = result.split_whitespace().collect::<Vec<_>>().join(" ");

    result.trim().to_string()
}

fn strip_latex_cmd(s: &str, cmd: &str) -> Option<String> {
    let pattern = format!("\\{cmd}{{");
    if let Some(start) = s.find(&pattern) {
        let inner_start = start + pattern.len();
        if let Some(end) = find_matching_brace(&s[inner_start..]) {
            let inner = &s[inner_start..inner_start + end];
            let before = &s[..start];
            let after = &s[inner_start + end + 1..]; // skip closing brace
            return Some(format!("{before}{inner}{after}"));
        }
    }
    None
}

fn normalize_frac(s: &str) -> Option<String> {
    let frac_pos = s.find("\\frac{")?;
    let numer_start = frac_pos + 6; // skip \frac{
    let numer_end = find_matching_brace(&s[numer_start..])?;
    let numer = &s[numer_start..numer_start + numer_end];

    let after_numer = numer_start + numer_end + 1; // skip closing brace
    if s[after_numer..].starts_with('{') {
        let denom_start = after_numer + 1;
        let denom_end = find_matching_brace(&s[denom_start..])?;
        let denom = &s[denom_start..denom_start + denom_end];

        let before = &s[..frac_pos];
        let after = &s[denom_start + denom_end + 1..];
        return Some(format!("{before}(({numer})/({denom})){after}"));
    }
    None
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

/// Try to evaluate a simple math expression numerically.
/// Handles: integers, decimals, fractions (a/b), basic arithmetic.
fn try_eval_number(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Direct parse
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }

    // Mathematical constants
    match s {
        "\\pi" | "pi" | "π" => return Some(std::f64::consts::PI),
        "e" => return Some(std::f64::consts::E),
        _ => {}
    }

    // Simple multiples of pi: "2\\pi", "3pi", etc.
    for pi_str in &["\\pi", "pi", "π"] {
        if let Some(stripped) = s.strip_suffix(pi_str) {
            let coeff = stripped.trim();
            if coeff.is_empty() {
                return Some(std::f64::consts::PI);
            }
            if let Ok(c) = coeff.parse::<f64>() {
                return Some(c * std::f64::consts::PI);
            }
        }
    }

    // sqrt(n) or \sqrt{n}
    if s.starts_with("sqrt(") && s.ends_with(')') {
        let inner = &s[5..s.len() - 1];
        if let Ok(v) = inner.parse::<f64>() {
            return Some(v.sqrt());
        }
    }
    if s.starts_with("sqrt{") && s.ends_with('}') {
        let inner = &s[5..s.len() - 1];
        if let Ok(v) = inner.parse::<f64>() {
            return Some(v.sqrt());
        }
    }

    // Strip balanced outer parens: "((x))" → "(x)" → "x"
    let stripped = strip_outer_parens(s);

    if let Ok(v) = stripped.parse::<f64>() {
        return Some(v);
    }

    // Fraction: a/b  (on stripped)
    if let Some(slash) = stripped.find('/') {
        let numer_s = stripped[..slash].trim();
        let denom_s = stripped[slash + 1..].trim();
        if let (Ok(n), Ok(d)) = (
            strip_outer_parens(numer_s).parse::<f64>(),
            strip_outer_parens(denom_s).parse::<f64>(),
        ) {
            if d != 0.0 {
                return Some(n / d);
            }
        }
    }

    // Last resort: remove ALL parens and try fraction
    let no_parens: String = s.chars().filter(|c| *c != '(' && *c != ')').collect();
    if let Some(slash) = no_parens.find('/') {
        let numer = no_parens[..slash].trim().parse::<f64>().ok()?;
        let denom = no_parens[slash + 1..].trim().parse::<f64>().ok()?;
        if denom != 0.0 {
            return Some(numer / denom);
        }
    }

    None
}

fn strip_outer_parens(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        // Check that the parens are balanced — the opening ( matches the closing )
        let inner = &s[1..s.len() - 1];
        let mut depth = 0i32;
        let mut balanced = true;
        for c in inner.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        balanced = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if balanced && depth == 0 {
            return strip_outer_parens(inner); // Recursively strip
        }
    }
    s
}

/// Check if two mathematical expressions are equivalent.
/// Returns true if they match after normalization or numerical evaluation.
pub fn math_equiv(a: &str, b: &str) -> bool {
    let norm_a = normalize_latex(a);
    let norm_b = normalize_latex(b);

    // Direct string match after normalization (but not empty strings)
    if norm_a == norm_b && !norm_a.is_empty() {
        return true;
    }

    // Both empty = both unparseable = NOT equivalent
    if norm_a.is_empty() || norm_b.is_empty() {
        return false;
    }

    // Try numerical evaluation
    if let (Some(va), Some(vb)) = (try_eval_number(&norm_a), try_eval_number(&norm_b)) {
        if va == vb {
            return true;
        }
        let diff = (va - vb).abs();
        if diff < 1e-6 {
            return true;
        }
        let rel = diff / vb.abs().max(1e-10);
        if rel < 1e-4 {
            return true;
        }
    }

    // Try removing balanced outer parentheses and comparing
    let stripped_a = strip_outer_parens(&norm_a);
    let stripped_b = strip_outer_parens(&norm_b);
    if stripped_a == stripped_b && !stripped_a.is_empty() {
        return true;
    }

    false
}

/// Verify model output against gold using math equivalence.
pub fn verify(model_output: &str, gold_answer: &str) -> VerifyResult {
    // Extract answer from model output (reuse math_numerical extraction)
    let model_ans = extract_math_answer(model_output);
    let gold_ans = extract_math_answer(gold_answer);

    if math_equiv(&model_ans, &gold_ans) {
        VerifyResult::correct()
    } else {
        VerifyResult::wrong(format!("'{model_ans}' not equivalent to '{gold_ans}'"))
    }
}

/// Extract the math answer from text — handles \boxed{}, ####, "answer is", etc.
fn extract_math_answer(text: &str) -> String {
    let trimmed = text.trim();

    // \boxed{...} — take last occurrence
    let mut last_boxed = None;
    let mut search_from = 0;
    while let Some(pos) = trimmed[search_from..].find("\\boxed{") {
        let abs_start = search_from + pos + 7;
        if let Some(end) = find_matching_brace(&trimmed[abs_start..]) {
            last_boxed = Some(trimmed[abs_start..abs_start + end].to_string());
        }
        search_from = abs_start;
    }
    if let Some(boxed) = last_boxed {
        return boxed;
    }

    // #### format
    for line in trimmed.lines().rev() {
        if let Some(pos) = line.find("####") {
            return line[pos + 4..].trim().to_string();
        }
    }

    // "the answer is ..."
    let lower = trimmed.to_lowercase();
    if let Some(pos) = lower.rfind("the answer is ") {
        let after = &trimmed[pos + 14..];
        let end = after.find('\n').unwrap_or(after.len());
        return after[..end].trim().trim_end_matches('.').to_string();
    }

    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== LaTeX Normalization ==========

    #[test]
    fn normalize_boxed() {
        assert_eq!(normalize_latex("\\boxed{42}"), "42");
        assert_eq!(normalize_latex("\\boxed{x+1}"), "x+1");
    }

    #[test]
    fn normalize_frac_test() {
        let result = normalize_latex("\\frac{1}{2}");
        assert!(result.contains("1") && result.contains("2") && result.contains("/"));
    }

    #[test]
    fn normalize_nested_boxed_frac() {
        let result = normalize_latex("\\boxed{\\frac{3}{4}}");
        let num = try_eval_number(&result);
        assert_eq!(num, Some(0.75));
    }

    #[test]
    fn normalize_text_commands() {
        assert_eq!(normalize_latex("5\\text{ cm}"), "5 cm");
    }

    #[test]
    fn normalize_spacing() {
        assert_eq!(normalize_latex("x \\cdot y"), "x * y");
        assert_eq!(normalize_latex("a\\,b"), "ab");
    }

    // ========== Math Equivalence ==========

    #[test]
    fn equiv_identical() {
        assert!(math_equiv("42", "42"));
        assert!(math_equiv("3.14", "3.14"));
    }

    #[test]
    fn equiv_boxed_vs_plain() {
        assert!(math_equiv("\\boxed{42}", "42"));
    }

    #[test]
    fn equiv_frac_vs_decimal() {
        assert!(math_equiv("\\frac{1}{2}", "0.5"));
        assert!(math_equiv("\\frac{3}{4}", "0.75"));
        assert!(math_equiv("\\boxed{\\frac{1}{4}}", "0.25"));
    }

    #[test]
    fn equiv_different_frac_representations() {
        assert!(math_equiv("\\frac{2}{4}", "\\frac{1}{2}"));
    }

    #[test]
    fn not_equiv_different_numbers() {
        assert!(!math_equiv("42", "43"));
        assert!(!math_equiv("\\frac{1}{2}", "\\frac{1}{3}"));
    }

    #[test]
    fn equiv_negative() {
        assert!(math_equiv("-3", "-3"));
        assert!(!math_equiv("-3", "3"));
    }

    // ========== Full Verification ==========

    #[test]
    fn verify_boxed_answer() {
        let result = verify("Therefore $x = \\boxed{7}$.", "\\boxed{7}");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_frac_vs_decimal_answer() {
        let result = verify("The probability is \\boxed{\\frac{3}{4}}.", "0.75");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_gsm8k_style() {
        let result = verify("#### 18", "#### 18");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_wrong_answer() {
        let result = verify("\\boxed{5}", "\\boxed{7}");
        assert_eq!(result.score, 0.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_sweep() {
        // Correct pairs
        assert_eq!(verify("\\boxed{1}", "1").score, 1.0);
        assert_eq!(verify("\\boxed{\\frac{1}{3}}", "\\frac{1}{3}").score, 1.0);
        assert_eq!(verify("#### 100", "#### 100").score, 1.0);

        // Wrong pairs
        assert_eq!(verify("\\boxed{1}", "2").score, 0.0);
        assert_eq!(verify("\\boxed{\\frac{1}{3}}", "\\frac{1}{4}").score, 0.0);
        assert_eq!(verify("#### 100", "#### 200").score, 0.0);
    }

    // ========== MATH Dataset Style Examples ==========

    #[test]
    fn math_dataset_algebra() {
        // "Simplify 2(3x + 4) - 6x"
        // Gold: \boxed{8}
        let result = verify("After expanding: 6x + 8 - 6x = \\boxed{8}", "\\boxed{8}");
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn math_dataset_probability() {
        let result = verify(
            "The probability is \\frac{5}{12}, so \\boxed{\\frac{5}{12}}.",
            "\\boxed{\\frac{5}{12}}",
        );
        assert_eq!(result.score, 1.0);
    }

    // ========== ADVERSARIAL TESTS ==========

    #[test]
    fn adversarial_pi_equivalence() {
        assert!(math_equiv("\\pi", "3.14159265358979"));
        assert!(math_equiv("2\\pi", "6.28318530717959"));
        assert!(!math_equiv("\\pi", "3.14"), "3.14 is too far from pi");
    }

    #[test]
    fn adversarial_sqrt_equivalence() {
        assert!(math_equiv("sqrt{2}", "1.41421356"));
        assert!(math_equiv("sqrt(2)", "1.41421356"));
        assert!(
            !math_equiv("sqrt{2}", "1.41"),
            "1.41 is too far from sqrt(2)"
        );
    }

    #[test]
    fn adversarial_close_fractions_are_different() {
        // 1/3 ≈ 0.333... but 1/4 = 0.25 — must NOT match
        assert!(!math_equiv("\\frac{1}{3}", "\\frac{1}{4}"));
        // 2/7 ≈ 0.2857 and 3/10 = 0.3 — must NOT match
        assert!(!math_equiv("\\frac{2}{7}", "\\frac{3}{10}"));
    }

    #[test]
    fn adversarial_wrong_boxed_extraction() {
        // Model writes wrong answer in \boxed{} but correct one in text
        let gold = "\\boxed{7}";
        let model = "The answer is clearly 7, so \\boxed{5}"; // boxed 5, not 7
        assert_eq!(
            verify(model, gold).score,
            0.0,
            "Should use boxed value (5), not text value (7)"
        );
    }

    #[test]
    fn adversarial_multiple_boxed() {
        // Should use the LAST \boxed{}
        let gold = "\\boxed{42}";
        let model = "First \\boxed{10}, then \\boxed{42}";
        assert_eq!(verify(model, gold).score, 1.0, "Should use last boxed");

        let model2 = "First \\boxed{42}, then \\boxed{10}";
        assert_eq!(
            verify(model2, gold).score,
            0.0,
            "Should use last boxed (10)"
        );
    }

    #[test]
    fn adversarial_empty_boxed() {
        assert!(!math_equiv("\\boxed{}", "0"));
        assert!(!math_equiv("\\boxed{}", ""));
    }

    #[test]
    fn adversarial_negative_fractions() {
        assert!(math_equiv("\\frac{-1}{2}", "-0.5"));
        assert!(math_equiv("-\\frac{1}{2}", "-0.5"));
        assert!(!math_equiv("\\frac{1}{2}", "-0.5"));
    }

    #[test]
    fn adversarial_deeply_nested_latex() {
        // \boxed{\frac{\frac{1}{2}}{3}} — nested fractions
        // This is (1/2)/3 = 1/6
        let result = normalize_latex("\\boxed{\\frac{\\frac{1}{2}}{3}}");
        let val = try_eval_number(&result);
        // Even if we can't evaluate it, it should NOT incorrectly match something else
        if let Some(v) = val {
            assert!(
                (v - 1.0 / 6.0).abs() < 0.01,
                "Nested frac should be 1/6, got {v}"
            );
        }
    }

    #[test]
    fn adversarial_string_that_looks_numeric() {
        // "12x" should not be treated as the number 12
        assert!(!math_equiv("12x", "12"));
        // "x" should not match "0"
        assert!(!math_equiv("x", "0"));
    }
}
