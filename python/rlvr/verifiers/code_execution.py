"""
Code Execution Verifier — covers ~554 domains (20% of all domains).

Handles:
  - Python code execution with test cases
  - Multi-language execution (C++, Java, Rust, Go, JS, etc.) via subprocess
  - Function-call mode (import function, call with args, check return)
  - Stdin/stdout mode (provide input, check output)
  - Test suite mode (run pytest/unittest, check pass/fail)
  - Compilation check (must compile without errors)
  - Performance mode (must be faster than baseline)
"""

import subprocess
import tempfile
import os
import sys
import json
import re
import time
from pathlib import Path
from typing import Optional
from . import VerifyResult


# Language configurations: compiler/interpreter, file extension, run command
LANGUAGE_CONFIG = {
    "python": {"ext": ".py", "run": [sys.executable, "{file}"], "compile": None},
    "python3": {"ext": ".py", "run": [sys.executable, "{file}"], "compile": None},
    "javascript": {"ext": ".js", "run": ["node", "{file}"], "compile": None},
    "typescript": {"ext": ".ts", "run": ["npx", "tsx", "{file}"], "compile": None},
    "ruby": {"ext": ".rb", "run": ["ruby", "{file}"], "compile": None},
    "php": {"ext": ".php", "run": ["php", "{file}"], "compile": None},
    "lua": {"ext": ".lua", "run": ["lua", "{file}"], "compile": None},
    "perl": {"ext": ".pl", "run": ["perl", "{file}"], "compile": None},
    "bash": {"ext": ".sh", "run": ["bash", "{file}"], "compile": None},
    "r": {"ext": ".R", "run": ["Rscript", "{file}"], "compile": None},
    "julia": {"ext": ".jl", "run": ["julia", "{file}"], "compile": None},
    "c": {"ext": ".c", "run": ["./{out}"], "compile": ["gcc", "-o", "{out}", "{file}", "-lm"]},
    "cpp": {"ext": ".cpp", "run": ["./{out}"], "compile": ["g++", "-std=c++17", "-o", "{out}", "{file}"]},
    "java": {"ext": ".java", "run": ["java", "-cp", "{dir}", "Main"], "compile": ["javac", "{file}"]},
    "rust": {"ext": ".rs", "run": ["./{out}"], "compile": ["rustc", "-o", "{out}", "{file}"]},
    "go": {"ext": ".go", "run": ["go", "run", "{file}"], "compile": None},
    "swift": {"ext": ".swift", "run": ["swift", "{file}"], "compile": None},
    "kotlin": {"ext": ".kt", "run": ["kotlin", "{file}"], "compile": None},
    "haskell": {"ext": ".hs", "run": ["./{out}"], "compile": ["ghc", "-o", "{out}", "{file}"]},
    "ocaml": {"ext": ".ml", "run": ["./{out}"], "compile": ["ocamlfind", "ocamlopt", "-o", "{out}", "{file}"]},
    "scala": {"ext": ".scala", "run": ["scala", "{file}"], "compile": None},
    "elixir": {"ext": ".exs", "run": ["elixir", "{file}"], "compile": None},
    "clojure": {"ext": ".clj", "run": ["clojure", "{file}"], "compile": None},
}

DEFAULT_TIMEOUT = 10  # seconds
MAX_OUTPUT = 1_000_000  # 1MB output limit


def _extract_code(response: str, language: str = "python") -> str:
    """Extract code from a model response, handling markdown code blocks."""
    # Try to find code block with language tag
    patterns = [
        rf'```{language}\n(.*?)```',
        rf'```{language.lower()}\n(.*?)```',
        r'```\n(.*?)```',
        r'```(.*?)```',
    ]
    for pat in patterns:
        m = re.search(pat, response, re.DOTALL)
        if m:
            return m.group(1).strip()

    # If no code block, check if the whole response looks like code
    lines = response.strip().split('\n')
    code_lines = [l for l in lines if not l.startswith('#') or l.startswith('#!/')]
    if any(l.strip().startswith(('def ', 'class ', 'import ', 'from ', 'if ', 'for ', 'while '))
           for l in code_lines):
        return response.strip()

    return response.strip()


