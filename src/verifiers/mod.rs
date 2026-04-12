pub mod math_numerical;
pub mod math_equivalence;
pub mod sudoku;
pub mod regex_synthesis;
pub mod json_schema;
pub mod instruction_following;
pub mod code_execution;
pub mod exact_match;
pub mod chemical_equation;
pub mod unit_conversion;
pub mod date_time;
pub mod sql_execution;
pub mod graph_properties;

/// Result of a verification: a score from 0.0 (completely wrong) to 1.0 (perfect).
#[derive(Debug, Clone, PartialEq)]
pub struct VerifyResult {
    pub score: f64,
    pub reason: String,
}

impl VerifyResult {
    pub fn correct() -> Self {
        Self { score: 1.0, reason: "correct".to_string() }
    }

    pub fn wrong(reason: impl Into<String>) -> Self {
        Self { score: 0.0, reason: reason.into() }
    }

    pub fn partial(score: f64, reason: impl Into<String>) -> Self {
        Self { score: score.clamp(0.0, 1.0), reason: reason.into() }
    }
}
