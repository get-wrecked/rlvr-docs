//! Runtime verifier routing for the HTTP API.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::verifiers::{
    self,
    code_execution::{ExecConfig, Language, TestCase},
    regex_synthesis::RegexTask,
    VerifyResult,
};

pub const SUPPORTED_VERIFIERS: &[&str] = &[
    "chemical_equation",
    "chemistry_check",
    "code_execution",
    "date_time",
    "exact_match",
    "graph_check",
    "graph_properties",
    "instruction_following",
    "json_schema",
    "math_equivalence",
    "math_numerical",
    "regex_synthesis",
    "schema_validation",
    "sql_execution",
    "sudoku",
    "unit_conversion",
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyRequest {
    pub domain: String,
    #[serde(default)]
    pub verifier: Option<String>,
    #[serde(default)]
    pub task: Value,
    pub response: String,
    #[serde(default)]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    pub domain: String,
    pub score: f64,
    pub reason: String,
}

pub fn verify_one(request: VerifyRequest) -> VerifyResponse {
    let result = dispatch(&request);
    VerifyResponse {
        task_id: request.task_id,
        domain: request.domain,
        score: result.score,
        reason: result.reason,
    }
}

fn dispatch(request: &VerifyRequest) -> VerifyResult {
    let verifier = request.verifier.as_deref().unwrap_or(&request.domain);
    match normalize_name(verifier).as_str() {
        "math_numerical" | "gsm8k" => verify_math_numerical(request),
        "math_equivalence" => verify_math_equivalence(request),
        "exact_match" | "exact" => verify_exact_match(request),
        "sudoku" => verify_sudoku(request),
        "regex_synthesis" | "regex" => verify_regex(request),
        "json_schema" | "schema_validation" => verify_json_schema(request),
        "instruction_following" => verify_instruction_following(request),
        "code_execution" => verify_code_execution(request),
        "chemical_equation" | "chemistry_check" => verify_chemical_equation(request),
        "unit_conversion" => verify_unit_conversion(request),
        "date_time" => verify_date_time(request),
        "sql_execution" => verify_sql_execution(request),
        "graph_properties" | "graph_check" => verify_graph_properties(request),
        unknown => VerifyResult::wrong(format!("unsupported verifier: {unknown}")),
    }
}

fn verify_math_numerical(request: &VerifyRequest) -> VerifyResult {
    match string_field(&request.task, &["gold", "answer", "gold_answer"]) {
        Some(gold) => verifiers::math_numerical::verify(&request.response, &gold),
        None => missing_field("gold"),
    }
}

fn verify_math_equivalence(request: &VerifyRequest) -> VerifyResult {
    match string_field(&request.task, &["gold", "answer", "gold_answer"]) {
        Some(gold) => verifiers::math_equivalence::verify(&request.response, &gold),
        None => missing_field("gold"),
    }
}

fn verify_exact_match(request: &VerifyRequest) -> VerifyResult {
    let golds = match gold_answers(&request.task) {
        Some(golds) if !golds.is_empty() => golds,
        _ => return missing_field("gold"),
    };
    let gold_refs = golds.iter().map(String::as_str).collect::<Vec<_>>();

    let mode = string_field(&request.task, &["mode", "type"]).unwrap_or_default();
    let mode = normalize_name(&mode);

    if mode == "number" || golds.iter().any(|gold| looks_like_number(gold)) {
        golds
            .iter()
            .map(|gold| verifiers::math_numerical::verify(&request.response, gold))
            .max_by(|left, right| {
                left.score
                    .partial_cmp(&right.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|| missing_field("gold"))
    } else if mode == "f1" || bool_field(&request.task, "partial_credit").unwrap_or(false) {
        verifiers::exact_match::verify_f1(&request.response, &gold_refs)
    } else if mode == "mcq" || mode == "multiple_choice" {
        verifiers::exact_match::verify_multiple_choice(&request.response, gold_refs[0])
    } else {
        verifiers::exact_match::verify_exact(&request.response, &gold_refs)
    }
}

fn verify_sudoku(request: &VerifyRequest) -> VerifyResult {
    match string_field(&request.task, &["puzzle", "puzzle_str"]) {
        Some(puzzle) => verifiers::sudoku::verify(&puzzle, &request.response),
        None => missing_field("puzzle"),
    }
}

fn verify_regex(request: &VerifyRequest) -> VerifyResult {
    let task: RegexTask = match serde_json::from_value(request.task.clone()) {
        Ok(task) => task,
        Err(e) => return VerifyResult::wrong(format!("invalid regex task: {e}")),
    };
    verifiers::regex_synthesis::verify(&task, &request.response)
}

fn verify_json_schema(request: &VerifyRequest) -> VerifyResult {
    match json_field_string(&request.task, &["schema_json", "schema"]) {
        Some(schema) => verifiers::json_schema::verify(&request.response, &schema),
        None => missing_field("schema"),
    }
}

fn verify_instruction_following(request: &VerifyRequest) -> VerifyResult {
    match json_field_string(&request.task, &["constraints_json", "constraints"]) {
        Some(constraints) => {
            verifiers::instruction_following::verify(&request.response, &constraints)
        }
        None => missing_field("constraints"),
    }
}

fn verify_code_execution(request: &VerifyRequest) -> VerifyResult {
    let test_cases = match code_test_cases(&request.task) {
        Some(test_cases) if !test_cases.is_empty() => test_cases,
        _ => return missing_field("test_cases"),
    };

    let language =
        match parse_language(string_field(&request.task, &["language", "lang"]).as_deref()) {
            Ok(language) => language,
            Err(reason) => return VerifyResult::wrong(reason),
        };

    let timeout = number_field(&request.task, &["timeout_ms"])
        .map(Duration::from_millis)
        .or_else(|| number_field(&request.task, &["timeout_secs"]).map(Duration::from_secs))
        .unwrap_or_else(|| ExecConfig::default().timeout);

    let config = ExecConfig {
        timeout,
        max_output_bytes: number_field(&request.task, &["max_output_bytes"])
            .map(|value| value as usize)
            .unwrap_or_else(|| ExecConfig::default().max_output_bytes),
        language,
    };

    let function_name = string_field(&request.task, &["function_name", "function", "entrypoint"]);
    verifiers::code_execution::verify(
        &request.response,
        &test_cases,
        function_name.as_deref(),
        &config,
    )
}

fn verify_chemical_equation(request: &VerifyRequest) -> VerifyResult {
    if let Some(unbalanced) = string_field(&request.task, &["unbalanced", "equation"]) {
        verifiers::chemical_equation::verify_balancing(&unbalanced, &request.response)
    } else {
        verifiers::chemical_equation::verify(&request.response)
    }
}

fn verify_unit_conversion(request: &VerifyRequest) -> VerifyResult {
    let value = match float_field(&request.task, &["value", "amount"]) {
        Some(value) => value,
        None => return missing_field("value"),
    };
    let from = match string_field(&request.task, &["from", "from_unit"]) {
        Some(unit) => unit,
        None => return missing_field("from"),
    };
    let to = match string_field(&request.task, &["to", "to_unit"]) {
        Some(unit) => unit,
        None => return missing_field("to"),
    };
    let proposed = verifiers::math_numerical::extract_answer(&request.response)
        .or_else(|| request.response.trim().parse::<f64>().ok());
    let Some(proposed) = proposed else {
        return VerifyResult::wrong("could not parse response as a number");
    };
    let rel_tol = float_field(&request.task, &["rel_tol", "tolerance"]).unwrap_or(1e-4);

    verifiers::unit_conversion::verify(value, &from, &to, proposed, rel_tol)
}

fn verify_date_time(request: &VerifyRequest) -> VerifyResult {
    let task_type = match string_field(&request.task, &["task_type", "operation", "type"]) {
        Some(task_type) => task_type,
        None => return missing_field("task_type"),
    };
    let params = request.task.get("params").unwrap_or(&request.task);
    verifiers::date_time::verify(&task_type, params, &request.response)
}

fn verify_sql_execution(request: &VerifyRequest) -> VerifyResult {
    let db_setup = match string_field(&request.task, &["db_setup", "setup", "schema"]) {
        Some(db_setup) => db_setup,
        None => return missing_field("db_setup"),
    };
    let gold_query = match string_field(&request.task, &["gold_query", "query", "answer"]) {
        Some(gold_query) => gold_query,
        None => return missing_field("gold_query"),
    };

    verifiers::sql_execution::verify(&db_setup, &gold_query, &request.response)
}

fn verify_graph_properties(request: &VerifyRequest) -> VerifyResult {
    let graph_json = match json_field_string(&request.task, &["graph_json", "graph"]) {
        Some(graph_json) => graph_json,
        None => return missing_field("graph"),
    };
    let graph_task = match string_field(&request.task, &["graph_task", "property", "task", "type"])
    {
        Some(task) => task,
        None => return missing_field("graph_task"),
    };
    let answer_json = graph_answer_json(request, &graph_task);

    verifiers::graph_properties::verify(&graph_json, &graph_task, &answer_json)
}

fn graph_answer_json(request: &VerifyRequest, graph_task: &str) -> String {
    if let Some(answer) = json_field_string(&request.task, &["answer_json", "answer"]) {
        return answer;
    }

    if normalize_name(graph_task) == "shortest_path" {
        if let (Some(source), Some(target), Some(length)) = (
            string_field(&request.task, &["source"]),
            string_field(&request.task, &["target"]),
            verifiers::math_numerical::extract_answer(&request.response),
        ) {
            return json!({
                "source": source,
                "target": target,
                "length": length
            })
            .to_string();
        }
    }

    request.response.clone()
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('-', "_")
}

fn missing_field(field: &str) -> VerifyResult {
    VerifyResult::wrong(format!("missing task.{field}"))
}

fn string_field(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(value_to_string))
}

fn json_field_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key))
        .and_then(value_to_string)
}

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) => Some(value.clone()),
        other => Some(other.to_string()),
    }
}

