//! JSON Schema Verifier
//!
//! Verifies that model output is valid JSON conforming to a specified schema.
//! Covers the `structured-output-generation` domain.
//!
//! Verification levels:
//! 1. Is it valid JSON? (partial credit)
//! 2. Does it conform to the schema? (full credit)

use super::VerifyResult;
use serde_json::Value;

/// A simplified JSON schema for verification.
/// We implement a subset of JSON Schema that covers common RLVR tasks.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(default)]
    pub properties: Option<std::collections::HashMap<String, Schema>>,
    #[serde(default)]
    pub required: Option<Vec<String>>,
    #[serde(default)]
    pub items: Option<Box<Schema>>,
    #[serde(rename = "enum", default)]
    pub enum_values: Option<Vec<Value>>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(rename = "minLength", default)]
    pub min_length: Option<usize>,
    #[serde(rename = "maxLength", default)]
    pub max_length: Option<usize>,
    #[serde(rename = "minItems", default)]
    pub min_items: Option<usize>,
    #[serde(rename = "maxItems", default)]
    pub max_items: Option<usize>,
    #[serde(default)]
    pub pattern: Option<String>,
}

/// Validate a JSON value against a schema. Returns list of violations.
pub fn validate(value: &Value, schema: &Schema) -> Vec<String> {
    let mut errors = Vec::new();
    validate_inner(value, schema, "", &mut errors);
    errors
}

fn validate_inner(value: &Value, schema: &Schema, path: &str, errors: &mut Vec<String>) {
    // Type check
    let type_ok = match schema.schema_type.as_str() {
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => {
            value.is_i64()
                || value.is_u64()
                || (value.is_f64() && value.as_f64().is_some_and(|f| f == f.round()))
        }
        "boolean" => value.is_boolean(),
        "null" => value.is_null(),
        _ => true,
    };

    if !type_ok {
        errors.push(format!(
            "{path}: expected type '{}', got {}",
            schema.schema_type,
            value_type_name(value)
        ));
        return; // Don't check sub-constraints if type is wrong
    }

    // Object properties
    if let (Some(props), Some(obj)) = (&schema.properties, value.as_object()) {
        // Check required fields
        if let Some(required) = &schema.required {
            for req in required {
                if !obj.contains_key(req) {
                    errors.push(format!("{path}: missing required field '{req}'"));
                }
            }
        }

        // Validate each property that exists
        for (key, prop_schema) in props {
            if let Some(prop_value) = obj.get(key) {
                let prop_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                validate_inner(prop_value, prop_schema, &prop_path, errors);
            }
        }
    }

    // Array items
    if let (Some(items_schema), Some(arr)) = (&schema.items, value.as_array()) {
        if let Some(min) = schema.min_items {
            if arr.len() < min {
                errors.push(format!(
                    "{path}: array has {} items, minimum is {min}",
                    arr.len()
                ));
            }
        }
        if let Some(max) = schema.max_items {
            if arr.len() > max {
                errors.push(format!(
                    "{path}: array has {} items, maximum is {max}",
                    arr.len()
                ));
            }
        }
        for (i, item) in arr.iter().enumerate() {
            let item_path = format!("{path}[{i}]");
            validate_inner(item, items_schema, &item_path, errors);
        }
    }

    // String constraints
    if let Some(s) = value.as_str() {
        if let Some(min) = schema.min_length {
            if s.len() < min {
                errors.push(format!("{path}: string length {} < minimum {min}", s.len()));
            }
        }
        if let Some(max) = schema.max_length {
            if s.len() > max {
                errors.push(format!("{path}: string length {} > maximum {max}", s.len()));
            }
        }
        if let Some(pattern) = &schema.pattern {
            if let Ok(re) = regex::Regex::new(pattern) {
                if !re.is_match(s) {
                    errors.push(format!("{path}: string does not match pattern '{pattern}'"));
                }
            }
        }
    }

    // Number constraints
    if let Some(n) = value.as_f64() {
        if let Some(min) = schema.minimum {
            if n < min {
                errors.push(format!("{path}: {n} < minimum {min}"));
            }
        }
        if let Some(max) = schema.maximum {
            if n > max {
                errors.push(format!("{path}: {n} > maximum {max}"));
            }
        }
    }

    // Enum constraint
    if let Some(enum_vals) = &schema.enum_values {
        if !enum_vals.contains(value) {
            errors.push(format!("{path}: value not in enum"));
        }
    }
}

fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::Object(_) => "object",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
    }
}

/// Verify that model output is valid JSON conforming to a schema.
pub fn verify(model_output: &str, schema_json: &str) -> VerifyResult {
    // Parse schema
    let schema: Schema = match serde_json::from_str(schema_json) {
        Ok(s) => s,
        Err(e) => return VerifyResult::wrong(format!("invalid schema: {e}")),
    };

    // Try to extract JSON from the model output
    let json_str = extract_json(model_output);

    // Parse JSON
    let value: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return VerifyResult::partial(0.1, "output is not valid JSON"),
    };

    // Validate against schema
    let errors = validate(&value, &schema);
    if errors.is_empty() {
        VerifyResult::correct()
    } else {
        let total_checks = count_schema_checks(&schema);
        let score = if total_checks > 0 {
            0.2 + 0.8 * (1.0 - errors.len() as f64 / total_checks as f64).max(0.0)
        } else {
            0.2 // Valid JSON but no schema constraints to check
        };
        VerifyResult::partial(
            score,
            format!("{} violations: {}", errors.len(), errors.join("; ")),
        )
    }
}

/// Extract JSON from model output — handles markdown code blocks, extra text around JSON.
fn extract_json(text: &str) -> &str {
    let trimmed = text.trim();

    // Try markdown code block
    if let Some(start) = trimmed.find("```json") {
        let after_marker = &trimmed[start + 7..];
        if let Some(end) = after_marker.find("```") {
            return after_marker[..end].trim();
        }
    }
    if let Some(start) = trimmed.find("```") {
        let after_marker = &trimmed[start + 3..];
        // Skip optional language tag on same line
        let after_newline = after_marker
            .find('\n')
            .map(|i| &after_marker[i + 1..])
            .unwrap_or(after_marker);
        if let Some(end) = after_newline.find("```") {
            return after_newline[..end].trim();
        }
    }

    // Try to find JSON object/array boundaries
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if start < end {
            return &trimmed[start..=end];
        }
    }
    if let (Some(start), Some(end)) = (trimmed.find('['), trimmed.rfind(']')) {
        if start < end {
            return &trimmed[start..=end];
        }
    }

    trimmed
}

