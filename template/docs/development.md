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
