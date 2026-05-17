# Copilot Instructions

## 项目概述
Tauri 桌面应用 + Web 模式，React 前端 + Rust 后端，pnpm monorepo。

## 架构约定
- 前端状态：React Query（不要用 useState 做服务端数据）
- 表单校验：Zod schema
- UI 组件：shadcn/ui（@wealthfolio/ui 包装层）
- 后端 ORM：Diesel（不要裸写 SQL 字符串）
- 错误处理：Rust 用 Result<T, E>，前端用 React Query 的 error 状态

## IPC 约定
Tauri 命令在 apps/tauri/src/ 定义，前端通过 apps/frontend/src/adapters/ 调用，
Web 模式通过 apps/server/ REST API。新功能必须同时支持两种 adapter。

## 禁止事项
- 不要在 Rust 代码中使用 unwrap()，用 ? 操作符
- 不要直接操作 DOM，使用 React 声明式写法
- 不要跳过 Zod 校验直接提交表单数据
