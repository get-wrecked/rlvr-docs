//! Exact Match / F1 Verifier
//!
//! Covers many QA and classification domains:
//! - question-answering-extractive (SQuAD-style)
//! - question-answering-closed (TriviaQA-style)
//! - fact-verification (FEVER-style)
//! - text-classification (label matching)
//! - commonsense-reasoning (multiple choice)
//! - entity-linking, temporal-reasoning, etc.
//!
//! Supports: exact match, normalized exact match, token F1, multiple valid answers.

use super::VerifyResult;

/// Normalize text for comparison: lowercase, strip articles/punctuation/whitespace.
pub fn normalize(text: &str) -> String {
    let lower = text.to_lowercase();
    // Remove punctuation, collapse whitespace
    let cleaned: String = lower
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect();
    // Split into tokens, remove articles, rejoin
    let tokens: Vec<&str> = cleaned
        .split_whitespace()
        .filter(|t| *t != "a" && *t != "an" && *t != "the")
        .collect();
    tokens.join(" ")
}

/// Check exact match after normalization.
pub fn exact_match(prediction: &str, gold: &str) -> bool {
    normalize(prediction) == normalize(gold)
}

/// Check exact match against multiple valid gold answers.
pub fn exact_match_any(prediction: &str, golds: &[&str]) -> bool {
    golds.iter().any(|g| exact_match(prediction, g))
}

/// Compute token-level F1 score (SQuAD-style).
pub fn token_f1(prediction: &str, gold: &str) -> f64 {
    let pred_norm = normalize(prediction);
    let gold_norm = normalize(gold);
    let pred_tokens: Vec<&str> = pred_norm.split_whitespace().collect();
    let gold_tokens: Vec<&str> = gold_norm.split_whitespace().collect();

    if gold_tokens.is_empty() && pred_tokens.is_empty() {
        return 1.0;
    }
    if gold_tokens.is_empty() || pred_tokens.is_empty() {
        return 0.0;
    }

    // Count common tokens using multiset intersection (handles duplicates correctly)
    let mut gold_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for t in &gold_tokens {
        *gold_counts.entry(t).or_insert(0) += 1;
    }
    let mut pred_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for t in &pred_tokens {
        *pred_counts.entry(t).or_insert(0) += 1;
    }
    let common: usize = gold_counts
        .iter()
        .map(|(token, &gold_count)| {
            let pred_count = pred_counts.get(token).copied().unwrap_or(0);
            gold_count.min(pred_count)
        })
        .sum();

    if common == 0 {
        return 0.0;
    }

    let precision = common as f64 / pred_tokens.len() as f64;
    let recall = common as f64 / gold_tokens.len() as f64;
    2.0 * precision * recall / (precision + recall)
}

/// Best F1 across multiple gold answers.
pub fn best_f1(prediction: &str, golds: &[&str]) -> f64 {
    golds
        .iter()
        .map(|g| token_f1(prediction, g))
        .fold(0.0f64, f64::max)
}

/// Extract the answer from model output.
/// Tries: last line, text after "answer:", text after "Answer:", etc.
pub fn extract_answer(text: &str) -> &str {
    let trimmed = text.trim();

    // Look for explicit answer markers
    let lower = trimmed.to_lowercase();
    for marker in &["the answer is ", "answer: ", "answer is: "] {
        if let Some(pos) = lower.rfind(marker) {
            let after = &trimmed[pos + marker.len()..];
            // Take until end of line or period
            let end = after.find('\n').unwrap_or(after.len());
            return after[..end].trim().trim_end_matches('.');
        }
    }

    // For multiple choice, look for single letter answer
    if trimmed.len() <= 3 {
        return trimmed;
    }

    // Fall back to last non-empty line
    trimmed
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or(trimmed)
        .trim()
}

/// Verify with exact match (binary: 1.0 or 0.0).
pub fn verify_exact(model_output: &str, gold_answers: &[&str]) -> VerifyResult {
    let prediction = extract_answer(model_output);

    if exact_match_any(prediction, gold_answers) {
        VerifyResult::correct()
    } else {
        VerifyResult::wrong(format!(
            "predicted '{}', expected one of {:?}",
            truncate(prediction, 50),
            gold_answers
                .iter()
                .map(|g| truncate(g, 30))
                .collect::<Vec<_>>()
        ))
    }
}

/// Verify with F1 score (continuous: 0.0 to 1.0).
pub fn verify_f1(model_output: &str, gold_answers: &[&str]) -> VerifyResult {
    let prediction = extract_answer(model_output);
    let f1 = best_f1(prediction, gold_answers);

    if f1 >= 1.0 - 1e-9 {
        VerifyResult::correct()
    } else if f1 > 0.0 {
        VerifyResult::partial(f1, format!("F1={f1:.3}"))
    } else {
        VerifyResult::wrong("no token overlap with gold")
    }
}

