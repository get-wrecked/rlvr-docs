//! Instruction Following Verifier
//!
//! Verifies that model output follows specific, checkable instructions.
//! Inspired by IFEval (Instruction Following Evaluation).
//!
//! Each instruction is a constraint that can be programmatically verified:
//! - Word count constraints
//! - Must include/exclude specific words
//! - Format constraints (all caps, bullet points, numbered list)
//! - Language constraints (start with, end with)
//! - Structural constraints (paragraph count, sentence count)

use super::VerifyResult;

/// A single checkable instruction/constraint.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Constraint {
    /// Output must contain between min and max words
    #[serde(rename = "word_count")]
    WordCount {
        min: Option<usize>,
        max: Option<usize>,
    },

    /// Output must contain this exact string (case-insensitive)
    #[serde(rename = "must_include")]
    MustInclude { text: String },

    /// Output must NOT contain this string (case-insensitive)
    #[serde(rename = "must_exclude")]
    MustExclude { text: String },

    /// Output must start with this prefix
    #[serde(rename = "starts_with")]
    StartsWith { prefix: String },

    /// Output must end with this suffix
    #[serde(rename = "ends_with")]
    EndsWith { suffix: String },

    /// Output must contain at least N sentences
    #[serde(rename = "min_sentences")]
    MinSentences { count: usize },

    /// Output must contain at most N sentences
    #[serde(rename = "max_sentences")]
    MaxSentences { count: usize },

    /// Output must contain exactly N paragraphs (separated by blank lines)
    #[serde(rename = "paragraph_count")]
    ParagraphCount { count: usize },

    /// Output must be entirely in UPPERCASE
    #[serde(rename = "all_uppercase")]
    AllUppercase,

    /// Output must be entirely in lowercase
    #[serde(rename = "all_lowercase")]
    AllLowercase,

    /// Output must contain a numbered list with at least N items
    #[serde(rename = "numbered_list")]
    NumberedList { min_items: usize },

    /// Output must contain a bullet list with at least N items
    #[serde(rename = "bullet_list")]
    BulletList { min_items: usize },

    /// Output must contain the word/phrase exactly N times
    #[serde(rename = "exact_count")]
    ExactCount { text: String, count: usize },

    /// Output must have at least N lines
    #[serde(rename = "min_lines")]
    MinLines { count: usize },

    /// Output must NOT exceed N characters
    #[serde(rename = "max_chars")]
    MaxChars { count: usize },
}

