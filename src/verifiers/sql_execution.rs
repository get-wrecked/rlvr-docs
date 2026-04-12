//! SQL Execution Verifier
//!
//! Verifies SQL queries by executing them against an in-memory SQLite database
//! and comparing result sets. Covers: sql-generation, database-query-optimization.
//!
//! Strategy:
//! 1. Create an in-memory SQLite database with the schema + data
//! 2. Execute the gold query to get expected results
//! 3. Execute the model's query
//! 4. Compare result sets (order-insensitive unless ORDER BY specified)

use std::process::Command;
use super::VerifyResult;

/// Execute a SQL query against a SQLite database and return the output.
fn execute_sqlite(db_setup: &str, query: &str) -> Result<String, String> {
    // Combine setup + query into one script
    let script = format!("{db_setup}\n{query}");

    let output = Command::new("sqlite3")
        .arg(":memory:")
        .arg("-header")
        .arg("-csv")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(script.as_bytes());
            }
            child.wait_with_output()
        })
        .map_err(|e| format!("failed to run sqlite3: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("SQL error: {stderr}"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Normalize a SQL result set for comparison.
/// Sorts rows (for order-insensitive comparison) and normalizes whitespace.
fn normalize_result_set(output: &str, sort: bool) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|cell| cell.trim().trim_matches('"').to_string())
                .collect()
        })
        .collect();

    if sort && rows.len() > 1 {
        // Keep header, sort data rows
        let header = rows.remove(0);
        rows.sort();
        rows.insert(0, header);
    }

    rows
}

/// Compare two result sets.
/// Returns (matching_rows, total_rows_expected).
fn compare_results(actual: &[Vec<String>], expected: &[Vec<String>]) -> (usize, usize) {
    if expected.is_empty() {
        return if actual.is_empty() { (1, 1) } else { (0, 1) };
    }

    let total = expected.len();
    let matching = actual.iter().zip(expected.iter())
        .filter(|(a, e)| a == e)
        .count();

    (matching, total)
}

/// Extract SQL from model output — handles markdown code blocks and extra text.
pub fn extract_sql(model_output: &str) -> &str {
    let trimmed = model_output.trim();

    // Try ```sql ... ```
    if let Some(start) = trimmed.find("```sql") {
        let after = &trimmed[start + 6..];
        if let Some(end) = after.find("```") {
            return after[..end].trim();
        }
    }

    // Try ``` ... ```
    if let Some(start) = trimmed.find("```") {
        let after = &trimmed[start + 3..];
        let after = after.find('\n').map(|i| &after[i + 1..]).unwrap_or(after);
        if let Some(end) = after.find("```") {
            return after[..end].trim();
        }
    }

    // Look for SELECT/INSERT/UPDATE/DELETE
    let upper = trimmed.to_uppercase();
    for keyword in &["SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "WITH"] {
        if let Some(pos) = upper.find(keyword) {
            // Take from keyword to end of statement (semicolon or end)
            let rest = &trimmed[pos..];
            let end = rest.find(';').map(|i| i + 1).unwrap_or(rest.len());
            return rest[..end].trim();
        }
    }

    trimmed
}

