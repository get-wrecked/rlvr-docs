---
domain: file-system-tasks
category: agent/system
verification_type: constraint | diff
dataset_scale: unlimited (procedurally generated from templates)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# File System Tasks

## Overview

File system tasks require an agent to organize, manipulate, and manage files and directories: creating, moving, renaming, deleting, searching, sorting, converting, deduplicating, and applying bulk transformations. Verification is clean and deterministic: compare the resulting filesystem state to a specification. This domain is practically important (file management is a daily computer task), infinitely scalable via procedural generation, and provides a natural progression from trivial to complex. It tests procedural execution, pattern matching, conditional logic, and the ability to translate natural-language specifications into precise operations.

## Verification Mechanism

1. **Filesystem state comparison** (constraint-based): After the agent completes its operations, check the filesystem against a specification:
   - **File existence**: Required files/directories exist at specified paths. `os.path.exists()`.
   - **File absence**: Deleted/moved files no longer exist at original paths.
   - **File content**: File contents match expected content. MD5/SHA256 hash comparison, or full content diff.
   - **Directory structure**: Directory tree matches expected structure. Recursive comparison.
   - **File permissions**: Files have correct permissions (read/write/execute). `os.stat()`.
   - **File metadata**: Modification times, sizes, or other metadata match specifications.
2. **Content verification** (diff-based): For file transformation tasks (convert CSV to JSON, merge files), compare output file content to reference. Exact byte comparison, or structured comparison for known formats.
3. **Structural verification** (constraint-based): For organizational tasks (sort files into folders by type/date/size), verify the organizational logic was applied correctly:
   - All .pdf files are in the "documents" folder.
   - All files modified before 2024 are in the "archive" folder.
   - No files are in the wrong folder.
   - No files are missing.
4. **Count/property verification** (constraint-based): For tasks like "remove all duplicate files" or "find all files larger than 10MB", verify the resulting state satisfies the constraint:
   - No two files have identical content (for dedup).
   - No remaining file exceeds 10MB (for filtering).

## Dataset Sources

- **Procedural generation (primary)**: File system tasks are ideal for unlimited procedural generation:
  - Generate a random filesystem tree (N directories, M files with random names, sizes, types, dates, content).
  - Define a task (organize by type, rename by pattern, delete duplicates, etc.).
  - Compute the expected result programmatically.
  - Create the task instruction in natural language.
  - Difficulty controlled by: number of files (10 to 10,000), number of operations, constraint complexity, edge cases.
- **OSWorld (file management subset)**: https://os-world.github.io/ — Desktop file management tasks with verification scripts.
- **SWE-bench (file creation subset)**: Tasks that involve creating/modifying files in a codebase.
- **Bash/Shell exercise datasets**: Exercises from Unix/Linux courses that involve file manipulation.
  - HackerRank Shell challenges: https://www.hackerrank.com/domains/shell
  - ExplainShell: Shell command exercises with expected outputs.
- **FileOrganizer templates**: Various "clean up your downloads folder" task templates that can be parameterized.
- **Data cleaning pipelines**: Tasks from data engineering (parse log files, split CSVs, merge datasets) with verifiable output files.

## Task Format

**File organization**:
- Input: Description of a directory containing files + organizational instruction (e.g., "Sort all files into subdirectories by file extension. Create a 'documents' folder for .pdf and .docx, 'images' for .jpg and .png, and 'other' for everything else.").
- Output: Shell commands or file operations to execute.
- Verification: Check that every file is in the correct subdirectory and no files are missing.

**Bulk rename**:
- Input: Directory listing + rename pattern (e.g., "Rename all files to format: YYYY-MM-DD_originalname.ext using the file's modification date.").
- Output: Rename commands.
- Verification: All files have new names matching the pattern with correct dates.

**File search and extraction**:
- Input: Directory tree + search criteria (e.g., "Find all Python files containing the word 'TODO' and copy them to a 'review' folder.").
- Output: Commands to search and copy.
- Verification: 'review' folder contains exactly the matching files.

**Content transformation**:
- Input: Input file(s) + transformation (e.g., "Convert data.csv to JSON format, with each row as an object.").
- Output: Transformation commands.
- Verification: Output file content matches reference.

**Deduplication**:
- Input: Directory with duplicate files (same content, possibly different names).
- Output: Commands to remove duplicates, keeping one copy.
- Verification: No two remaining files have identical content. All unique content is preserved.

**Cleanup**:
- Input: Directory + cleanup rules (e.g., "Delete all .tmp files, archive all files older than 1 year to archive.tar.gz, and organize remaining by type.").
- Output: Commands.
- Verification: No .tmp files exist, archive contains correct files, remaining files organized.

## Difficulty Curriculum

1. **Single file operations** (trivial): Create, copy, move, or delete one file.
2. **Simple organization** (easy): Move 5-10 files into 2-3 folders by extension.
3. **Pattern-based rename** (easy-medium): Rename files using a simple pattern. 10-20 files.
4. **Conditional operations** (medium): Operations with conditions (if file > 10MB, move to large/; if older than 2024, archive).
5. **Recursive operations** (medium): Apply operations to nested directory trees.
6. **Content-based operations** (medium-hard): Search/filter/sort by file content, not just metadata.
7. **Deduplication** (hard): Find and remove duplicates across a complex directory tree. Handle same-content-different-name and different-content-same-name.
8. **Complex multi-step workflows** (hard): Chain multiple operations with dependencies (extract, transform, organize, verify).
9. **Scripting** (very hard): Write a shell script that performs a complex file management task robustly (handle edge cases: spaces in filenames, symbolic links, special characters).
10. **Large-scale file management** (very hard): Manage thousands of files across complex directory structures with multiple interacting constraints.

## Limitations & Risks

- **Destructive operations**: File deletion and overwriting are irreversible. If the agent makes a mistake, data is lost. Training must use sandboxed filesystems.
- **Edge cases abound**: Filenames with spaces, special characters, Unicode, very long paths, symbolic links, hidden files (.dotfiles), empty files, zero-byte files, and permission-restricted files all create edge cases that are easy to miss in verification.
- **Platform differences**: File path separators (/ vs \), case sensitivity (Linux vs macOS/Windows), permission models, and available commands differ across OS platforms.
- **Natural language ambiguity**: "Organize these files" is underspecified. "Old files" needs a date threshold. Tasks must be specific enough to have a unique correct answer, which limits naturalness.
- **Verification completeness**: State comparison might miss side effects (wrong permissions set, files corrupted during copy, metadata lost). Comprehensive verification requires checking many properties.
- **Simplistic tasks for LLMs**: Basic file operations map directly to shell commands. An LLM that can generate bash commands can solve most file tasks without deep reasoning. The RLVR value is in complex, multi-step tasks.
- **Security concerns**: File system access is inherently privileged. An agent trained on file system tasks could learn patterns that are dangerous outside a sandbox (rm -rf, chmod 777, etc.).

## Connections

- [[computer-use]] — File management is a core component of computer use
- [[gui-navigation]] — File manager app interaction is a GUI navigation task
- [[competitive-programming-interactive]] — Both involve code execution with state verification
- [[spreadsheet-tasks]] — Both produce digital artifacts verified by state comparison
- [[email-tasks]] — File attachments and file organization support email workflows
