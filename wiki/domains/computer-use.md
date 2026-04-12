---
domain: computer-use
category: agent/desktop
verification_type: constraint | diff | outcome
dataset_scale: >1K tasks (benchmarks); procedurally generatable
difficulty_range: medium/hard/superhuman
modality: multimodal
status: strong_hypothesis
---

# Computer Use

## Overview

Computer use tasks require an agent to operate a full desktop environment — managing files, using applications (text editors, terminals, browsers, office suites), adjusting system settings, and orchestrating multi-app workflows. This is among the most ambitious RLVR agent domains: it targets general-purpose digital autonomy. Verification is achieved by checking the final system state (files created/modified, application states, configuration values) against a specification. The domain is harder to verify than web navigation because the state space is vastly larger and less structured.

## Verification Mechanism

1. **Filesystem state checking** (constraint-based): Verify that specific files exist, have correct content, correct permissions, are in the correct directories. Implementation: scripted assertions on the filesystem. E.g., `assert os.path.exists("/home/user/report.pdf")`, `assert "Q3 Revenue" in open("report.txt").read()`.
2. **Application state checking** (constraint-based): Verify that applications are in the correct state. E.g., a text editor has specific content, a spreadsheet has correct formulas, a terminal shows expected output. Implementation: accessibility APIs, window inspection tools, app-specific queries.
3. **Configuration state checking** (constraint-based): Verify system or application settings. E.g., display resolution changed, default browser set, WiFi connected to specific network. Implementation: system API queries.
4. **Screenshot comparison** (diff-based): Compare final desktop/application screenshot to a reference. Useful for visual tasks (set wallpaper, arrange windows). Metrics: pixel similarity, SSIM, or perceptual hashing.
5. **Output validation** (exact match / diff): For tasks producing concrete output (generate a chart, compile code, create a presentation), compare output to reference. Content comparison, structural comparison, or visual comparison depending on output type.
6. **Process/command verification** (outcome-based): Verify that certain processes ran, certain commands were executed, or certain results were achieved. E.g., "Install package X" verified by checking the package is importable.

## Dataset Sources

- **OSWorld**: https://os-world.github.io/ — 369 tasks across Ubuntu, Windows, and macOS environments. Tasks span file management, web browsing, office work, coding, and system administration. Each task has a setup script, instruction, and verification script. State-of-the-art benchmark.
- **WindowsAgentArena**: https://microsoft.github.io/WindowsAgentArena/ — Windows-specific agent benchmark. 154 tasks covering Windows applications and settings.
- **GAIA**: https://huggingface.co/gaia-benchmark — General AI Assistant benchmark requiring tool use and computer interaction. 466 questions at 3 difficulty levels.
- **AgentBench**: https://llmbench.ai/agent — Multi-environment benchmark including OS interaction tasks.
- **SWE-bench**: https://swe-bench.com/ — Software engineering tasks (resolve GitHub issues). Verification: test suite passes. 2,294 real-world issues.
- **OmniACT**: Desktop-based action completion tasks with screen annotations.
- **ScreenAgent**: Training data for GUI-based computer interaction.
- **AssistGUI**: https://showlab.github.io/assistgui/ — Desktop GUI task completion dataset.

## Task Format

**Desktop task completion**:
- Input: Screenshot and/or accessibility tree of current desktop state + task instruction (e.g., "Create a new folder called 'Q3_Reports' on the Desktop and move all PDF files from Downloads into it").
- Output: Sequence of actions: `click(x, y)`, `type("text")`, `hotkey("ctrl+c")`, `scroll(direction)`, `wait(seconds)`.
- Verification: Run verification script checking filesystem/application state.

**Multi-application workflow**:
- Input: Task requiring multiple applications (e.g., "Look up today's weather, then create a calendar event for 'Outdoor meeting' if it's sunny, otherwise 'Indoor meeting'").
- Output: Action sequence across multiple apps.
- Verification: Check final state of all relevant applications.

**System administration**:
- Input: "Install Python 3.11 and set up a virtual environment."
- Output: Terminal commands and GUI actions.
- Verification: `python3.11 --version` succeeds, venv directory exists.

## Difficulty Curriculum

1. **Single-app, single-action** (easy): Open an application, click a menu item, change a setting.
2. **File management** (easy-medium): Create, move, rename, delete files and folders.
3. **Text editing** (medium): Open a file, find and replace text, save.
4. **Multi-step single-app** (medium): Create a document with specific formatting, build a spreadsheet with formulas.
5. **Terminal/command-line tasks** (medium-hard): Run commands, pipe output, edit config files.
6. **Cross-application workflows** (hard): Copy data from a browser into a spreadsheet, attach a file to an email.
7. **System configuration** (hard): Network settings, user management, software installation.
8. **Complex debugging** (very hard): Diagnose and fix system issues. SWE-bench style.
9. **Open-ended creative tasks** (superhuman): "Set up a development environment for project X" with minimal specification.

## Limitations & Risks

- **Verification is expensive and fragile**: Each task requires a custom verification script. These scripts are brittle — they may check for exact file content that could be equivalently correct in other forms. Writing good verifiers is a significant engineering challenge.
- **Environment reproducibility**: Desktop environments have enormous state. Ensuring consistent starting states requires VM snapshots or containers, adding infrastructure complexity and latency.
- **Safety**: An agent with full computer access can cause real damage — deleting files, sending emails, modifying settings, running malicious code. Sandboxing is absolutely essential. Even in sandboxed environments, the agent might learn unsafe patterns.
- **Observation complexity**: Desktop screenshots are high-resolution with dense information. Accessibility trees are large and noisy. Processing these inputs is computationally expensive.
- **Platform diversity**: Tasks differ across Linux, macOS, and Windows. An agent trained on one OS may not transfer to another. Verification scripts are OS-specific.
- **Slow feedback loop**: Desktop interactions involve GUI rendering, application startup, and file I/O. RL training requires many episodes, and slow episodes are a bottleneck.
- **Limited benchmark scale**: Current benchmarks have hundreds, not millions, of tasks. Procedural generation of desktop tasks is possible but challenging to make realistic.

## Connections

- [[web-navigation]] — Web navigation is a major component of computer use
- [[gui-navigation]] — Mobile GUI navigation is the mobile counterpart
- [[file-system-tasks]] — File management is a core subtask of computer use
- [[spreadsheet-tasks]] — Spreadsheet work is a common computer use task
- [[email-tasks]] — Email is a key computer use workflow
