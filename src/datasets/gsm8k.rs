//! GSM8K Dataset Loader
//!
//! Loads the Grade School Math 8K dataset from JSONL files.
//! Each problem has a question and an answer with the format:
//! "reasoning steps...\n#### <number>"

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gsm8kProblem {
    pub question: String,
    pub answer: String,
}

/// Load GSM8K problems from a JSONL file.
pub fn load(path: &str) -> Result<Vec<Gsm8kProblem>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut problems = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let problem: Gsm8kProblem = serde_json::from_str(line)?;
        problems.push(problem);
    }
    Ok(problems)
}

/// Extract the numerical answer from a GSM8K answer string.
/// The answer format is: "reasoning...\n#### <number>"
pub fn extract_gold_answer(answer: &str) -> Option<f64> {
    crate::verifiers::math_numerical::extract_answer(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn load_gsm8k_test_set() {
        let path = "raw/datasets/gsm8k_test.jsonl";
        if !Path::new(path).exists() {
            eprintln!("SKIP: GSM8K test set not found at {path}");
            return;
        }

        let problems = load(path).expect("failed to load GSM8K");
        assert_eq!(problems.len(), 1319, "GSM8K test set should have 1319 problems");

        // Verify we can extract gold answers from ALL problems
        let mut extract_failures = Vec::new();
        for (i, problem) in problems.iter().enumerate() {
            if extract_gold_answer(&problem.answer).is_none() {
                extract_failures.push(i);
            }
        }

        assert!(
            extract_failures.is_empty(),
            "Failed to extract gold answer from {} problems: {:?}",
            extract_failures.len(),
            &extract_failures[..extract_failures.len().min(10)]
        );
    }

    #[test]
    fn gsm8k_first_problem_is_janet() {
        let path = "raw/datasets/gsm8k_test.jsonl";
        if !Path::new(path).exists() {
            return;
        }

        let problems = load(path).unwrap();
        assert!(problems[0].question.contains("Janet"));
        assert_eq!(extract_gold_answer(&problems[0].answer), Some(18.0));
    }

    #[test]
    fn gsm8k_verify_correct_model_output() {
        let path = "raw/datasets/gsm8k_test.jsonl";
        if !Path::new(path).exists() {
            return;
        }

        let problems = load(path).unwrap();

        // Simulate: model output matches gold answer for first 10 problems
        for problem in problems.iter().take(10) {
            let gold = extract_gold_answer(&problem.answer).unwrap();
            // Simulate model producing the correct answer
            let model_output = format!("The answer is {gold}\n#### {gold}");
            let result = crate::verifiers::math_numerical::verify(&model_output, &problem.answer);
            assert_eq!(
                result.score, 1.0,
                "Correct answer should score 1.0 for: {}",
                &problem.question[..50.min(problem.question.len())]
            );
        }
    }

    #[test]
    fn gsm8k_verify_wrong_model_output() {
        let path = "raw/datasets/gsm8k_test.jsonl";
        if !Path::new(path).exists() {
            return;
        }

        let problems = load(path).unwrap();

        // Simulate: model output is always wrong (answer + 1)
        for problem in problems.iter().take(10) {
            let gold = extract_gold_answer(&problem.answer).unwrap();
            let wrong = gold + 1.0;
            let model_output = format!("#### {wrong}");
            let result = crate::verifiers::math_numerical::verify(&model_output, &problem.answer);
            assert_eq!(
                result.score, 0.0,
                "Wrong answer ({wrong} vs {gold}) should score 0.0"
            );
        }
    }

    #[test]
    fn gsm8k_answer_distribution() {
        // Sanity check: verify the distribution of gold answers makes sense
        let path = "raw/datasets/gsm8k_test.jsonl";
        if !Path::new(path).exists() {
            return;
        }

        let problems = load(path).unwrap();
        let answers: Vec<f64> = problems.iter()
            .filter_map(|p| extract_gold_answer(&p.answer))
            .collect();

        // All answers should be extractable
        assert_eq!(answers.len(), problems.len());

        // Check basic stats
        let min = answers.iter().copied().fold(f64::INFINITY, f64::min);
        let max = answers.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mean = answers.iter().sum::<f64>() / answers.len() as f64;

        // GSM8K answers range from small negatives to large positives
        assert!(min < 0.0 || min >= 0.0, "min should be a number"); // just not NaN
        assert!(max > 0.0, "max should be positive");
        assert!(mean > 0.0, "mean should be positive (most answers are positive)");
        eprintln!("GSM8K answer stats: min={min}, max={max}, mean={mean:.1}, n={}", answers.len());
    }
}