/// Verify multiple choice (prediction must match one of A/B/C/D).
pub fn verify_multiple_choice(model_output: &str, correct_label: &str) -> VerifyResult {
    let prediction = extract_answer(model_output).trim().to_uppercase();
    let gold = correct_label.trim().to_uppercase();

    // Direct match
    if prediction == gold {
        return VerifyResult::correct();
    }

    // Check if the first word/token is the answer letter
    let first_word: String = prediction
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .collect();
    if first_word == gold {
        return VerifyResult::correct();
    }

    // Check for "(A)" style
    let parens = format!("({})", gold);
    if prediction.starts_with(&parens)
        || prediction.starts_with(&format!("{gold}."))
        || prediction.starts_with(&format!("{gold}:"))
        || prediction.starts_with(&format!("{gold})"))
    {
        return VerifyResult::correct();
    }

    // Check for "(A)" anywhere as standalone
    if prediction.contains(&parens) {
        return VerifyResult::correct();
    }

    VerifyResult::wrong(format!("predicted '{prediction}', expected '{gold}'"))
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Normalization ==========

    #[test]
    fn normalize_basic() {
        assert_eq!(normalize("Hello World!"), "hello world");
        assert_eq!(normalize("  The   Answer  "), "answer");
        assert_eq!(normalize("U.S.A."), "u s"); // "a" removed as article — matches SQuAD behavior
    }

    #[test]
    fn normalize_removes_articles() {
        assert_eq!(normalize("the cat"), "cat");
        assert_eq!(normalize("a dog"), "dog");
        assert_eq!(normalize("an apple"), "apple");
    }

    // ========== Exact Match ==========

    #[test]
    fn exact_match_identical() {
        assert!(exact_match("Paris", "Paris"));
    }

    #[test]
    fn exact_match_case_insensitive() {
        assert!(exact_match("paris", "Paris"));
        assert!(exact_match("PARIS", "paris"));
    }

    #[test]
    fn exact_match_with_articles() {
        assert!(exact_match("the Eiffel Tower", "Eiffel Tower"));
    }

    #[test]
    fn exact_match_with_punctuation() {
        assert!(exact_match("Dr. Watson", "Dr Watson"));
    }

    #[test]
    fn exact_match_different() {
        assert!(!exact_match("Paris", "London"));
    }

    #[test]
    fn exact_match_multiple_golds() {
        assert!(exact_match_any(
            "NYC",
            &["New York City", "NYC", "New York"]
        ));
        assert!(!exact_match_any(
            "LA",
            &["New York City", "NYC", "New York"]
        ));
    }

    // ========== Token F1 ==========

    #[test]
    fn f1_perfect() {
        assert!((token_f1("the capital of France", "the capital of France") - 1.0).abs() < 0.01);
    }

    #[test]
    fn f1_partial_overlap() {
        let f1 = token_f1("the capital is Paris", "Paris is the capital of France");
        assert!(f1 > 0.0 && f1 < 1.0, "F1 should be partial: {f1}");
    }

    #[test]
    fn f1_no_overlap() {
        assert_eq!(token_f1("hello world", "goodbye moon"), 0.0);
    }

    // ========== Answer Extraction ==========

    #[test]
    fn extract_with_marker() {
        assert_eq!(extract_answer("I think the answer is Paris."), "Paris");
    }

    #[test]
    fn extract_answer_colon() {
        assert_eq!(extract_answer("Let me think. Answer: 42"), "42");
    }

    #[test]
    fn extract_last_line() {
        assert_eq!(
            extract_answer("Step 1: think\nStep 2: compute\nParis"),
            "Paris"
        );
    }

    #[test]
    fn extract_single_letter() {
        assert_eq!(extract_answer("B"), "B");
    }

    // ========== Full Verification ==========

    #[test]
    fn verify_exact_correct() {
        let result = verify_exact("The answer is Paris", &["Paris"]);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_exact_wrong() {
        let result = verify_exact("The answer is London", &["Paris"]);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_exact_multiple_gold() {
        let result = verify_exact("The answer is NYC", &["New York City", "NYC", "New York"]);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_f1_partial() {
        let result = verify_f1("the big red dog", &["the red dog"]);
        assert!(result.score > 0.5 && result.score < 1.0);
    }

    // ========== Multiple Choice ==========

    #[test]
    fn mc_direct_match() {
        assert_eq!(verify_multiple_choice("B", "B").score, 1.0);
        assert_eq!(verify_multiple_choice("A", "B").score, 0.0);
    }

    #[test]
    fn mc_with_explanation() {
        assert_eq!(
            verify_multiple_choice("The answer is B because...", "B").score,
            1.0
        );
    }

    #[test]
    fn mc_with_parentheses() {
        assert_eq!(
            verify_multiple_choice("I choose (C) as my answer", "C").score,
            1.0
        );
    }

    #[test]
    fn mc_case_insensitive() {
        assert_eq!(verify_multiple_choice("b", "B").score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_answers() {
        for (pred, gold, expected_pass) in &[
            ("Paris", "Paris", true),
            ("London", "Paris", false),
            ("42", "42", true),
            ("43", "42", false),
            ("True", "True", true),
            ("False", "True", false),
        ] {
            let result = verify_exact(pred, &[gold]);
            assert_eq!(
                result.score == 1.0,
                *expected_pass,
                "pred={pred}, gold={gold}"
            );
        }
    }

    // ========== Real QA Examples ==========

    #[test]
    fn squad_style_qa() {
        // SQuAD: "In what year was the University of Notre Dame founded?"
        // Gold: "1842"
        // F1 is the right metric for extractive QA (finds token overlap)
        let result = verify_f1(
            "Based on the passage, the University of Notre Dame was founded in 1842.",
            &["1842"],
        );
        assert!(result.score > 0.0, "should find '1842' in token overlap");

        // Exact match works when the model gives a clean answer
        let result2 = verify_exact("The answer is 1842", &["1842"]);
        assert_eq!(result2.score, 1.0);
    }

    #[test]
    fn triviaqa_style() {
        // TriviaQA: "What is the capital of Australia?"
        let result = verify_exact("The answer is Canberra", &["Canberra"]);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn fever_style() {
        // FEVER: claim verification
        let result = verify_exact("SUPPORTS", &["SUPPORTS"]);
        assert_eq!(result.score, 1.0);

        let result = verify_exact("REFUTES", &["SUPPORTS"]);
        assert_eq!(result.score, 0.0);
    }
}
