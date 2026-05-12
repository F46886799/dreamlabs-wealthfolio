# AGENTS.md

此存储库的 AI 代理指南。涵盖行为规则、架构和常见任务手册。

---

## 行为准则

**这些首先出现，因为它们可以防止大多数错误。**

### 1. 编码前思考

- 明确陈述假设。如果不确定，请询问。
- 如果存在多种解释，请呈现它们——不要默默选择。
- 如果存在更简单的方法，请说出来。在必要时反对。

### 2. 简单优先

- 不要超出要求的功能。
- 不要为单次使用的代码创建抽象。
- 不要为不可能的场景处理错误。
- 如果 200 行可以是 50 行，请重写它。

### 3. 精准修改

- 不要"改进"相邻的代码、注释或格式。
- 不要重构没有问题的东西。
- 匹配现有风格，即使您会以不同的方式做。
- 如果您注意到不相关的问题，请提及它们——不要修复它们。
- 仅删除您的更改使其未使用的内容。

### 4. 目标驱动执行

- 将任务转化为可验证的目标。
- 对于多步骤任务，陈述一个简要的计划和验证步骤。
- 未经验证的工作是不完整的工作。

### 5. 输出精度

- 以发现为主导，而不是过程描述。
- 使用结构化格式（列表、表格、代码块）。
- 包括绝对文件路径——永远不要相对路径。

---

## 概述

- **前端**：React + Vite + Tailwind v4 + shadcn（`apps/frontend/`）
- **桌面**：Tauri/Rust，使用 SQLite（`apps/tauri/`、`crates/`）
- **Web 模式**：Axum HTTP 服务器（`apps/server/`）
- **包**：`@wealthfolio/ui`、addon-sdk、addon-dev-tools（`packages/`）

## 代码布局

```
apps/frontend/
├── src/
│   ├── pages/          # 路由页面
│   ├── components/     # 共享组件
│   ├── features/       # 自包含功能模块
│   ├── commands/       # 后端调用包装器（Tauri/Web）
│   ├── adapters/       # 运行时检测（桌面 vs Web）
│   └── addons/         # 插件运行时

apps/tauri/src/
└── commands/           # Tauri IPC 命令

apps/server/src/
└── api/                # Axum HTTP 处理程序

crates/
├── core/               # 业务逻辑、模型、服务
├── storage-sqlite/     # Diesel ORM、存储库、迁移
├── market-data/        # 市场数据提供商
├── connect/            # 外部集成
├── device-sync/        # 设备同步、E2EE
└── ai/                 # AI 提供商和 LLM 集成
```

## 运行目标

| 任务           | 命令                 |
| -------------- | -------------------- |
| 桌面开发       | `pnpm tauri dev`     |
| Web 开发       | `pnpm run dev:web`   |
| 测试（TS）     | `pnpm test`          |
| 测试（Rust）   | `cargo test`         |
| 类型检查       | `pnpm type-check`    |
| Lint           | `pnpm lint`          |
| 所有检查       | `pnpm check`         |

---

## 代理手册

### 添加具有后端数据的功能

1. **前端路由/UI** → `apps/frontend/src/pages/`、`apps/frontend/src/routes.tsx`
2. **命令包装器** → `apps/frontend/src/commands/<domain>.ts`（遵循 `RUN_ENV` 模式）
3. **Tauri 命令** → `apps/tauri/src/commands/*.rs`，在 `mod.rs` + `lib.rs` 中连接
4. **Web 端点** → `apps/server/src/api/`，调用 `crates/core` 服务
5. **核心逻辑** → `crates/core/` 服务/存储库
6. **测试** → TS 使用 Vitest，Rust 使用 `#[test]`

### UI 模式

- 组件：`@wealthfolio/ui` 和 `packages/ui/src/components/`
- 表单：`react-hook-form` + `zod` 架构，来自 `apps/frontend/src/lib/schemas.ts`
- 主题：`apps/frontend/src/globals.css` 中的令牌

### 架构模式

```
前端 → 适配器（tauri/web）→ 命令包装器
                ↓
        Tauri IPC  |  Axum HTTP
                ↓
           crates/core（业务逻辑）
                ↓
           crates/storage-sqlite
```

---

## 约定

### TypeScript

- 严格模式，无未使用的局部变量/参数
- 优先使用接口而不是类型，避免枚举
- 功能组件，命名导出
- 目录名称：小写带破折号

### Rust

- 惯用的 Rust，小而专注的函数
- `Result`/`Option`，使用 `?` 传播，`thiserror` 用于领域错误
- 保持 Tauri/Axum 命令简洁——委托给 `crates/core`
- 迁移在 `crates/storage-sqlite/migrations` 中

### 安全

- 所有数据本地（SQLite），无云
- 通过 OS 密钥环保存密钥——永远不要磁盘/localStorage
- 永远不要记录密钥或财务数据

---

## 验证清单

完成任何任务之前：

- [ ] 构建：`pnpm build` 或 `pnpm tauri dev` 或 `cargo check`
- [ ] 测试通过：`pnpm test` 和/或 `cargo test`
- [ ] 如果接触共享代码，桌面和 Web 都编译
- [ ] 更改是最小和精准的

---

## 计划模式

- 使计划极其简洁。为了简洁而牺牲语法。
- 如果有未解决的问题，以此结尾。

---

如有疑问，请遵循最接近的现有模式。