fn count_schema_checks(schema: &Schema) -> usize {
    let mut count = 1; // type check
    if let Some(required) = &schema.required {
        count += required.len();
    }
    if let Some(props) = &schema.properties {
        for prop_schema in props.values() {
            count += count_schema_checks(prop_schema);
        }
    }
    if let Some(items) = &schema.items {
        count += count_schema_checks(items);
    }
    if schema.minimum.is_some() {
        count += 1;
    }
    if schema.maximum.is_some() {
        count += 1;
    }
    if schema.min_length.is_some() {
        count += 1;
    }
    if schema.max_length.is_some() {
        count += 1;
    }
    if schema.min_items.is_some() {
        count += 1;
    }
    if schema.max_items.is_some() {
        count += 1;
    }
    if schema.enum_values.is_some() {
        count += 1;
    }
    if schema.pattern.is_some() {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERSON_SCHEMA: &str = r#"{
        "type": "object",
        "required": ["name", "age"],
        "properties": {
            "name": { "type": "string", "minLength": 1 },
            "age": { "type": "integer", "minimum": 0, "maximum": 150 },
            "email": { "type": "string", "pattern": "^[^@]+@[^@]+$" }
        }
    }"#;

    // ========== JSON Extraction ==========

    #[test]
    fn extract_from_code_block() {
        let text = "Here is the JSON:\n```json\n{\"name\": \"Alice\"}\n```";
        assert_eq!(extract_json(text), "{\"name\": \"Alice\"}");
    }

    #[test]
    fn extract_from_bare_json() {
        let text = "{\"name\": \"Alice\"}";
        assert_eq!(extract_json(text), "{\"name\": \"Alice\"}");
    }

    #[test]
    fn extract_json_with_surrounding_text() {
        let text = "The output is: {\"x\": 1} and that's it.";
        assert_eq!(extract_json(text), "{\"x\": 1}");
    }

    // ========== Schema Validation ==========

    #[test]
    fn valid_person() {
        let result = verify(r#"{"name": "Alice", "age": 30}"#, PERSON_SCHEMA);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn valid_person_with_email() {
        let result = verify(
            r#"{"name": "Bob", "age": 25, "email": "bob@example.com"}"#,
            PERSON_SCHEMA,
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn missing_required_field() {
        let result = verify(r#"{"name": "Alice"}"#, PERSON_SCHEMA);
        assert!(result.score < 1.0, "Missing 'age' should fail");
        assert!(result.score > 0.0, "Valid JSON should get partial credit");
    }

    #[test]
    fn wrong_type_for_age() {
        let result = verify(r#"{"name": "Alice", "age": "thirty"}"#, PERSON_SCHEMA);
        assert!(result.score < 1.0, "String age should fail");
    }

    #[test]
    fn age_out_of_range() {
        let result = verify(r#"{"name": "Alice", "age": 200}"#, PERSON_SCHEMA);
        assert!(result.score < 1.0, "Age 200 > maximum 150");
    }

    #[test]
    fn invalid_email_pattern() {
        let result = verify(
            r#"{"name": "Alice", "age": 30, "email": "no-at-sign"}"#,
            PERSON_SCHEMA,
        );
        assert!(result.score < 1.0, "Email without @ should fail pattern");
    }

    #[test]
    fn empty_name() {
        let result = verify(r#"{"name": "", "age": 30}"#, PERSON_SCHEMA);
        assert!(result.score < 1.0, "Empty name violates minLength 1");
    }

    // ========== Array Schema ==========

    #[test]
    fn valid_array() {
        let schema = r#"{
            "type": "array",
            "items": { "type": "integer", "minimum": 0 },
            "minItems": 1,
            "maxItems": 5
        }"#;
        let result = verify("[1, 2, 3]", schema);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn array_too_short() {
        let schema = r#"{
            "type": "array",
            "items": { "type": "integer" },
            "minItems": 3
        }"#;
        let result = verify("[1]", schema);
        assert!(result.score < 1.0);
    }

    #[test]
    fn array_wrong_item_type() {
        let schema = r#"{
            "type": "array",
            "items": { "type": "string" }
        }"#;
        let result = verify(r#"[1, 2, 3]"#, schema);
        assert!(
            result.score < 1.0,
            "Integer items should fail string schema"
        );
    }

    // ========== Enum Schema ==========

    #[test]
    fn valid_enum() {
        let schema = r#"{ "type": "string", "enum": ["red", "green", "blue"] }"#;
        let result = verify(r#""red""#, schema);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn invalid_enum() {
        let schema = r#"{ "type": "string", "enum": ["red", "green", "blue"] }"#;
        let result = verify(r#""yellow""#, schema);
        assert!(result.score < 1.0);
    }

    // ========== Edge Cases ==========

    #[test]
    fn not_json_at_all() {
        let result = verify("This is just plain text, not JSON.", PERSON_SCHEMA);
        assert!(result.score <= 0.1, "Non-JSON should score very low");
    }

    #[test]
    fn json_in_markdown_code_block() {
        let text = "Here's the person:\n```json\n{\"name\": \"Alice\", \"age\": 30}\n```\nDone!";
        let result = verify(text, PERSON_SCHEMA);
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_schemas_different_results() {
        let int_schema = r#"{ "type": "integer" }"#;
        let str_schema = r#"{ "type": "string" }"#;

        // "42" is valid integer, not valid string (as JSON, "42" without quotes is integer)
        assert_eq!(verify("42", int_schema).score, 1.0);
        assert!(verify("42", str_schema).score < 1.0);

        // "\"hello\"" is valid string, not valid integer
        assert_eq!(verify(r#""hello""#, str_schema).score, 1.0);
        assert!(verify(r#""hello""#, int_schema).score < 1.0);
    }

    #[test]
    fn antihardcode_same_json_different_schemas() {
        let json = r#"{"x": 5}"#;

        let schema_pass = r#"{ "type": "object", "required": ["x"], "properties": { "x": { "type": "integer" } } }"#;
        let schema_fail = r#"{ "type": "object", "required": ["y"], "properties": { "y": { "type": "string" } } }"#;

        assert_eq!(verify(json, schema_pass).score, 1.0);
        assert!(verify(json, schema_fail).score < 1.0);
    }

    // ========== Nested Object ==========

    #[test]
    fn nested_object_validation() {
        let schema = r#"{
            "type": "object",
            "required": ["user"],
            "properties": {
                "user": {
                    "type": "object",
                    "required": ["id", "name"],
                    "properties": {
                        "id": { "type": "integer", "minimum": 1 },
                        "name": { "type": "string" }
                    }
                }
            }
        }"#;

        let valid = r#"{"user": {"id": 42, "name": "Alice"}}"#;
        assert_eq!(verify(valid, schema).score, 1.0);

        let bad_id = r#"{"user": {"id": 0, "name": "Alice"}}"#;
        assert!(verify(bad_id, schema).score < 1.0, "id=0 < minimum=1");

        let missing_user = r#"{"other": "stuff"}"#;
        assert!(verify(missing_user, schema).score < 1.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_extra_fields_allowed() {
        let schema = r#"{"type": "object", "required": ["name"], "properties": {"name": {"type": "string"}}}"#;
        let result = verify(r#"{"name": "Alice", "age": 30}"#, schema);
        assert_eq!(
            result.score, 1.0,
            "Extra fields should be allowed by default"
        );
    }

    #[test]
    fn adversarial_string_vs_integer() {
        let schema = r#"{"type": "integer"}"#;
        let result = verify(r#""42""#, schema);
        assert!(result.score < 1.0, "String '42' is not integer");
    }

    #[test]
    fn adversarial_multiple_json_in_text() {
        // When multiple JSON objects appear in text, extract_json grabs from first '{' to last '}'
        // which creates an invalid blob — verifier should NOT give full credit
        let schema =
            r#"{"type": "object", "required": ["x"], "properties": {"x": {"type": "integer"}}}"#;
        let result = verify(r#"Answer: {"x": 1} is correct but {"x": 2} is not"#, schema);
        assert!(
            result.score < 1.0,
            "Ambiguous JSON extraction should not score 1.0"
        );
    }

    #[test]
    fn adversarial_deeply_nested_valid() {
        let schema = r#"{
            "type": "object",
            "required": ["a"],
            "properties": {
                "a": {
                    "type": "object",
                    "required": ["b"],
                    "properties": {
                        "b": {
                            "type": "object",
                            "required": ["c"],
                            "properties": {
                                "c": {"type": "integer", "minimum": 0}
                            }
                        }
                    }
                }
            }
        }"#;
        let result = verify(r#"{"a": {"b": {"c": 42}}}"#, schema);
        assert_eq!(result.score, 1.0, "Deeply nested valid structure");

        let result2 = verify(r#"{"a": {"b": {"c": -1}}}"#, schema);
        assert!(result2.score < 1.0, "c=-1 violates minimum=0");
    }
}
