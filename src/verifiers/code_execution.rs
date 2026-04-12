//! Code Execution Verifier
//!
//! Runs model-generated code in a sandboxed subprocess and checks results.
//! Supports: Python (primary), with extension points for other languages.
//!
//! Verification: compile/run the code, execute test cases, compare outputs.
//! This covers: code-generation, code-repair, code-translation, data-wrangling, etc.

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;
use super::VerifyResult;

/// A single test case for code verification.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TestCase {
    pub input: String,
    pub expected_output: String,
}

/// Configuration for code execution.
#[derive(Debug, Clone)]
pub struct ExecConfig {
    pub timeout: Duration,
    pub max_output_bytes: usize,
    pub language: Language,
}

impl Default for ExecConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            max_output_bytes: 1_000_000, // 1MB
            language: Language::Python,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Python,
    JavaScript,
    Rust,
}

/// Result of executing code.
#[derive(Debug)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
}

/// Execute code in a subprocess with timeout.
pub fn execute_code(code: &str, stdin_input: &str, config: &ExecConfig) -> ExecResult {
    let (cmd, args): (&str, Vec<&str>) = match config.language {
        Language::Python => ("python3", vec!["-c", code]),
        Language::JavaScript => ("node", vec!["-e", code]),
        Language::Rust => {
            return ExecResult {
                stdout: String::new(),
                stderr: "Rust execution not yet implemented".to_string(),
                exit_code: None,
                timed_out: false,
            };
        }
    };

    let mut child = match Command::new(cmd)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // Sandbox: restrict environment — clear everything, only keep PATH
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .env("HOME", "/tmp")
        .env("TMPDIR", "/tmp")
        // Sandbox: set working directory to /tmp
        .current_dir("/tmp")
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return ExecResult {
                stdout: String::new(),
                stderr: format!("failed to spawn {cmd}: {e}"),
                exit_code: None,
                timed_out: false,
            };
        }
    };

    // Write stdin
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(stdin_input.as_bytes());
    }

    // Wait with timeout
    match child.wait_timeout(config.timeout) {
        Ok(Some(status)) => {
            let stdout = child.stdout.take()
                .and_then(|mut o| {
                    let mut buf = Vec::new();
                    std::io::Read::read_to_end(&mut o, &mut buf).ok()?;
                    buf.truncate(config.max_output_bytes);
                    String::from_utf8(buf).ok()
                })
                .unwrap_or_default();

            let stderr = child.stderr.take()
                .and_then(|mut e| {
                    let mut buf = Vec::new();
                    std::io::Read::read_to_end(&mut e, &mut buf).ok()?;
                    buf.truncate(config.max_output_bytes);
                    String::from_utf8(buf).ok()
                })
                .unwrap_or_default();

            ExecResult {
                stdout,
                stderr,
                exit_code: status.code(),
                timed_out: false,
            }
        }
        Ok(None) => {
            // Timed out — kill the process
            let _ = child.kill();
            let _ = child.wait();
            ExecResult {
                stdout: String::new(),
                stderr: format!("execution timed out after {:?}", config.timeout),
                exit_code: None,
                timed_out: true,
            }
        }
        Err(e) => ExecResult {
            stdout: String::new(),
            stderr: format!("wait error: {e}"),
            exit_code: None,
            timed_out: false,
        },
    }
}

/// Extract code from model output — handles markdown code blocks.
pub fn extract_code(model_output: &str, language: Language) -> &str {
    let lang_tag = match language {
        Language::Python => "python",
        Language::JavaScript => "javascript",
        Language::Rust => "rust",
    };

    let trimmed = model_output.trim();

    // Try ```python ... ``` first
    if let Some(start) = trimmed.find(&format!("```{lang_tag}")) {
        let after = &trimmed[start + 3 + lang_tag.len()..];
        if let Some(end) = after.find("```") {
            return after[..end].trim();
        }
    }

    // Try bare ``` ... ```
    if let Some(start) = trimmed.find("```") {
        let after = &trimmed[start + 3..];
        let after = after.find('\n').map(|i| &after[i + 1..]).unwrap_or(after);
        if let Some(end) = after.find("```") {
            return after[..end].trim();
        }
    }

    // No code blocks — assume the whole thing is code
    trimmed
}