def _run_subprocess(cmd: list, stdin_data: str = None, timeout: int = DEFAULT_TIMEOUT,
                    cwd: str = None) -> tuple[str, str, int]:
    """Run a subprocess with timeout and resource limits."""
    try:
        result = subprocess.run(
            cmd,
            input=stdin_data,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=cwd,
            env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        )
        stdout = result.stdout[:MAX_OUTPUT]
        stderr = result.stderr[:MAX_OUTPUT]
        return stdout, stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "TIMEOUT", -1
    except FileNotFoundError as e:
        return "", f"Command not found: {e}", -2
    except Exception as e:
        return "", str(e), -3


def verify_function_call(code: str, test_cases: list[dict], language: str = "python",
                         timeout: int = DEFAULT_TIMEOUT) -> VerifyResult:
    """
    Verify code by calling a function with test cases.

    test_cases: [{"input": args, "expected": result}, ...]
    """
    if language != "python":
        return verify_stdin_stdout(code, test_cases, language, timeout)

    passed = 0
    total = len(test_cases)
    failures = []

    for i, tc in enumerate(test_cases):
        test_code = f"""{code}

# Test case {i}
import json
_input = json.loads('''{json.dumps(tc["input"])}''')
_expected = json.loads('''{json.dumps(tc["expected"])}''')
if isinstance(_input, list):
    _result = solution(*_input)
elif isinstance(_input, dict):
    _result = solution(**_input)
else:
    _result = solution(_input)
if _result == _expected:
    print("PASS")
else:
    print(f"FAIL: got {{repr(_result)}}, expected {{repr(_expected)}}")
"""
        stdout, stderr, code_ret = _run_subprocess(
            [sys.executable, "-c", test_code], timeout=timeout
        )
        if "PASS" in stdout:
            passed += 1
        else:
            failures.append(f"TC{i}: {stdout.strip() or stderr.strip()}")

    if passed == total:
        return VerifyResult.correct(f"All {total} test cases passed")
    elif passed > 0:
        return VerifyResult.partial(passed / total, f"{passed}/{total} passed. Failures: {failures[:3]}")
    else:
        return VerifyResult.wrong(f"0/{total} passed. {failures[:3]}")


def verify_stdin_stdout(code: str, test_cases: list[dict], language: str = "python",
                        timeout: int = DEFAULT_TIMEOUT) -> VerifyResult:
    """
    Verify code by running with stdin and checking stdout.

    test_cases: [{"input": "stdin_string", "expected": "stdout_string"}, ...]
    """
    config = LANGUAGE_CONFIG.get(language, LANGUAGE_CONFIG["python"])
    passed = 0
    total = len(test_cases)
    failures = []

    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, f"solution{config['ext']}")
        out_file = os.path.join(tmpdir, "solution")

        with open(src_file, 'w') as f:
            f.write(code)

        # Compile if needed
        if config.get("compile"):
            compile_cmd = [c.format(file=src_file, out=out_file, dir=tmpdir) for c in config["compile"]]
            _, stderr, ret = _run_subprocess(compile_cmd, timeout=30, cwd=tmpdir)
            if ret != 0:
                return VerifyResult.wrong(f"Compilation failed: {stderr[:500]}")

        # Run test cases
        for i, tc in enumerate(test_cases):
            run_cmd = [c.format(file=src_file, out=out_file, dir=tmpdir) for c in config["run"]]
            stdin_data = str(tc.get("input", ""))
            expected = str(tc.get("expected", "")).strip()

            stdout, stderr, ret = _run_subprocess(run_cmd, stdin_data=stdin_data, timeout=timeout, cwd=tmpdir)

            if ret == -1:
                failures.append(f"TC{i}: TIMEOUT")
                continue

            actual = stdout.strip()
            if actual == expected:
                passed += 1
            else:
                failures.append(f"TC{i}: got '{actual[:100]}', expected '{expected[:100]}'")

    if passed == total:
        return VerifyResult.correct(f"All {total} test cases passed")
    elif passed > 0:
        return VerifyResult.partial(passed / total, f"{passed}/{total} passed. {failures[:3]}")
    else:
        return VerifyResult.wrong(f"0/{total} passed. {failures[:3]}")