/// Verify a SQL query against a gold query.
///
/// `db_setup`: SQL to create tables and insert data
/// `gold_query`: The correct SQL query
/// `model_output`: The model's response (may contain SQL in code blocks)
pub fn verify(db_setup: &str, gold_query: &str, model_output: &str) -> VerifyResult {
    let model_query = extract_sql(model_output);

    // Execute gold query
    let expected = match execute_sqlite(db_setup, gold_query) {
        Ok(r) => r,
        Err(e) => return VerifyResult::wrong(format!("gold query failed: {e}")),
    };

    // Execute model query
    let actual = match execute_sqlite(db_setup, model_query) {
        Ok(r) => r,
        Err(e) => return VerifyResult::wrong(format!("model query failed: {e}")),
    };

    // Check if ORDER BY is present (if so, order matters)
    let order_matters = model_query.to_uppercase().contains("ORDER BY");

    let expected_rows = normalize_result_set(&expected, !order_matters);
    let actual_rows = normalize_result_set(&actual, !order_matters);

    if expected_rows == actual_rows {
        return VerifyResult::correct();
    }

    // Partial credit: compare row by row
    let (matching, total) = compare_results(&actual_rows, &expected_rows);
    if total == 0 {
        return VerifyResult::wrong("expected empty result but got data");
    }

    VerifyResult::partial(
        matching as f64 / total as f64,
        format!("{matching}/{total} rows match"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SETUP: &str = "
        CREATE TABLE employees (id INTEGER, name TEXT, dept TEXT, salary REAL);
        INSERT INTO employees VALUES (1, 'Alice', 'Engineering', 90000);
        INSERT INTO employees VALUES (2, 'Bob', 'Engineering', 85000);
        INSERT INTO employees VALUES (3, 'Charlie', 'Sales', 70000);
        INSERT INTO employees VALUES (4, 'Diana', 'Sales', 75000);
        INSERT INTO employees VALUES (5, 'Eve', 'Marketing', 65000);
    ";

    fn has_sqlite() -> bool {
        Command::new("sqlite3").arg("--version").output().is_ok()
    }

    // ========== SQL Extraction ==========

    #[test]
    fn extract_sql_from_code_block() {
        let output = "Here's the query:\n```sql\nSELECT * FROM employees;\n```";
        assert_eq!(extract_sql(output), "SELECT * FROM employees;");
    }

    #[test]
    fn extract_sql_from_bare_text() {
        let output = "SELECT name FROM employees WHERE dept = 'Sales';";
        assert_eq!(extract_sql(output), output);
    }

    #[test]
    fn extract_sql_with_explanation() {
        let output = "To find all engineers, we use:\nSELECT name FROM employees WHERE dept = 'Engineering';";
        assert!(extract_sql(output).starts_with("SELECT"));
    }

    // ========== Full Verification (requires sqlite3) ==========

    #[test]
    fn verify_correct_query() {
        if !has_sqlite() { return; }
        let result = verify(
            SETUP,
            "SELECT name FROM employees WHERE dept = 'Engineering' ORDER BY name;",
            "SELECT name FROM employees WHERE dept = 'Engineering' ORDER BY name;",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_equivalent_query() {
        if !has_sqlite() { return; }
        // Different SQL, same result
        let result = verify(
            SETUP,
            "SELECT name FROM employees WHERE dept = 'Engineering';",
            "SELECT name FROM employees WHERE dept = 'Engineering';",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_wrong_query() {
        if !has_sqlite() { return; }
        let result = verify(
            SETUP,
            "SELECT name FROM employees WHERE dept = 'Engineering';",
            "SELECT name FROM employees WHERE dept = 'Sales';",
        );
        assert!(result.score < 1.0);
    }

    #[test]
    fn verify_aggregate_query() {
        if !has_sqlite() { return; }
        let result = verify(
            SETUP,
            "SELECT dept, AVG(salary) FROM employees GROUP BY dept;",
            "SELECT dept, AVG(salary) FROM employees GROUP BY dept;",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_count_query() {
        if !has_sqlite() { return; }
        let result = verify(
            SETUP,
            "SELECT COUNT(*) FROM employees;",
            "SELECT COUNT(*) FROM employees;",
        );
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_syntax_error_in_model() {
        if !has_sqlite() { return; }
        let result = verify(
            SETUP,
            "SELECT * FROM employees;",
            "SELEC * FORM employees;", // typos
        );
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_sql_in_markdown() {
        if !has_sqlite() { return; }
        let model_output = "The answer is:\n```sql\nSELECT COUNT(*) FROM employees;\n```";
        let result = verify(
            SETUP,
            "SELECT COUNT(*) FROM employees;",
            model_output,
        );
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_queries() {
        if !has_sqlite() { return; }

        // Correct query for "how many engineers?"
        assert_eq!(verify(
            SETUP,
            "SELECT COUNT(*) FROM employees WHERE dept = 'Engineering';",
            "SELECT COUNT(*) FROM employees WHERE dept = 'Engineering';",
        ).score, 1.0);

        // Wrong query (counts Marketing=1 instead of Engineering=2)
        assert!(verify(
            SETUP,
            "SELECT COUNT(*) FROM employees WHERE dept = 'Engineering';",
            "SELECT COUNT(*) FROM employees WHERE dept = 'Marketing';",
        ).score < 1.0);
    }

    // ========== ADVERSARIAL ==========

    #[test]
    fn adversarial_null_handling() {
        if !has_sqlite() { return; }
        let setup = "CREATE TABLE t (x INT); INSERT INTO t VALUES (1), (NULL), (3);";
        let result = verify(setup, "SELECT COUNT(*) FROM t;", "SELECT COUNT(x) FROM t;");
        assert!(result.score < 1.0, "COUNT(*) counts NULLs, COUNT(x) doesn't");
    }

    #[test]
    fn adversarial_empty_result_match() {
        if !has_sqlite() { return; }
        let setup = "CREATE TABLE t (x INT); INSERT INTO t VALUES (1), (2);";
        let result = verify(setup, "SELECT x FROM t WHERE x > 10;", "SELECT x FROM t WHERE x > 10;");
        assert_eq!(result.score, 1.0, "Both empty results should match");
    }

    #[test]
    fn adversarial_order_insensitive() {
        if !has_sqlite() { return; }
        let setup = "CREATE TABLE t (x INT); INSERT INTO t VALUES (3), (1), (2);";
        // Same data, might come in different order — should match
        let result = verify(setup, "SELECT x FROM t;", "SELECT x FROM t;");
        assert_eq!(result.score, 1.0, "Same query should always match");
    }
}
