# Rust Workspace 项目模板设计

日期：2026-10-09

## 1. 目标

建设一套基于 `cargo-generate` 的 Rust workspace 项目模板，使新项目可以通过一条命令快速生成，并默认具备：

- Rust 2024 edition；
- workspace 形式的 `lib` 与 `bin` 双 crate 结构；
- GitHub Actions CI；
- 格式化、Clippy、测试、文档构建、依赖检查；
- lcov 覆盖率文件生成与构建产物保存；
- `justfile` 与 `pre-commit` 本地工作流；
- MIT OR Apache-2.0 双许可；
- 中英双语 README 和基础开发文档。

模板默认面向通用内部项目和开源项目，不绑定特定 Web 框架，不默认引入异步运行时。

## 2. 使用方式

模板发布后，新项目通过以下命令生成：

```bash
cargo install cargo-generate
cargo generate --git https://github.com/<owner>/<repo>.git rust-workspace --name my-app
```

其中 `rust-workspace` 是仓库内模板子目录。生成过程中使用 `cargo-generate` 的 Liquid 占位符能力处理项目名称、目录名称、包名、作者、描述和仓库信息。

初始变量约定：

- `project-name`：用户提供的 kebab-case 项目名；
- `crate_name`：由项目名推导的 snake_case 名称；
- `authors`：默认从本地 Cargo 或 Git 配置推导；
- `description`：项目描述；
- `github_repo`：GitHub 仓库的 `owner/repo`，允许留空。

## 3. 仓库结构

模板仓库采用“模板内容位于子目录”的结构，隔离模板仓库自身 CI 与生成项目 CI。

```text
rust-template/
├── .github/
│   └── workflows/
│       └── template-ci.yml
├── README.md
├── docs/
│   └── superpowers/
│       └── specs/
│           └── 2026-10-09-rust-workspace-template-design.md
└── rust-workspace/
    ├── cargo-generate.toml
    ├── Cargo.toml.liquid
    ├── README.md.liquid
    ├── LICENSE-MIT
    ├── LICENSE-APACHE
    ├── rust-toolchain.toml
    ├── rustfmt.toml
    ├── clippy.toml
    ├── deny.toml
    ├── .editorconfig
    ├── .gitignore
    ├── .pre-commit-config.yaml
    ├── justfile
    ├── .github/
    │   └── workflows/
    │       └── ci.yml
    ├── docs/
    │   ├── architecture.md
    │   └── development.md
    ├── scripts/
    │   └── coverage.sh
    ├── crates/
    │   ├── {{project-name}}-core/
    │   │   ├── Cargo.toml.liquid
    │   │   └── src/lib.rs
    │   └── {{project-name}}-cli/
    │       ├── Cargo.toml.liquid
    │       └── src/main.rs
    └── tests/
        └── smoke_test.rs
```

实现时以 `cargo-generate` 对模板子目录和 Liquid 文件后缀的实际规则为准；若当前版本要求固定模板配置文件名，则保留 `cargo-generate.toml`，并在其中声明模板元数据、提示变量和忽略规则。

## 4. 生成后的项目结构

```text
my-app/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── rust-toolchain.toml
├── rustfmt.toml
├── clippy.toml
├── deny.toml
├── .editorconfig
├── .gitignore
├── .pre-commit-config.yaml
├── justfile
├── .github/
│   └── workflows/
│       └── ci.yml
├── docs/
│   ├── architecture.md
│   └── development.md
├── scripts/
│   └── coverage.sh
├── crates/
│   ├── my-app-core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── my-app-cli/
│       ├── Cargo.toml
│       └── src/main.rs
└── tests/
    └── smoke_test.rs
```

### 4.1 `my-app-core`

`my-app-core` 提供可复用领域逻辑和库 API：

- 不依赖 CLI 层；
- 不直接读取环境变量、标准输入或命令行参数；
- 包含一个简单示例函数、单元测试和文档测试；
- 保持 API 小而清晰，方便新项目删除示例代码后继续使用。

### 4.2 `my-app-cli`

`my-app-cli` 只负责命令行入口：

- 使用 `clap` 的 derive API 解析参数；
- 使用 `tracing` 和 `tracing-subscriber` 初始化日志；
- 调用 `my-app-core` 暴露的函数；
- 使用 `thiserror` 定义示例错误；
- 在 `main` 中统一处理错误并返回合适的进程退出状态。

## 5. Workspace 与依赖策略

根 `Cargo.toml` 统一管理 workspace 成员、共享依赖、profile、package 元数据和 lints。

初始依赖：

