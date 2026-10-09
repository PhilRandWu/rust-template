# 架构说明

## Workspace 分层

- Core crate：可复用领域逻辑和库 API；
- CLI crate：命令行参数解析、日志初始化和进程退出处理。

## 依赖方向

CLI 可以依赖 Core，但 Core 不允许依赖 CLI。

Core 不直接读取命令行参数、环境变量、标准输入或标准输出。

## 新增 crate

新增库优先放入 `crates/`，并在根 `Cargo.toml` 的 `members` 中注册。

## 错误处理和日志

可复用错误定义在 Core 中，入口层负责展示错误和决定退出码。业务库默认不写日志。