/// Check a single constraint against the model output.
/// Returns (passed: bool, explanation: String)
pub fn check_constraint(output: &str, constraint: &Constraint) -> (bool, String) {
    match constraint {
        Constraint::WordCount { min, max } => {
            let wc = word_count(output);
            let min_ok = min.is_none_or(|m| wc >= m);
            let max_ok = max.is_none_or(|m| wc <= m);
            if min_ok && max_ok {
                (true, format!("word count {wc} in range"))
            } else {
                (
                    false,
                    format!("word count {wc}, expected {:?}-{:?}", min, max),
                )
            }
        }
        Constraint::MustInclude { text } => {
            let lower_output = output.to_lowercase();
            let lower_text = text.to_lowercase();
            if lower_output.contains(&lower_text) {
                (true, format!("contains '{text}'"))
            } else {
                (false, format!("missing required text '{text}'"))
            }
        }
        Constraint::MustExclude { text } => {
            let lower_output = output.to_lowercase();
            let lower_text = text.to_lowercase();
            if !lower_output.contains(&lower_text) {
                (true, format!("correctly excludes '{text}'"))
            } else {
                (false, format!("contains forbidden text '{text}'"))
            }
        }
        Constraint::StartsWith { prefix } => {
            let trimmed = output.trim();
            if trimmed.starts_with(prefix.as_str()) {
                (true, format!("starts with '{prefix}'"))
            } else {
                (false, format!("does not start with '{prefix}'"))
            }
        }
        Constraint::EndsWith { suffix } => {
            let trimmed = output.trim();
            if trimmed.ends_with(suffix.as_str()) {
                (true, format!("ends with '{suffix}'"))
            } else {
                (false, format!("does not end with '{suffix}'"))
            }
        }
        Constraint::MinSentences { count } => {
            let sc = sentence_count(output);
            if sc >= *count {
                (true, format!("{sc} sentences >= {count}"))
            } else {
                (false, format!("{sc} sentences < required {count}"))
            }
        }
        Constraint::MaxSentences { count } => {
            let sc = sentence_count(output);
            if sc <= *count {
                (true, format!("{sc} sentences <= {count}"))
            } else {
                (false, format!("{sc} sentences > maximum {count}"))
            }
        }
        Constraint::ParagraphCount { count } => {
            let pc = paragraph_count(output);
            if pc == *count {
                (true, format!("{pc} paragraphs == {count}"))
            } else {
                (false, format!("{pc} paragraphs, expected {count}"))
            }
        }
        Constraint::AllUppercase => {
            let alpha: String = output.chars().filter(|c| c.is_alphabetic()).collect();
            if !alpha.is_empty() && alpha == alpha.to_uppercase() {
                (true, "all uppercase".to_string())
            } else {
                (false, "not all uppercase".to_string())
            }
        }
        Constraint::AllLowercase => {
            let alpha: String = output.chars().filter(|c| c.is_alphabetic()).collect();
            if !alpha.is_empty() && alpha == alpha.to_lowercase() {
                (true, "all lowercase".to_string())
            } else {
                (false, "not all lowercase".to_string())
            }
        }
        Constraint::NumberedList { min_items } => {
            let items = count_numbered_list_items(output);
            if items >= *min_items {
                (true, format!("{items} numbered items >= {min_items}"))
            } else {
                (
                    false,
                    format!("{items} numbered items < required {min_items}"),
                )
            }
        }
        Constraint::BulletList { min_items } => {
            let items = count_bullet_list_items(output);
            if items >= *min_items {
                (true, format!("{items} bullet items >= {min_items}"))
            } else {
                (
                    false,
                    format!("{items} bullet items < required {min_items}"),
                )
            }
        }
        Constraint::ExactCount { text, count } => {
            let lower = output.to_lowercase();
            let search = text.to_lowercase();
            let actual = lower.matches(&search).count();
            if actual == *count {
                (true, format!("'{text}' appears {actual} times"))
            } else {
                (
                    false,
                    format!("'{text}' appears {actual} times, expected {count}"),
                )
            }
        }
        Constraint::MinLines { count } => {
            let lc = output.lines().count();
            if lc >= *count {
                (true, format!("{lc} lines >= {count}"))
            } else {
                (false, format!("{lc} lines < required {count}"))
            }
        }
        Constraint::MaxChars { count } => {
            let cc = output.len();
            if cc <= *count {
                (true, format!("{cc} chars <= {count}"))
            } else {
                (false, format!("{cc} chars > maximum {count}"))
            }
        }
    }
}

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

fn sentence_count(s: &str) -> usize {
    // Split on sentence-ending punctuation followed by space + capital letter or end of string
    // This is more robust than just counting periods
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();

    // Common abbreviations that end with a period but aren't sentence boundaries
    let abbrevs = [
        "mr.", "mrs.", "ms.", "dr.", "prof.", "sr.", "jr.", "vs.", "etc.", "inc.", "ltd.", "st.",
        "ave.", "i.e.", "e.g.", "u.s.", "u.k.", "a.m.", "p.m.",
    ];

    for (i, &c) in chars.iter().enumerate() {
        if (c == '.' || c == '!' || c == '?')
            && (i + 1 >= chars.len() || chars[i + 1].is_whitespace())
        {
            // Skip ellipsis
            if i > 0 && chars[i - 1] == '.' {
                continue;
            }

            // Skip common abbreviations
            let prefix: String = chars[..=i].iter().collect();
            let prefix_lower = prefix.to_lowercase();
            let is_abbrev = abbrevs.iter().any(|a| prefix_lower.ends_with(a));
            if is_abbrev {
                continue;
            }

            count += 1;
        }
    }

    // At minimum, non-empty text has at least one "sentence" even without terminal punctuation
    if count == 0 && !s.trim().is_empty() {
        count = 1;
    }

    count
}

fn paragraph_count(s: &str) -> usize {
    s.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .count()
}

fn count_numbered_list_items(s: &str) -> usize {
    s.lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Match "1.", "2.", "10.", "1)", "2)", etc.
            trimmed.len() >= 2
                && trimmed.chars().next().is_some_and(|c| c.is_ascii_digit())
                && (trimmed.contains(". ") || trimmed.contains(") "))
        })
        .count()
}

fn count_bullet_list_items(s: &str) -> usize {
    s.lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("• ")
        })
        .count()
}

