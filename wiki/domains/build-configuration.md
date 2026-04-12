---
domain: Build Configuration
category: Code & Software
verification_type: execution-based (build succeeds + tests pass + artifact validation)
dataset_scale: 500–20K build tasks
difficulty_range: single-file compilation → multi-target cross-platform builds with dependencies
modality: text-to-config (CMake, Makefile, Cargo.toml, package.json, Gradle, Bazel)
status: early-stage
---

## Overview

Build configuration generation tasks the model with producing build system files — CMakeLists.txt, Makefiles, Cargo.toml, package.json with scripts, build.gradle, BUILD (Bazel), meson.build — that correctly compile source code, link dependencies, and produce working artifacts. Verification is execution-based: run the build, check that it succeeds, and run the project's tests to confirm the build is correct.

This domain is a viable RLVR target because (a) build success is a clear binary signal, (b) test passage confirms the build is functionally correct, (c) the task is practically valuable — build configuration is notoriously finicky and a common source of developer frustration.

## Verification Mechanism

```
def verify_build_config(generated_config, source_files, test_suite,
                        build_system, expected_artifacts):
    # 1. Set up project directory with source files and generated config
    project_dir = create_temp_dir()
    for path, content in source_files.items():
        write_file(project_dir / path, content)
    write_file(project_dir / config_filename(build_system), generated_config)

    # 2. Run the build
    if build_system == "cmake":
        build_dir = project_dir / "build"
        mkdir(build_dir)
        configure = run_in_sandbox(
            command="cmake ..",
            cwd=build_dir,
            timeout=120
        )
        if configure.returncode != 0:
            return 0.0  # CMake configuration failed

        build = run_in_sandbox(
            command="cmake --build . --parallel",
            cwd=build_dir,
            timeout=300
        )

    elif build_system == "make":
        build = run_in_sandbox(
            command="make -j$(nproc)",
            cwd=project_dir,
            timeout=300
        )

    elif build_system == "cargo":
        build = run_in_sandbox(
            command="cargo build",
            cwd=project_dir,
            timeout=300
        )

    elif build_system == "npm":
        install = run_in_sandbox(
            command="npm install",
            cwd=project_dir,
            timeout=120
        )
        if install.returncode != 0:
            return 0.0
        build = run_in_sandbox(
            command="npm run build",
            cwd=project_dir,
            timeout=300
        )

    elif build_system == "gradle":
        build = run_in_sandbox(
            command="./gradlew build",
            cwd=project_dir,
            timeout=600
        )

    elif build_system == "bazel":
        build = run_in_sandbox(
            command="bazel build //...",
            cwd=project_dir,
            timeout=600
        )

    if build.returncode != 0:
        return 0.0  # build failed

    # 3. Check expected artifacts exist
    for artifact in expected_artifacts:
        artifact_path = find_artifact(project_dir, artifact.name)
        if artifact_path is None:
            return 0.0  # expected artifact not produced

        # Check artifact type
        if artifact.type == "executable":
            if not is_executable(artifact_path):
                return 0.0
        elif artifact.type == "library":
            if not is_library(artifact_path):  # .so, .a, .dylib
                return 0.0

    # 4. Run tests
    if test_suite:
        if build_system == "cmake":
            test_result = run_in_sandbox(
                command="ctest --output-on-failure",
                cwd=build_dir,
                timeout=120
            )
        elif build_system == "cargo":
            test_result = run_in_sandbox(
                command="cargo test",
                cwd=project_dir,
                timeout=120
            )
        elif build_system == "npm":
            test_result = run_in_sandbox(
                command="npm test",
                cwd=project_dir,
                timeout=120
            )
        elif build_system == "make":
            test_result = run_in_sandbox(
                command="make test",
                cwd=project_dir,
                timeout=120
            )
        else:
            test_result = run_tests_generic(project_dir, test_suite)

        if test_result.returncode != 0:
            return 0.0  # tests failed

    return 1.0  # build succeeded and all tests pass
```

Additional checks:

```
def verify_build_properties(project_dir, build_system, requirements):
    """Check non-functional build properties."""
    checks = []

    # Dependency resolution
    if requirements.get("dependencies"):
        for dep in requirements["dependencies"]:
            checks.append(dependency_resolved(project_dir, dep))

    # Compiler flags
    if requirements.get("optimization_level"):
        checks.append(
            check_compiler_flags(project_dir, build_system,
                                 f"-O{requirements['optimization_level']}")
        )

    # Target platforms
    if requirements.get("cross_compile"):
        for target in requirements["cross_compile"]:
            cross_build = run_in_sandbox(
                command=cross_compile_command(build_system, target),
                cwd=project_dir,
                timeout=300
            )
            checks.append(cross_build.returncode == 0)

    return all(checks)
```