```toml
clap = { version = "4", features = ["derive"] }
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

默认不引入 `tokio`。若生成项目未来发展为异步服务，可在具体项目中显式添加异步运行时。

## 6. 格式化与质量约束

### 6.1 `rustfmt.toml`

使用 stable 可用配置：

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

不使用 nightly-only 选项。

### 6.2 Workspace lints

根 workspace 配置：

```toml
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = "warn"
cargo = "warn"
pedantic = "warn"
```

各 crate 使用：

```toml
[lints]
workspace = true
```

CI 对 Clippy 和文档使用 `-D warnings`。

### 6.3 `cargo deny`

`deny.toml` 检查：

- 安全公告；
- 依赖来源；
- 许可证；
- 重复依赖；
- 被禁止的 crate。

许可证允许 `MIT` 与 `Apache-2.0`。

## 7. CI 设计

### 7.1 模板仓库 CI

模板仓库的 `.github/workflows/template-ci.yml` 执行：

1. checkout 模板仓库；
2. 安装 stable Rust；
3. 安装 `cargo-generate`；
4. 在临时目录基于 `rust-workspace` 生成示例项目；
5. 进入生成项目执行：
   - `cargo fmt --all -- --check`；
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`；
   - `cargo test --workspace --all-features`；
   - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`；
   - `cargo deny check`；
   - `scripts/coverage.sh` 生成 `lcov.info`；
6. 上传 `lcov.info` 为构建产物。

### 7.2 生成项目 CI

生成项目自带 `.github/workflows/ci.yml`，在 Ubuntu 和 stable Rust 上执行相同质量检查，并通过 `actions/upload-artifact` 保存覆盖率文件。

覆盖率只生成并保存为 artifact，不上传 Codecov 或其他第三方平台。

## 8. 本地开发工作流

### 8.1 `justfile`

提供以下命令：

```just
default        # 列出可用命令
fmt            # 格式化
check-fmt      # 检查格式
clippy         # 运行 Clippy
test           # 运行测试
doc            # 构建文档
deny           # 检查依赖
coverage       # 生成覆盖率
ci             # 串行执行本地 CI 检查
```

### 8.2 `pre-commit`

`.pre-commit-config.yaml` 在提交前运行：

- 格式检查；
- Clippy；
- 测试。

开发者通过以下命令启用：

```bash
pre-commit install
pre-commit run --all-files
```

## 9. 文档约定

### 9.1 README

生成项目的 README 使用中英双语，中文为主，英文摘要在后。结构为：

```markdown
# 项目名称
> 简短描述

## 功能特性
## 快速开始
### 环境要求
### 安装
### 运行
## 项目结构
## 开发命令
## 质量检查
## CI 与覆盖率
## 文档
## 贡献指南
## 许可证

---

# English
## Features
## Quick Start
## Project Structure
## Development
## License
```

### 9.2 `docs/architecture.md`

说明：

- workspace 分层；
- core 与 CLI 的职责边界；
- 新增 crate 的推荐位置；
- 依赖方向；
- 错误处理和日志约定。

### 9.3 `docs/development.md`

说明：

- 本地环境要求；
- 常用 `just` 命令；
- pre-commit 安装；
- 测试、文档和覆盖率生成方式。

模板仓库根 README 面向模板维护者和使用者，说明如何安装 `cargo-generate`、如何生成项目、模板包含哪些检查，以及模板仓库自身 CI 的验证方式。

## 10. 文件规模与复杂度约束

- 单个示例 Rust 文件保持短小；
- `lib.rs` 预计 20-60 行；
- `main.rs` 预计 60-100 行；
- 覆盖率命令放入独立脚本；
- 模板仓库 CI 与生成项目 CI 分为两个 workflow；
- 不把多种业务模板塞入当前模板；
- 如后续新增 server、database、queue 等能力，应作为独立模板或独立 crate 设计。

## 11. 验收标准

模板完成后，应满足：

1. 可以通过 `cargo generate` 从 `rust-workspace` 子目录生成新项目；
2. 生成后的 crate 名称、目录名称和包名与输入项目名一致；
3. 生成项目可以运行 `cargo fmt --all -- --check`；
4. 生成项目可以运行 `cargo clippy --workspace --all-targets --all-features -- -D warnings`；
5. 生成项目可以运行 `cargo test --workspace --all-features`；
6. 生成项目可以在 `-D warnings` 下构建文档；
7. `cargo deny check` 可以通过；
8. 可以生成 `lcov.info`；
9. 模板仓库 CI 能先生成项目再验证项目；
10. README 和开发文档清晰说明使用方式、目录约定和质量检查。
