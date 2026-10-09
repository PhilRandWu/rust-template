# Rust Workspace Template Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a `cargo-generate` template that creates a Rust 2024 workspace with a reusable core library, a synchronous CLI, quality gates, GitHub Actions CI, and bilingual documentation.

**Architecture:** The repository root contains template-repository documentation and CI. The actual generated project lives entirely in `template/`; template CI expands that directory into `.generated/sample-app` and validates the generated project.

**Tech Stack:** Rust 2024, Cargo workspaces, `cargo-generate`, GitHub Actions, Rustfmt, Clippy, `cargo-deny`, cargo-tarpaulin, `just`, and `pre-commit`.

---

## File Map

- `.github/workflows/template-ci.yml`: validates the template by generating a sample project.
- `README.md`: explains how to use and maintain the template repository.
- `template/cargo-generate.toml`: declares template metadata and project-specific prompts.
- `template/Cargo.toml.liquid`: workspace manifest template.
- `template/rust-toolchain.toml`: selects stable Rust with Rustfmt and Clippy.
- `template/rustfmt.toml` and `template/deny.toml`: formatting and dependency policies.
- `template/justfile` and `.pre-commit-config.yaml`: local developer workflow.
- `template/scripts/coverage.sh`: generates `lcov.info` with cargo-tarpaulin.
- `template/.github/workflows/ci.yml`: generated-project CI.
- `template/crates/{{project-name}}-core/`: core library template.
- `template/crates/{{project-name}}-cli/`: CLI template with its own smoke tests.
- `template/docs/`: architecture and development docs.
- `template/LICENSE-MIT` and `LICENSE-APACHE`: dual licenses.

Liquid naming rules:

- Use `{{crate_name}}` for Rust identifiers; use the kebab-case form for `CARGO_BIN_EXE_*`.
- Use `{{crate_name | replace: "_", "-"}}` in file contents when a kebab-case package name is required.
- Use `{{project-name}}` only in template directory names, where `cargo-generate` performs path substitution.

---

### Task 1: Repository Base

**Files:**
- Create: `.gitignore`
- Create: `README.md`

- [ ] **Step 1: Create root ignore rules**

```gitignore
# macOS
.DS_Store

# Editor files
*.swp
*.swo
*~

# Local generated test projects
.generated/
target/
```

- [ ] **Step 2: Create the template repository README**

```markdown
# Rust Workspace Template

一个基于 `cargo-generate` 的 Rust workspace 项目模板。

## 功能特性

- Rust 2024 stable
- workspace 结构
- `core` 库与 `cli` 二进制入口
- Rustfmt、Clippy、测试、文档和 `cargo-deny`
- GitHub Actions CI
- lcov 覆盖率产物
- `justfile` 与 `pre-commit`
- MIT OR Apache-2.0 双许可

## 使用模板

```bash
cargo install cargo-generate --locked
cargo generate --git https://github.com/PhilRandWu/rust-template.git template --name my-app
```

生成后进入项目目录并运行：

```bash
cd my-app
just ci
```

## 目录约定

- `.github/workflows/template-ci.yml`：模板仓库自身 CI；
- `template/`：生成项目的完整模板；
- `template/crates/{{project-name}}-core/`：可复用库代码；
- `template/crates/{{project-name}}-cli/`：命令行入口。

## 模板维护

模板仓库 CI 会在临时目录生成示例项目，并对生成结果运行格式、Clippy、测试、文档、依赖检查和覆盖率生成。

## License

Licensed under either of Apache License, Version 2.0, or the MIT license, at your option.
```

- [ ] **Step 3: Commit**

```bash
git add .gitignore README.md
git commit -m "chore: add template repository base"
```

---

### Task 2: Template Configuration and Workspace Settings

**Files:**
- Create: `template/cargo-generate.toml`
- Create: `template/Cargo.toml.liquid`
- Create: `template/rust-toolchain.toml`
- Create: `template/rustfmt.toml`
- Create: `template/deny.toml`
- Create: `template/.editorconfig`
- Create: `template/.gitignore`

- [ ] **Step 1: Create cargo-generate configuration**

```toml
[template]
cargo_generate_version = ">=0.23.7"

[placeholders.description]
type = "string"
prompt = "Project description"
default = "A Rust workspace project"

[placeholders.github_repo]
type = "string"
prompt = "GitHub repository as owner/repo, or leave empty"
default = ""
```

- [ ] **Step 2: Create the workspace manifest**

```liquid
{% assign package_name = crate_name | replace: "_", "-" %}
[workspace]
resolver = "2"
members = [
    "crates/{{package_name}}-core",
    "crates/{{package_name}}-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["{{authors}}"]
description = "{{description}}"
license = "MIT OR Apache-2.0"

{% if github_repo != "" %}repository = "https://github.com/{{github_repo}}"{% endif %}

[workspace.dependencies]
{{package_name}}-core = { path = "crates/{{package_name}}-core", version = "0.1.0" }
clap = { version = "4", features = ["derive"] }
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[profile.release]
lto = true
codegen-units = 1

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
cargo-common-metadata = "allow"
```

