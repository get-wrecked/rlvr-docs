---
domain: Shell Commands
category: Code & Software
verification_type: execution-based (run command in sandbox, check output and filesystem state)
dataset_scale: 1K–50K command-task pairs
difficulty_range: simple file operations → complex pipelines with awk/sed/xargs
modality: text-to-shell
status: emerging
---

## Overview

Shell command generation tasks the model with translating a natural-language instruction into a bash/shell command (or pipeline) that accomplishes a specified goal. Verification is execution-based: run the command in a sandboxed environment, then check that the filesystem state, stdout, or exit code matches the expected outcome.

This domain is well-suited for RLVR because (a) commands are short and fast to execute, making RL training efficient, (b) verification is concrete — check file contents, directory structure, or output text, and (c) the task has high practical value for developer productivity tools and system administration assistants.

## Verification Mechanism

```
def verify_shell_command(task_description, generated_command,
                         initial_fs_state, expected_outcome):
    # 1. Set up sandbox with initial filesystem state
    sandbox = create_sandbox(
        filesystem=initial_fs_state,  # pre-populated files/dirs
        network=False,                # no network access
        timeout=30,
        user="nobody"                 # unprivileged user
    )

    # 2. Execute the generated command
    try:
        result = sandbox.run(
            command=generated_command,
            shell="/bin/bash",
            timeout=30
        )
    except TimeoutError:
        return 0.0
    except SandboxViolation:
        return 0.0  # tried to escape sandbox

    # 3. Check expected outcome (multiple verification modes)
    checks_passed = []

    # Mode A: Check stdout
    if expected_outcome.stdout is not None:
        checks_passed.append(
            normalize(result.stdout) == normalize(expected_outcome.stdout)
        )

    # Mode B: Check exit code
    if expected_outcome.exit_code is not None:
        checks_passed.append(
            result.returncode == expected_outcome.exit_code
        )

    # Mode C: Check filesystem state
    if expected_outcome.filesystem_checks:
        for check in expected_outcome.filesystem_checks:
            if check.type == "file_exists":
                checks_passed.append(sandbox.file_exists(check.path))
            elif check.type == "file_content":
                actual = sandbox.read_file(check.path)
                checks_passed.append(actual.strip() == check.expected.strip())
            elif check.type == "file_count":
                actual = len(sandbox.list_dir(check.path))
                checks_passed.append(actual == check.expected_count)
            elif check.type == "permissions":
                actual = sandbox.get_permissions(check.path)
                checks_passed.append(actual == check.expected_perms)

    # Mode D: Check stderr (for error-handling tasks)
    if expected_outcome.stderr_pattern is not None:
        checks_passed.append(
            re.search(expected_outcome.stderr_pattern, result.stderr)
        )

    return 1.0 if all(checks_passed) else 0.0
```

Key considerations:

- **Sandboxing is critical**: Shell commands can be destructive (`rm -rf /`). Use containers with read-only root filesystems, no network, and resource limits.
- **Idempotency**: Some tasks can be solved multiple ways. Checking the outcome (file state, output) rather than the exact command ensures multiple valid solutions are accepted.
- **Normalization**: Whitespace, trailing newlines, and sort order differences in output should be normalized before comparison.
- **Stateful environments**: Some tasks require pre-existing files, environment variables, or installed packages. The sandbox must be set up with the correct initial state.

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **NLC2CMD** | 10,044 pairs | NeurIPS 2020 competition | Natural language → bash commands; manually curated |
| **TLDR pages** | ~30,000 commands | tldr-pages project | Simplified man pages with examples; can be reversed into NL→command pairs |
| **Bash-Explainability** | 9,000+ pairs | Mined from web | Command + NL explanation pairs |
| **ShellBench** | 200 tasks | Academic benchmark | Graded difficulty with filesystem verification |
| **InterCode-Bash** | 200 tasks | Princeton | Interactive coding environment with bash tasks |
| **NL2Bash** | 9,305 pairs | Academic | NL descriptions paired with bash commands from experts |
| **CmdChallenge** | ~60 challenges | cmdchallenge.com | Competitive bash challenges with automated checking |
| **ExplainShell corpus** | 10,000+ | explainshell.com | Parsed man pages with command breakdown |

**Synthetic data generation**: Parse man pages for common utilities (find, grep, sed, awk, tar, curl, jq), generate parameter combinations, create NL descriptions, and build verification environments. This can scale to 100K+ training instances.

## Task Format

**Input prompt**:
```
Current directory contains files: data.csv, report.txt, notes.md,
image.png, backup.tar.gz

Task: Find all .txt and .md files, count the total number of lines
across all of them, and write the count to a file called linecount.txt
```

**Expected output**:
```bash
cat *.txt *.md | wc -l > linecount.txt
```
(or equivalently: `find . -name "*.txt" -o -name "*.md" | xargs wc -l | tail -1 | awk '{print $1}' > linecount.txt`)

**Verification**: Check that `linecount.txt` exists and contains the correct number.

## Difficulty Curriculum

| Level | Command Complexity | Example |
|-------|-------------------|---------|
| 1 | Single command, no pipes | `ls -la`, `mkdir foo`, `cp a.txt b.txt` |
| 2 | Single command with flags | `find . -name "*.py" -mtime -7` |
| 3 | Simple pipes (2 commands) | `grep "error" log.txt \| wc -l` |
| 4 | Multi-stage pipelines | `find . -name "*.log" \| xargs grep "ERR" \| sort \| uniq -c \| sort -rn` |
| 5 | Loops and conditionals | `for f in *.csv; do head -1 "$f" > "headers/$f"; done` |
| 6 | Complex text processing | `awk`, `sed` with regex, `jq` for JSON, process substitution |

## Limitations & Risks

- **Security**: Shell commands are inherently dangerous. Sandboxing must be watertight. Even within a sandbox, commands like fork bombs can exhaust resources. Use cgroups, PID limits, and memory limits.
- **Platform dependence**: GNU vs. BSD coreutils differ (e.g., `sed -i` syntax). The sandbox must define which platform is being used.
- **Multiple valid solutions**: Many tasks have dozens of correct command variations. Output-based verification handles this, but some tasks (e.g., "sort this file in place") require filesystem-state checking.
- **Interactive commands**: Some commands expect interactive input (e.g., `vim`, `less`). Tasks must be restricted to non-interactive commands or provide stdin.
- **Environment assumptions**: Tasks may assume specific tools are installed (e.g., `jq`, `ripgrep`). The sandbox image must be well-defined and documented.
- **Partial correctness**: A pipeline might get 90% of the task right but fail on one edge case. Binary reward doesn't capture this; partial credit for matching partial output is possible but complex.

## Connections

- **Regex Synthesis** is a building block: many shell tasks involve regex patterns for grep/sed/awk.
- **Infrastructure-as-Code** tasks often require shell commands for setup, deployment, and orchestration.
- **Build Configuration** tasks may include shell commands in Makefiles and CI scripts.
- **Web Scraping** can be done via shell tools (curl + grep/sed/jq pipelines).
- Shares the NL→formal-language structure with **SQL Generation** and **API Usage**.
