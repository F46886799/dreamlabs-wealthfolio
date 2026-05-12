# 为 Wealthfolio 做贡献

感谢您对为 Wealthfolio 做贡献的兴趣！我们欢迎来自社区的贡献。

## 贡献者许可协议（CLA）

我们要求所有贡献都需要 CLA。

通过提交拉取请求，您确认您已阅读并同意 [CLA.md](CLA.md)。未签署 CLA 协议的 PR 将不会被合并。

## 如何贡献

### 报告错误

1. 检查该错误是否已在 [Issues](https://github.com/wealthfolio/wealthfolio/issues) 中报告
2. 如果没有，请创建一个新问题，包含：
   - 清晰、描述性的标题
   - 重现步骤
   - 预期行为与实际行为
   - 您的环境（操作系统、版本等）

### 建议功能

1. 检查现有问题和讨论中是否有类似建议
2. 打开一个新问题，描述：
   - 您试图解决的问题
   - 您提出的解决方案
   - 您考虑过的任何替代方案

### 提交代码

1. Fork 存储库
2. 创建一个功能分支（`git checkout -b feature/amazing-feature`）
3. 进行更改
4. 确保测试通过并且代码遵循项目风格
5. 使用清晰、描述性的消息提交更改
6. 推送到您的 fork（`git push origin feature/amazing-feature`）
7. 打开一个拉取请求

### 开发设置

详细的设置说明请参见 [README](README.md#getting-started)。

### 可选的 Git 钩子

安装跟踪的 Git 钩子以在 `git push` 之前运行 CI 等效检查：

```bash
pnpm hooks:install
```

Git 不会自动启用存储库钩子，因此每个贡献者必须在每次克隆时选择加入一次。

### 代码风格

- **Rust**：遵循标准的 Rust 约定，使用 `cargo fmt` 和 `cargo clippy`
- **TypeScript/React**：遵循现有代码风格，已配置 ESLint 和 Prettier
- **提交**：编写清晰、简洁的提交消息

## 行为准则

请在所有互动中保持尊重和建设性。我们正在一起构建一些东西！

## 有疑问？

- 加入我们的 [Discord](https://discord.gg/WDMCY6aPWK)
- 打开一个 [讨论](https://github.com/wealthfolio/wealthfolio/discussions)
- 邮箱：hello@wealthfolio.app

## 许可证

通过贡献，您同意您的贡献将根据项目的 [AGPL-3.0 许可证](LICENSE) 进行许可，但需遵守我们的 [CLA](CLA.md) 条款。

---

Wealthfolio 和 Wealthfolio 徽标是 Teymz Inc. 的商标。有关商标政策，请参见 [TRADEMARKS.md](TRADEMARKS.md)。