- [ ] **Step 3: Create toolchain, formatting, deny, editor, and ignore files**

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = []
```

`rustfmt.toml`:

```toml
edition = "2024"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
```

`deny.toml`:

```toml
[advisories]
version = 2

[licenses]
version = 2
allow = ["MIT", "Apache-2.0", "Unicode-3.0"]
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

`.editorconfig`:

```ini
root = true

[*]
charset = utf-8
end_of_line = lf
indent_style = space
indent_size = 4
insert_final_newline = true
trim_trailing_whitespace = true

[*.{yml,yaml,md}]
indent_size = 2
```

`.gitignore`:

```gitignore
/target
lcov.info
*.profraw
.DS_Store
```

- [ ] **Step 4: Commit**

```bash
git add template/cargo-generate.toml template/Cargo.toml.liquid \
    template/rust-toolchain.toml template/rustfmt.toml \
    template/deny.toml template/.editorconfig template/.gitignore
git commit -m "chore: add workspace settings"
```

---

### Task 3: Core Library

**Files:**
- Create: `template/crates/{{project-name}}-core/Cargo.toml.liquid`
- Create: `template/crates/{{project-name}}-core/src/lib.rs`

- [ ] **Step 1: Create the core manifest**

```liquid
{% assign package_name = crate_name | replace: "_", "-" %}
[package]
name = "{{package_name}}-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
description.workspace = true
license.workspace = true

[dependencies]
thiserror = { workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Create the core API**

```rust
//! Core library for the generated workspace.

use thiserror::Error;

/// Errors returned by core operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// Returned when the input is empty.
    #[error("input must not be empty")]
    EmptyInput,
}