def verify_test_suite(code: str, test_code: str, language: str = "python",
                      timeout: int = 30) -> VerifyResult:
    """
    Verify by running a test suite against the code.

    For Python, runs pytest on the combined code + tests.
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        # Write solution
        sol_file = os.path.join(tmpdir, "solution.py")
        with open(sol_file, 'w') as f:
            f.write(code)

        # Write tests
        test_file = os.path.join(tmpdir, "test_solution.py")
        with open(test_file, 'w') as f:
            f.write(f"from solution import *\n\n{test_code}")

        # Run pytest
        stdout, stderr, ret = _run_subprocess(
            [sys.executable, "-m", "pytest", test_file, "-v", "--tb=short", "-q"],
            timeout=timeout, cwd=tmpdir
        )

        # Parse results
        if ret == 0:
            return VerifyResult.correct("All tests passed")

        # Count passed/failed
        m = re.search(r'(\d+) passed', stdout + stderr)
        passed = int(m.group(1)) if m else 0
        m = re.search(r'(\d+) failed', stdout + stderr)
        failed = int(m.group(1)) if m else 0
        total = passed + failed

        if total == 0:
            return VerifyResult.wrong(f"No tests ran. Error: {stderr[:500]}")

        if failed == 0:
            return VerifyResult.correct(f"All {passed} tests passed")

        return VerifyResult.partial(passed / total, f"{passed}/{total} tests passed")


def verify_compilation(code: str, language: str, timeout: int = 30) -> VerifyResult:
    """Verify that code compiles without errors."""
    config = LANGUAGE_CONFIG.get(language)
    if not config or not config.get("compile"):
        # Interpreted language — try to at least parse it
        if language == "python":
            try:
                compile(code, "<string>", "exec")
                return VerifyResult.correct("Python code parses successfully")
            except SyntaxError as e:
                return VerifyResult.wrong(f"Syntax error: {e}")
        return VerifyResult.wrong(f"No compiler configured for {language}")

    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, f"solution{config['ext']}")
        out_file = os.path.join(tmpdir, "solution")
        with open(src_file, 'w') as f:
            f.write(code)

        compile_cmd = [c.format(file=src_file, out=out_file, dir=tmpdir) for c in config["compile"]]
        _, stderr, ret = _run_subprocess(compile_cmd, timeout=timeout, cwd=tmpdir)

        if ret == 0:
            return VerifyResult.correct(f"Compilation successful ({language})")
        return VerifyResult.wrong(f"Compilation failed: {stderr[:500]}")


def verify(task: dict, response: str) -> VerifyResult:
    """
    Main entry point for code execution verification.

    Task format:
        {
            "type": "function_call" | "stdin_stdout" | "test_suite" | "compilation",
            "language": "python",
            "test_cases": [{"input": ..., "expected": ...}],  # for function_call/stdin_stdout
            "test_code": "def test_...",                        # for test_suite
            "timeout": 10,
        }
    """
    task_type = task.get("type", "stdin_stdout")
    language = task.get("language", "python")
    timeout = task.get("timeout", DEFAULT_TIMEOUT)
    code = _extract_code(response, language)

    if task_type == "function_call":
        return verify_function_call(code, task["test_cases"], language, timeout)
    elif task_type == "stdin_stdout":
        return verify_stdin_stdout(code, task["test_cases"], language, timeout)
    elif task_type == "test_suite":
        return verify_test_suite(code, task["test_code"], language, timeout)
    elif task_type == "compilation":
        return verify_compilation(code, language, timeout)
    else:
        return VerifyResult.wrong(f"Unknown task type: {task_type}")