/// Verify model output against a list of constraints.
/// Score = fraction of constraints satisfied.
pub fn verify(model_output: &str, constraints_json: &str) -> VerifyResult {
    let constraints: Vec<Constraint> = match serde_json::from_str(constraints_json) {
        Ok(c) => c,
        Err(e) => return VerifyResult::wrong(format!("invalid constraints: {e}")),
    };

    if constraints.is_empty() {
        return VerifyResult::wrong("no constraints provided");
    }

    let mut passed = 0;
    let mut failures = Vec::new();

    for constraint in &constraints {
        let (ok, reason) = check_constraint(model_output, constraint);
        if ok {
            passed += 1;
        } else {
            failures.push(reason);
        }
    }

    let total = constraints.len();
    if passed == total {
        VerifyResult::correct()
    } else {
        VerifyResult::partial(
            passed as f64 / total as f64,
            format!(
                "{passed}/{total} constraints met. Failed: {}",
                failures.join("; ")
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(json: &str) -> String {
        json.to_string()
    }

    // ========== Word Count ==========

    #[test]
    fn word_count_in_range() {
        let constraints = c(r#"[{"type": "word_count", "min": 5, "max": 10}]"#);
        let result = verify("This is a seven word sentence here.", &constraints);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn word_count_too_few() {
        let constraints = c(r#"[{"type": "word_count", "min": 10}]"#);
        let result = verify("Too few words.", &constraints);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn word_count_too_many() {
        let constraints = c(r#"[{"type": "word_count", "max": 3}]"#);
        let result = verify("This has more than three words in it.", &constraints);
        assert_eq!(result.score, 0.0);
    }

    // ========== Include/Exclude ==========

    #[test]
    fn must_include_present() {
        let constraints = c(r#"[{"type": "must_include", "text": "hello"}]"#);
        assert_eq!(verify("Say hello to the world!", &constraints).score, 1.0);
    }

    #[test]
    fn must_include_case_insensitive() {
        let constraints = c(r#"[{"type": "must_include", "text": "hello"}]"#);
        assert_eq!(verify("HELLO WORLD!", &constraints).score, 1.0);
    }

    #[test]
    fn must_include_missing() {
        let constraints = c(r#"[{"type": "must_include", "text": "hello"}]"#);
        assert_eq!(verify("Goodbye world!", &constraints).score, 0.0);
    }

    #[test]
    fn must_exclude_absent() {
        let constraints = c(r#"[{"type": "must_exclude", "text": "bad"}]"#);
        assert_eq!(verify("This is a good response.", &constraints).score, 1.0);
    }

    #[test]
    fn must_exclude_present() {
        let constraints = c(r#"[{"type": "must_exclude", "text": "bad"}]"#);
        assert_eq!(verify("This is a bad response.", &constraints).score, 0.0);
    }

    // ========== Format Constraints ==========

    #[test]
    fn all_uppercase() {
        let constraints = c(r#"[{"type": "all_uppercase"}]"#);
        assert_eq!(verify("THIS IS UPPERCASE!", &constraints).score, 1.0);
        assert_eq!(verify("This is Mixed.", &constraints).score, 0.0);
    }

    #[test]
    fn all_lowercase() {
        let constraints = c(r#"[{"type": "all_lowercase"}]"#);
        assert_eq!(verify("this is lowercase!", &constraints).score, 1.0);
        assert_eq!(verify("This is Mixed.", &constraints).score, 0.0);
    }

    #[test]
    fn starts_with_constraint() {
        let constraints = c(r#"[{"type": "starts_with", "prefix": "Dear"}]"#);
        assert_eq!(verify("Dear sir, ...", &constraints).score, 1.0);
        assert_eq!(verify("Hello sir, ...", &constraints).score, 0.0);
    }

    #[test]
    fn ends_with_constraint() {
        let constraints = c(r#"[{"type": "ends_with", "suffix": "Sincerely."}]"#);
        assert_eq!(verify("Best regards. Sincerely.", &constraints).score, 1.0);
    }

    // ========== Lists ==========

    #[test]
    fn numbered_list() {
        let constraints = c(r#"[{"type": "numbered_list", "min_items": 3}]"#);
        let output = "Here are the items:\n1. First item\n2. Second item\n3. Third item";
        assert_eq!(verify(output, &constraints).score, 1.0);
    }

    #[test]
    fn numbered_list_too_few() {
        let constraints = c(r#"[{"type": "numbered_list", "min_items": 5}]"#);
        let output = "1. First\n2. Second";
        assert_eq!(verify(output, &constraints).score, 0.0);
    }

    #[test]
    fn bullet_list() {
        let constraints = c(r#"[{"type": "bullet_list", "min_items": 2}]"#);
        let output = "Points:\n- First point\n- Second point\n- Third point";
        assert_eq!(verify(output, &constraints).score, 1.0);
    }

    // ========== Multi-Constraint ==========

    #[test]
    fn multiple_constraints_all_pass() {
        let constraints = c(r#"[
            {"type": "word_count", "min": 5, "max": 20},
            {"type": "must_include", "text": "answer"},
            {"type": "starts_with", "prefix": "The"}
        ]"#);
        let result = verify("The answer to your question is yes.", &constraints);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn multiple_constraints_partial() {
        let constraints = c(r#"[
            {"type": "must_include", "text": "hello"},
            {"type": "must_include", "text": "world"},
            {"type": "must_include", "text": "foo"}
        ]"#);
        let result = verify("hello world", &constraints);
        // 2/3 pass
        assert!((result.score - 2.0 / 3.0).abs() < 0.01);
    }

    // ========== Exact Count ==========

    #[test]
    fn exact_count_constraint() {
        let constraints = c(r#"[{"type": "exact_count", "text": "the", "count": 3}]"#);
        assert_eq!(
            verify("the cat and the dog ate the food", &constraints).score,
            1.0
        );
        assert_eq!(verify("the cat ate food", &constraints).score, 0.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_same_text_different_constraints() {
        let text = "Hello World";

        let c1 = c(r#"[{"type": "must_include", "text": "Hello"}]"#);
        let c2 = c(r#"[{"type": "must_include", "text": "Goodbye"}]"#);
        let c3 = c(r#"[{"type": "all_uppercase"}]"#);
        let c4 = c(r#"[{"type": "all_lowercase"}]"#);

        assert_eq!(verify(text, &c1).score, 1.0);
        assert_eq!(verify(text, &c2).score, 0.0);
        assert_eq!(verify(text, &c3).score, 0.0);
        assert_eq!(verify(text, &c4).score, 0.0);
    }

    // ========== Paragraph and Sentence Counts ==========

    #[test]
    fn paragraph_count_check() {
        let constraints = c(r#"[{"type": "paragraph_count", "count": 3}]"#);
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        assert_eq!(verify(text, &constraints).score, 1.0);
    }

    #[test]
    fn sentence_count_check() {
        let constraints = c(r#"[{"type": "min_sentences", "count": 3}]"#);
        let text = "First sentence. Second sentence. Third sentence.";
        assert_eq!(verify(text, &constraints).score, 1.0);

        let too_few = "Just one sentence.";
        assert_eq!(verify(too_few, &constraints).score, 0.0);
    }

    // ========== ADVERSARIAL TESTS ==========

    #[test]
    fn adversarial_abbreviation_sentence_count() {
        // "Dr. Smith went to Washington." should be ONE sentence, not two
        let constraints = c(r#"[{"type": "max_sentences", "count": 1}]"#);
        let text = "Dr. Smith went to Washington.";
        assert_eq!(
            verify(text, &constraints).score,
            1.0,
            "Dr. should not count as sentence end"
        );

        let text2 = "Mr. Jones and Mrs. Smith met at St. Paul's.";
        assert_eq!(
            verify(text2, &constraints).score,
            1.0,
            "Abbreviations should not count"
        );
    }

    #[test]
    fn adversarial_numbered_list_false_positive() {
        // "In 2024. The economy..." should NOT count as a numbered list item
        let constraints = c(r#"[{"type": "numbered_list", "min_items": 2}]"#);
        let text = "In 2024. The economy grew by 3%.";
        assert_eq!(
            verify(text, &constraints).score,
            0.0,
            "Year references should not count as list items"
        );
    }

    #[test]
    fn adversarial_word_count_with_punctuation() {
        // "well-known" — is this 1 word or 2?
        let constraints = c(r#"[{"type": "word_count", "min": 2, "max": 2}]"#);
        let text = "well-known";
        // split_whitespace treats "well-known" as 1 word — should fail min=2
        assert_eq!(verify(text, &constraints).score, 0.0);
    }

    #[test]
    fn adversarial_must_include_substring() {
        // "must include 'cat'" — "concatenate" contains "cat" but shouldn't count...
        // Actually in our current design, substring matching IS the intended behavior.
        // This is a known limitation — document it but don't "fix" it.
        let constraints = c(r#"[{"type": "must_include", "text": "cat"}]"#);
        assert_eq!(
            verify("I have a concatenation problem.", &constraints).score,
            1.0,
            "Substring matching is intentional — matching IFEval behavior"
        );
    }

    #[test]
    fn adversarial_empty_output_all_constraints_fail() {
        let constraints = c(r#"[
            {"type": "word_count", "min": 5},
            {"type": "must_include", "text": "hello"},
            {"type": "all_uppercase"}
        ]"#);
        let result = verify("", &constraints);
        assert_eq!(
            result.score, 0.0,
            "Empty output should fail all constraints"
        );
    }

    #[test]
    fn adversarial_unicode_in_constraints() {
        let constraints = c(r#"[{"type": "must_include", "text": "café"}]"#);
        assert_eq!(verify("I went to the café.", &constraints).score, 1.0);
        assert_eq!(
            verify("I went to the cafe.", &constraints).score,
            0.0,
            "café ≠ cafe (accent matters)"
        );
    }
}