fn bool_field(value: &Value, key: &str) -> Option<bool> {
    value.get(key).and_then(Value::as_bool)
}

fn looks_like_number(value: &str) -> bool {
    let trimmed = value
        .trim()
        .trim_start_matches('$')
        .trim_end_matches('%')
        .replace(',', "");
    if trimmed.parse::<f64>().is_ok() {
        return true;
    }

    if let Some((numerator, denominator)) = trimmed.split_once('/') {
        return numerator.trim().parse::<f64>().is_ok()
            && denominator.trim().parse::<f64>().is_ok();
    }

    false
}

fn number_field(value: &Value, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| value.get(*key)?.as_u64())
}

fn float_field(value: &Value, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|key| value.get(*key)?.as_f64())
}

fn gold_answers(task: &Value) -> Option<Vec<String>> {
    ["gold_answers", "golds", "gold", "answer", "answers"]
        .iter()
        .find_map(|key| {
            let value = task.get(*key)?;
            if let Some(values) = value.as_array() {
                Some(values.iter().filter_map(value_to_string).collect())
            } else {
                value_to_string(value).map(|value| vec![value])
            }
        })
}

fn code_test_cases(task: &Value) -> Option<Vec<TestCase>> {
    let cases = task.get("test_cases")?.as_array()?;
    let test_cases = cases
        .iter()
        .filter_map(|case| {
            let input = case.get("input").and_then(value_to_string)?;
            let expected_output = case
                .get("expected_output")
                .or_else(|| case.get("expected"))
                .and_then(value_to_string)?;
            Some(TestCase {
                input,
                expected_output,
            })
        })
        .collect();
    Some(test_cases)
}