Key considerations:

- **Build system diversity**: CMake, Make, Cargo, npm, Gradle, Bazel, Meson — each has its own syntax, semantics, and ecosystem. The verification environment must have the relevant toolchains installed.
- **Dependency availability**: Builds that depend on external packages require those packages to be available. Use Docker images with pre-installed common dependencies, or offline package caches.
- **Build time**: Compilation can be slow, especially for C++ projects. Timeout limits must be generous enough for legitimate builds but catch infinite loops.
- **Reproducibility**: Builds should be deterministic. Pin dependency versions, disable network access during builds (use cached/vendored dependencies).
- **Platform specificity**: A Makefile that works on Linux may fail on macOS. The sandbox platform must be specified.

## Dataset Sources

| Dataset | Size | Build System | Notes |
|---------|------|-------------|-------|
| **GitHub CMake projects** | 100K+ repos | CMake | Mine CMakeLists.txt files from C/C++ repos |
| **GitHub Makefiles** | 200K+ repos | Make | Extremely common across all languages |
| **crates.io** | 140K+ crates | Cargo | Rust package registry with Cargo.toml files |
| **npm registry** | 2M+ packages | npm | package.json files with build scripts |
| **Maven Central** | 500K+ artifacts | Maven/Gradle | Java build configurations |
| **Bazel examples** | 1K+ | Bazel | Google's build system examples |
| **Meson WrapDB** | 200+ | Meson | Meson build definitions for common libraries |
| **Buildifier corpus** | Varies | Bazel | Formatted BUILD files from Google-style repos |

**Synthetic data generation**:
1. Take source files from known-good projects.
2. Remove the build configuration.
3. Generate a task: "Write a CMakeLists.txt that builds these source files into a library and executable, links against libpthread, and enables C++17."
4. Verification: the build succeeds and tests pass.
5. Vary by adding/removing dependencies, changing compiler flags, targeting different platforms.

## Task Format

**Input prompt**:
```
Write a CMakeLists.txt for a C++ project with the following structure:

  src/
    main.cpp          (uses functions from lib.h)
    lib.h             (header for the library)
    lib.cpp           (library implementation)
  tests/
    test_lib.cpp      (uses Google Test)

Requirements:
- Minimum CMake version 3.14
- C++17 standard required
- Build a static library 'mylib' from lib.cpp
- Build an executable 'app' from main.cpp, linked to mylib
- Build test executable using Google Test (fetch via FetchContent)
- Enable CTest
```

**Expected output**: A complete CMakeLists.txt file.

**Verification**: `cmake ..`, `cmake --build .`, `ctest` all succeed.

## Difficulty Curriculum

| Level | Complexity | Example |
|-------|-----------|---------|
| 1 | Single-file compilation | `gcc -o main main.c` equivalent in CMake |
| 2 | Library + executable | Static lib + executable linking against it |
| 3 | External dependencies | FetchContent/find_package for Google Test, Boost, etc. |
| 4 | Multi-target builds | Library, multiple executables, install targets |
| 5 | Cross-compilation | ARM target from x86, Android NDK builds |
| 6 | Complex build systems | Monorepo with Bazel, custom rules, code generation steps |

## Limitations & Risks

- **Toolchain availability**: Verification requires compilers, linkers, and build tools to be installed. Docker images with pre-installed toolchains are essential.
- **External dependency fetching**: Builds that download dependencies at build time require network access or pre-cached packages. This complicates sandboxing.
- **Build time**: Large projects can take minutes to build. This makes RL training expensive. Start with small projects (< 10 source files).
- **Platform specifics**: Build configs are often platform-specific. A CMakeLists.txt that works on Linux may fail on Windows. The sandbox OS must be consistent.
- **Version sensitivity**: Build tools and dependencies change. A working configuration may break with newer compiler/library versions. Pin all versions in the sandbox.
- **Limited datasets**: Build configuration datasets are less curated than code generation benchmarks. Most training data must be mined from GitHub.

## Connections

- **Infrastructure-as-Code** is the deployment counterpart: build configs build software; IaC deploys it.
- **Code Generation** produces the source code that build configurations compile.
- **Shell Commands** are the imperative equivalent: running compiler commands directly vs. declarative build configs.
- **Compiler Tasks** share the compilation infrastructure: build configs invoke compilers.
- **Code Repair** applies: fixing broken build configurations is a common developer task.
- **Test Generation** produces the tests that build configurations wire up for `make test` / `ctest`.