/// Build a greeting for `name`.
///
/// # Errors
///
/// Returns [`CoreError::EmptyInput`] when `name` is empty.
///
/// # Examples
///
/// ```
/// use {{crate_name}}_core::greet;
/// assert_eq!(greet("Rust")?, "Hello, Rust!");
/// # Ok::<(), {{crate_name}}_core::CoreError>(())
/// ```
pub fn greet(name: &str) -> Result<String, CoreError> {
    if name.is_empty() {
        return Err(CoreError::EmptyInput);
    }

    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_greeting() {
        assert_eq!(greet("Rust").as_deref(), Ok("Hello, Rust!"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(greet(""), Err(CoreError::EmptyInput));
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add 'template/crates/{{project-name}}-core'
git commit -m "feat: add core library template"
```

---

### Task 4: CLI and Smoke Tests

**Files:**
- Create: `template/crates/{{project-name}}-cli/Cargo.toml.liquid`
- Create: `template/crates/{{project-name}}-cli/src/main.rs`
- Create: `template/crates/{{project-name}}-cli/tests/smoke_test.rs`

- [ ] **Step 1: Create the CLI manifest**

```liquid
{% assign package_name = crate_name | replace: "_", "-" %}
[package]
name = "{{package_name}}-cli"
version.workspace = true
edition.workspace = true
authors.workspace = true
description.workspace = true
license.workspace = true

[[bin]]
name = "{{package_name}}"
path = "src/main.rs"

[dependencies]
{{package_name}}-core = { workspace = true }
clap = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Create the CLI**

```rust
use std::process::ExitCode;

use clap::Parser;
use {{crate_name}}_core::{CoreError, greet};
use thiserror::Error;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

/// A Rust workspace starter CLI.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name to greet.
    #[arg(short, long, default_value = "world")]
    name: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let args = Args::parse();

    match run(&args.name) {
        Ok(message) => {
            info!("command completed");
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(name: &str) -> Result<String, AppError> {
    Ok(greet(name)?)
}
```

- [ ] **Step 3: Create smoke tests inside the CLI crate**

```rust
use std::process::Command;

#[test]
fn cli_prints_default_greeting() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin).output().expect("CLI binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn cli_rejects_empty_name() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin)
        .args(["--name", ""])
        .output()
        .expect("CLI binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("input must not be empty"));
}

#[test]
fn cli_accepts_name() {
    let bin = std::env::var("CARGO_BIN_EXE_{{crate_name | replace: "_", "-"}}")
        .expect("CLI binary path should be provided by Cargo");
    let output = Command::new(bin)
        .args(["--name", "Rust"])
        .output()
        .expect("CLI binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("Hello, Rust!"));
}
```

- [ ] **Step 4: Commit**

```bash
git add 'template/crates/{{project-name}}-cli'
git commit -m "feat: add cli template"
```

---

### Task 5: Local Workflows, Dependency Checks, and Coverage Script

**Files:**
- Create: `template/justfile`
- Create: `template/.pre-commit-config.yaml`
- Create: `template/scripts/coverage.sh`

- [ ] **Step 1: Create the justfile**

```just
default:
    @just --list

fmt:
    cargo fmt --all

check-fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-features

doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

deny:
    cargo deny check

coverage:
    ./scripts/coverage.sh

ci: check-fmt clippy test doc deny
```

- [ ] **Step 2: Create pre-commit configuration**

```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all -- --check
        language: system
        pass_filenames: false

      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --workspace --all-targets --all-features -- -D warnings
        language: system
        pass_filenames: false

      - id: cargo-test
        name: cargo test
        entry: cargo test --workspace --all-features
        language: system
        pass_filenames: false
```

- [ ] **Step 3: Create the coverage script**

```bash
#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo install cargo-tarpaulin --locked
fi

cargo tarpaulin \
    --workspace \
    --all-features \
    --out Lcov \
    --output-dir . \
    --skip-clean
```

Then run:

```bash
chmod +x template/scripts/coverage.sh
```

- [ ] **Step 4: Commit**

```bash
git add template/justfile template/.pre-commit-config.yaml \
    template/scripts/coverage.sh
git commit -m "chore: add local quality workflows"
```

---

### Task 6: CI Workflows

**Files:**
- Create: `template/.github/workflows/ci.yml`
- Create: `.github/workflows/template-ci.yml`

- [ ] **Step 1: Create generated-project CI**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always

jobs:
  quality:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v5

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run Clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings

      - name: Run tests
        run: cargo test --workspace --all-features

      - name: Build docs
        run: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

      - name: Install cargo-deny
        uses: taiki-e/install-action@v2
        with:
          tool: cargo-deny

      - name: Check dependencies
        run: cargo deny check

      - name: Install cargo-tarpaulin
        uses: taiki-e/install-action@v2
        with:
          tool: cargo-tarpaulin

      - name: Generate coverage
        run: ./scripts/coverage.sh

      - name: Upload coverage
        uses: actions/upload-artifact@v5
        with:
          name: lcov
          path: lcov.info
```

- [ ] **Step 2: Create template-repository CI**

```yaml
name: Template CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always

jobs:
  template:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v5

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Install template tools
        uses: taiki-e/install-action@v2
        with:
          tool: cargo-generate,cargo-deny,cargo-tarpaulin

      - name: Generate sample project
        run: |
          mkdir -p .generated
          cargo generate \
            --path template \
            --name sample-app \
            --destination .generated \
            --define description="Sample application" \
            --define github_repo= \
            --define authors="Template Authors" \
            --vcs none

      - name: Check formatting
        working-directory: .generated/sample-app
        run: cargo fmt --all -- --check

      - name: Run Clippy
        working-directory: .generated/sample-app
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings

      - name: Run tests
        working-directory: .generated/sample-app
        run: cargo test --workspace --all-features

      - name: Build docs
        working-directory: .generated/sample-app
        run: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

      - name: Check dependencies
        working-directory: .generated/sample-app
        run: cargo deny check

      - name: Generate coverage
        working-directory: .generated/sample-app
        run: ./scripts/coverage.sh

      - name: Upload coverage
        uses: actions/upload-artifact@v5
        with:
          name: template-lcov
          path: .generated/sample-app/lcov.info
```

- [ ] **Step 3: Commit**

```bash
git add template/.github/workflows/ci.yml .github/workflows/template-ci.yml
git commit -m "ci: add template and generated project pipelines"
```

---

### Task 7: Documentation and Licenses

**Files:**
- Create: `template/README.md.liquid`
- Create: `template/docs/architecture.md`
- Create: `template/docs/development.md`
- Create: `template/LICENSE-MIT`
- Create: `template/LICENSE-APACHE`

- [ ] **Step 1: Create the bilingual README**

```liquid
{% assign package_name = crate_name | replace: "_", "-" %}
# {{package_name}}

> {{description}}

## 功能特性

- Rust 2024 workspace
- 可复用核心库
- 命令行入口
- 格式化、Clippy、测试、文档和依赖检查
- GitHub Actions CI 和 lcov 覆盖率产物

## 快速开始

### 环境要求

- Rust stable

### 运行

```bash
cargo run -p {{package_name}}-cli -- --name Rust
```

预期输出：

```text
Hello, Rust!
```

## 项目结构

```text
crates/
├── {{package_name}}-core/  # 可复用领域逻辑
└── {{package_name}}-cli/   # 命令行入口
docs/                      # 架构和开发文档
```

## 开发命令

```bash
just fmt
just clippy
just test
just doc
just deny
just coverage
just ci
```

## CI 与覆盖率

CI 会检查格式、Clippy、测试、文档和依赖，并生成 `lcov.info` 构建产物。覆盖率不会上传到第三方服务。

## 文档

- [架构说明](docs/architecture.md)
- [开发指南](docs/development.md)

## 许可证

本项目采用 MIT OR Apache-2.0 双许可。

---

# English

## Features

- Rust 2024 workspace
- Reusable core library
- Command-line entry point
- Formatting, linting, testing, docs, and dependency checks
- GitHub Actions CI and an lcov coverage artifact

## Quick Start

```bash
cargo run -p {{package_name}}-cli -- --name Rust
```

## Development

Run `just ci` to execute all local quality checks.

## License

Licensed under either of the Apache License, Version 2.0, or the MIT license, at your option.
```

- [ ] **Step 2: Create architecture documentation**

```liquid
{% assign package_name = crate_name | replace: "_", "-" %}
# 架构说明

## Workspace 分层

- `{{package_name}}-core`：可复用领域逻辑和库 API；
- `{{package_name}}-cli`：命令行参数解析、日志初始化和进程退出处理。

## 依赖方向

`{{package_name}}-cli` 可以依赖 `{{package_name}}-core`，但 core 不允许依赖 CLI。

core 不直接读取命令行参数、环境变量、标准输入或标准输出。

## 新增 crate

新增库优先放入 `crates/`，并在根 `Cargo.toml` 的 `members` 中注册。

## 错误处理和日志

可复用错误定义在 core 中，入口层负责展示错误和决定退出码。业务库默认不写日志。
```

- [ ] **Step 3: Create development documentation**

```markdown
# 开发指南

## 环境要求

- Rust stable
- `pre-commit`，可选但推荐
- `just`，可选但推荐

## 初始化

```bash
pre-commit install
```

## 常用命令

| 命令 | 说明 |
| --- | --- |
| `just fmt` | 格式化代码 |
| `just check-fmt` | 检查格式 |
| `just clippy` | 运行 Clippy |
| `just test` | 运行全部测试 |
| `just doc` | 构建文档 |
| `just deny` | 检查依赖和许可证 |
| `just coverage` | 生成 lcov 覆盖率 |
| `just ci` | 运行本地完整检查 |

`just coverage` 会在项目根目录生成 `lcov.info`，该文件已被 `.gitignore` 忽略。
```

- [ ] **Step 4: Add licenses**

Create `LICENSE-MIT` with the complete standard MIT text and this copyright line:

```text
Copyright (c) 2026 The Project Authors
```

Create `LICENSE-APACHE` by downloading the exact Apache License 2.0 text:

```bash
curl -L https://www.apache.org/licenses/LICENSE-2.0.txt -o template/LICENSE-APACHE
```

- [ ] **Step 5: Commit**

```bash
git add template/README.md.liquid template/docs/ \
    template/LICENSE-MIT template/LICENSE-APACHE
git commit -m "docs: add generated project documentation"
```

---

### Task 8: End-to-End Verification

**Files:** No planned source changes; only modify template files if verification finds a defect.

- [ ] **Step 1: Verify tools**

```bash
rustc --version
cargo --version
cargo generate --version
```

- [ ] **Step 2: Generate the sample project locally**

```bash
rm -rf .generated/sample-app
mkdir -p .generated
cargo generate --path template --name sample-app --destination .generated \
  --define description="Sample application" --define github_repo= \
  --define authors="Template Authors" --vcs none
```

- [ ] **Step 3: Run generated-project checks**

```bash
cd .generated/sample-app
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
./scripts/coverage.sh
test -f lcov.info
```

- [ ] **Step 4: Fix defects only in the source template**

If a check fails, edit the corresponding file under `template/`, regenerate the sample project, and rerun the failed check. Do not edit files under `.generated/`.

- [ ] **Step 5: Commit fixes if needed**

```bash
git status --short
```

If template files changed, commit them with:

```bash
git add <changed-template-files>
git commit -m "fix: correct generated template"
```

Do not create an empty commit when verification passes without changes.

---

## Self-Review

- Every requirement from the approved design is covered by Tasks 1-8.
- No unresolved placeholder function calls remain in final code.
- Smoke tests live in the CLI crate because a virtual workspace root has no package of its own.
- No invalid `rust-version = "stable"` field is used.
- CI supplies all custom placeholder values and runs without interactive prompts.


## Final Implementation Notes

- The installed local `cargo-generate` version is 0.23.7.
- The smoke tests read `CARGO_BIN_EXE_<kebab-case-binary-name>` at runtime.
- CLI diagnostic logs are explicitly written to stderr; successful command output remains on stdout.
- `cargo-common-metadata` is allowed because the repository URL is an optional placeholder.
- `Unicode-3.0` is allowed for the transitive `unicode-ident` dependency; generated projects remain MIT OR Apache-2.0.
- End-to-end verification reached 100% line coverage and produced a non-empty `lcov.info`.
