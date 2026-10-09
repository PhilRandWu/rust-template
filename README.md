# Rust Workspace Template

一个基于 `cargo-generate` 的 Rust workspace 项目模板，用于快速创建具备基础工程规范的新项目。

## 功能特性

- Rust 2024 stable
- workspace 多 crate 结构
- `core` 库与 `cli` 二进制入口
- Rustfmt、Clippy、测试、文档和 `cargo-deny`
- GitHub Actions CI
- lcov 覆盖率构建产物
- `justfile` 与 `pre-commit`
- MIT OR Apache-2.0 双许可
- 中英双语项目说明

## 使用模板

```bash
cargo install cargo-generate --locked
cargo generate --path template --name my-app
```

从 Git 仓库使用时：

```bash
cargo generate --git https://github.com/PhilRandWu/rust-template.git template --name my-app
```

生成后运行：

```bash
cd my-app
just ci
```

## 目录约定

- `template/`：生成项目的完整模板；
- `template/crates/{{project-name}}-core/`：可复用领域库；
- `template/crates/{{project-name}}-cli/`：命令行入口；
- `.github/workflows/template-ci.yml`：模板仓库自身 CI；
- `docs/`：模板设计和实施计划。

## 本地验证

```bash
cargo generate --path template --name sample-app --destination .generated \
  --define description="Sample application" \
  --define github_repo= \
  --define authors="Template Authors" \
  --vcs none
```

## License

Licensed under either of Apache License, Version 2.0, or the MIT license, at your option.