/// Build a test harness that runs the model's code against test cases.
/// Returns Python code that runs the function and prints outputs.
fn build_python_harness(code: &str, function_name: &str, test_cases: &[TestCase]) -> String {
    let mut harness = String::new();

    // Suppress model's stdout: redirect to devnull, use stderr for harness output
    harness.push_str("import sys as _sys, io as _io, json as _json\n");
    harness.push_str("_real_stderr = _sys.stderr\n");
    harness.push_str("_sys.stdout = _io.StringIO()\n\n");

    // Model's code (any print() calls go to StringIO devnull)
    harness.push_str(code);

    harness.push_str("\n\n_results = []\n");

    for (i, tc) in test_cases.iter().enumerate() {
        harness.push_str(&format!(
            "try:\n    _input_{i} = _json.loads({input_json})\n    if isinstance(_input_{i}, list):\n        _out = {func}(*_input_{i})\n    else:\n        _out = {func}(_input_{i})\n    _results.append({{'pass': True, 'output': repr(_out)}})\nexcept Exception as e:\n    _results.append({{'pass': False, 'error': str(e)}})\n",
            i = i,
            input_json = serde_json::to_string(&tc.input).unwrap_or_else(|_| format!("\"{}\"", tc.input)),
            func = function_name,
        ));
    }

    // Output results via stderr (immune to model's stdout manipulation)
    // The verify function reads from stderr for function mode
    harness.push_str("print(_json.dumps(_results), file=_real_stderr)\n");
    harness
}

/// Build a simpler harness: code reads from stdin, writes to stdout.
/// Each test case is run separately.
fn build_stdin_stdout_harness(code: &str) -> String {
    code.to_string()
}

/// Verify model-generated code against test cases.
///
/// `function_name`: if Some, wrap in a function-call harness.
///                  if None, use stdin/stdout per test case.
pub fn verify(
    model_output: &str,
    test_cases: &[TestCase],
    function_name: Option<&str>,
    config: &ExecConfig,
) -> VerifyResult {
    if test_cases.is_empty() {
        return VerifyResult::wrong("no test cases provided");
    }

    let code = extract_code(model_output, config.language);

    if let Some(func_name) = function_name {
        // Function-call mode: run all tests in one process
        verify_function_mode(code, func_name, test_cases, config)
    } else {
        // Stdin/stdout mode: run code once per test case
        verify_stdio_mode(code, test_cases, config)
    }
}

fn verify_function_mode(
    code: &str,
    function_name: &str,
    test_cases: &[TestCase],
    config: &ExecConfig,
) -> VerifyResult {
    let harness = build_python_harness(code, function_name, test_cases);
    let result = execute_code(&harness, "", config);

    if result.timed_out {
        return VerifyResult::wrong("execution timed out");
    }

    // In function mode, the harness outputs JSON to stderr (to avoid model's print corruption)
    // A non-zero exit code is okay if we still get valid JSON on stderr

    // Parse JSON results from stderr (where the harness writes them)
    let results: Vec<serde_json::Value> = match serde_json::from_str(&result.stderr.trim()) {
        Ok(r) => r,
        Err(e) => {
            // If we can't parse stderr, check if it was a runtime error
            if result.exit_code != Some(0) {
                return VerifyResult::wrong(format!(
                    "runtime error (exit {}): {}",
                    result.exit_code.unwrap_or(-1),
                    truncate(&result.stderr, 200)
                ));
            }
            return VerifyResult::wrong(format!("failed to parse test results: {e}. stderr: {}", truncate(&result.stderr, 200)));
        }
    };

    let mut passed = 0;
    let total = test_cases.len();

    for (i, (tc, res)) in test_cases.iter().zip(results.iter()).enumerate() {
        let did_pass = res.get("pass").and_then(|v| v.as_bool()).unwrap_or(false);
        if did_pass {
            if let Some(output) = res.get("output").and_then(|v| v.as_str()) {
                if outputs_match(output, &tc.expected_output) {
                    passed += 1;
                }
            }
        }
    }

    if passed == total {
        VerifyResult::correct()
    } else {
        VerifyResult::partial(
            passed as f64 / total as f64,
            format!("{passed}/{total} test cases passed"),
        )
    }
}