fn parse_language(language: Option<&str>) -> Result<Language, String> {
    match language.map(normalize_name).as_deref() {
        None | Some("") | Some("python") | Some("python3") => Ok(Language::Python),
        Some("javascript") | Some("js") | Some("node") => Ok(Language::JavaScript),
        Some("rust") => Ok(Language::Rust),
        Some(other) => Err(format!("unsupported execution language: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_math_numerical_requests() {
        let response = verify_one(VerifyRequest {
            domain: "gsm8k".to_string(),
            verifier: Some("math_numerical".to_string()),
            task: json!({"gold": "#### 18"}),
            response: "The answer is 18".to_string(),
            task_id: Some("task-1".to_string()),
        });

        assert_eq!(response.task_id.as_deref(), Some("task-1"));
        assert_eq!(response.score, 1.0);
    }

    #[test]
    fn routes_schema_validation_alias() {
        let response = verify_one(VerifyRequest {
            domain: "structured-output-generation".to_string(),
            verifier: Some("schema_validation".to_string()),
            task: json!({
                "schema": {
                    "type": "object",
                    "required": ["answer"],
                    "properties": {"answer": {"type": "integer"}}
                }
            }),
            response: r#"{"answer": 42}"#.to_string(),
            task_id: None,
        });

        assert_eq!(response.score, 1.0);
    }
}