fn verify_stdio_mode(
    code: &str,
    test_cases: &[TestCase],
    config: &ExecConfig,
) -> VerifyResult {
    let mut passed = 0;
    let total = test_cases.len();

    for tc in test_cases {
        let result = execute_code(code, &tc.input, config);
        if result.timed_out {
            continue;
        }
        if result.exit_code != Some(0) {
            continue;
        }
        if outputs_match(&result.stdout.trim(), &tc.expected_output.trim()) {
            passed += 1;
        }
    }

    if passed == total {
        VerifyResult::correct()
    } else {
        VerifyResult::partial(
            passed as f64 / total as f64,
            format!("{passed}/{total} test cases passed"),
        )
    }
}

/// Compare outputs, handling whitespace normalization.
fn outputs_match(actual: &str, expected: &str) -> bool {
    let a = actual.trim();
    let e = expected.trim();

    if a == e {
        return true;
    }

    // Try normalizing whitespace
    let a_norm: String = a.split_whitespace().collect::<Vec<_>>().join(" ");
    let e_norm: String = e.split_whitespace().collect::<Vec<_>>().join(" ");
    if a_norm == e_norm {
        return true;
    }

    // Try repr() vs raw comparison (Python repr adds quotes around strings)
    let a_unquoted = a.trim_matches('\'').trim_matches('"');
    if a_unquoted == e {
        return true;
    }

    false
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max { s } else { &s[..max] }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> ExecConfig {
        ExecConfig {
            timeout: Duration::from_secs(5),
            ..Default::default()
        }
    }

    // ========== Code Extraction ==========

    #[test]
    fn extract_from_python_block() {
        let output = "Here's the solution:\n```python\ndef add(a, b):\n    return a + b\n```";
        assert_eq!(extract_code(output, Language::Python), "def add(a, b):\n    return a + b");
    }

    #[test]
    fn extract_from_bare_code() {
        let output = "def add(a, b):\n    return a + b";
        assert_eq!(extract_code(output, Language::Python), output);
    }

    // ========== Basic Execution ==========

    #[test]
    fn execute_simple_python() {
        let result = execute_code("print('hello')", "", &default_config());
        assert_eq!(result.stdout.trim(), "hello");
        assert_eq!(result.exit_code, Some(0));
        assert!(!result.timed_out);
    }

    #[test]
    fn execute_python_with_stdin() {
        let code = "x = input()\nprint(f'got: {x}')";
        let result = execute_code(code, "test_input", &default_config());
        assert_eq!(result.stdout.trim(), "got: test_input");
    }

    #[test]
    fn execute_python_syntax_error() {
        let result = execute_code("def broken(:", "", &default_config());
        assert_ne!(result.exit_code, Some(0));
        assert!(result.stderr.contains("SyntaxError"));
    }

    #[test]
    fn execute_python_timeout() {
        let config = ExecConfig {
            timeout: Duration::from_secs(1),
            ..default_config()
        };
        let result = execute_code("import time; time.sleep(10)", "", &config);
        assert!(result.timed_out);
    }

    #[test]
    fn execute_python_runtime_error() {
        let result = execute_code("1/0", "", &default_config());
        assert_ne!(result.exit_code, Some(0));
        assert!(result.stderr.contains("ZeroDivisionError"));
    }

    // ========== Stdio Mode Verification ==========

    #[test]
    fn verify_stdio_correct() {
        let code = "n = int(input())\nprint(n * 2)";
        let tests = vec![
            TestCase { input: "5".into(), expected_output: "10".into() },
            TestCase { input: "0".into(), expected_output: "0".into() },
            TestCase { input: "100".into(), expected_output: "200".into() },
        ];
        let result = verify(code, &tests, None, &default_config());
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_stdio_wrong() {
        let code = "n = int(input())\nprint(n + 1)"; // adds 1 instead of doubling
        let tests = vec![
            TestCase { input: "5".into(), expected_output: "10".into() },
        ];
        let result = verify(code, &tests, None, &default_config());
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn verify_stdio_partial() {
        let code = "n = int(input())\nprint(n * 2)";
        let tests = vec![
            TestCase { input: "5".into(), expected_output: "10".into() },  // pass
            TestCase { input: "3".into(), expected_output: "7".into() },   // fail (6 != 7)
        ];
        let result = verify(code, &tests, None, &default_config());
        assert!((result.score - 0.5).abs() < 0.01);
    }

    // ========== Function Mode Verification ==========

    #[test]
    fn verify_function_correct() {
        let code = "def add(a, b):\n    return a + b";
        let tests = vec![
            TestCase { input: "[2, 3]".into(), expected_output: "5".into() },
            TestCase { input: "[0, 0]".into(), expected_output: "0".into() },
            TestCase { input: "[-1, 1]".into(), expected_output: "0".into() },
        ];
        let result = verify(code, &tests, Some("add"), &default_config());
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn verify_function_wrong() {
        let code = "def add(a, b):\n    return a - b"; // subtract instead of add
        let tests = vec![
            TestCase { input: "[2, 3]".into(), expected_output: "5".into() },
        ];
        let result = verify(code, &tests, Some("add"), &default_config());
        assert_eq!(result.score, 0.0);
    }

    // ========== Code Block Extraction + Verification ==========

    #[test]
    fn verify_with_markdown_code_block() {
        let model_output = "Here's my solution:\n```python\ndef double(n):\n    return n * 2\n```";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
            TestCase { input: "[0]".into(), expected_output: "0".into() },
        ];
        let result = verify(model_output, &tests, Some("double"), &default_config());
        assert_eq!(result.score, 1.0);
    }

    // ========== Anti-Hardcoding ==========

    #[test]
    fn antihardcode_different_functions_different_results() {
        let correct_code = "def f(x): return x * 2";
        let wrong_code = "def f(x): return x * 3";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];

        assert_eq!(verify(correct_code, &tests, Some("f"), &default_config()).score, 1.0);
        assert_eq!(verify(wrong_code, &tests, Some("f"), &default_config()).score, 0.0);
    }

    #[test]
    fn antihardcode_multiple_problems() {
        // Problem 1: FizzBuzz
        let fizzbuzz = r#"
def fizzbuzz(n):
    if n % 15 == 0: return "FizzBuzz"
    if n % 3 == 0: return "Fizz"
    if n % 5 == 0: return "Buzz"
    return str(n)
"#;
        let fb_tests = vec![
            TestCase { input: "[3]".into(), expected_output: "Fizz".into() },
            TestCase { input: "[5]".into(), expected_output: "Buzz".into() },
            TestCase { input: "[15]".into(), expected_output: "FizzBuzz".into() },
            TestCase { input: "[7]".into(), expected_output: "7".into() },
        ];
        assert_eq!(verify(fizzbuzz, &fb_tests, Some("fizzbuzz"), &default_config()).score, 1.0);

        // Problem 2: Reverse string
        let reverse = "def reverse(s): return s[::-1]";
        let rev_tests = vec![
            TestCase { input: "[\"hello\"]".into(), expected_output: "olleh".into() },
            TestCase { input: "[\"abc\"]".into(), expected_output: "cba".into() },
        ];
        assert_eq!(verify(reverse, &rev_tests, Some("reverse"), &default_config()).score, 1.0);
    }

    // ========== HumanEval-style Problem ==========

    #[test]
    fn humaneval_style_has_close_elements() {
        // HumanEval/0: has_close_elements
        let code = r#"
def has_close_elements(numbers, threshold):
    for i in range(len(numbers)):
        for j in range(i + 1, len(numbers)):
            if abs(numbers[i] - numbers[j]) < threshold:
                return True
    return False
"#;
        let tests = vec![
            TestCase {
                input: "[[1.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.3]".into(),
                expected_output: "True".into(),
            },
            TestCase {
                input: "[[1.0, 2.0, 3.9, 4.0, 5.0, 2.2], 0.05]".into(),
                expected_output: "False".into(),
            },
            TestCase {
                input: "[[1.0, 2.0, 5.9, 4.0, 5.0], 0.95]".into(),
                expected_output: "True".into(),
            },
        ];
        let result = verify(code, &tests, Some("has_close_elements"), &default_config());
        assert_eq!(result.score, 1.0);
    }

    // ========== ADVERSARIAL TESTS ==========

    #[test]
    fn adversarial_code_with_print_in_function_mode() {
        // Model's code prints stuff — should NOT corrupt harness JSON output
        let code = r#"
print("Debug: starting computation")
def add(a, b):
    print(f"Adding {a} + {b}")
    return a + b
print("Function defined!")
"#;
        let tests = vec![
            TestCase { input: "[2, 3]".into(), expected_output: "5".into() },
        ];
        let result = verify(code, &tests, Some("add"), &default_config());
        assert_eq!(result.score, 1.0, "Print statements should not break function mode");
    }

    #[test]
    fn adversarial_hardcoded_output() {
        // Model hardcodes the expected output instead of computing it
        // This SHOULD pass (we can't distinguish hardcoding from real computation)
        // But with held-out tests it would fail
        let code = "def f(x): return 10";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];
        assert_eq!(verify(code, &tests, Some("f"), &default_config()).score, 1.0);

        // But it fails on a different input
        let tests2 = vec![
            TestCase { input: "[3]".into(), expected_output: "6".into() },
        ];
        assert_eq!(verify(code, &tests2, Some("f"), &default_config()).score, 0.0);
    }

    #[test]
    fn adversarial_import_os() {
        // Model tries to import os — this should still work (we sandbox via env/cwd)
        // but shouldn't be able to do damage
        let code = "import os\ndef f(x): return x * 2";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];
        let result = verify(code, &tests, Some("f"), &default_config());
        assert_eq!(result.score, 1.0, "Import os is allowed, just sandboxed");
    }

    #[test]
    fn adversarial_infinite_output() {
        // Code that produces massive output
        let config = ExecConfig {
            timeout: Duration::from_secs(2),
            max_output_bytes: 1000,
            ..default_config()
        };
        let code = "print('x' * 10000000)";
        let result = execute_code(code, "", &config);
        // Should not crash — output is truncated
        assert!(result.stdout.len() <= 1000 || result.timed_out);
    }

    #[test]
    fn adversarial_exception_in_function() {
        // Function raises an exception — should score 0, not crash the verifier
        let code = "def f(x): raise ValueError('nope')";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];
        let result = verify(code, &tests, Some("f"), &default_config());
        assert_eq!(result.score, 0.0, "Exception should score 0");
    }

    #[test]
    fn adversarial_wrong_function_name() {
        // Model defines a different function name than expected
        let code = "def add(a, b): return a + b";
        let tests = vec![
            TestCase { input: "[2, 3]".into(), expected_output: "5".into() },
        ];
        // Looking for function "multiply" but code defines "add"
        let result = verify(code, &tests, Some("multiply"), &default_config());
        assert_eq!(result.score, 0.0, "Wrong function name should fail");
    }

    #[test]
    fn adversarial_none_return() {
        // Function returns None when it should return a value
        let code = "def f(x): pass";
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];
        let result = verify(code, &tests, Some("f"), &default_config());
        assert_eq!(result.score, 0.0, "None != 10");
    }

    #[test]
    fn adversarial_empty_code() {
        let tests = vec![
            TestCase { input: "[5]".into(), expected_output: "10".into() },
        ];
        let result = verify("", &tests, Some("f"), &default_config());
        assert_eq!(result.score, 0.0, "Empty code should fail");
    }
}
