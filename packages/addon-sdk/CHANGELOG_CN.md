# 变更日志

格式基于 [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)，
此项目遵循 [语义版本控制](https://semver.org/spec/v2.0.0.html)。

## [1.0.0] - 2024-12-19

### 新增

- **初始发布** - 用于构建 Wealthfolio 插件的完整 TypeScript SDK
- **核心类型**：AddonContext、SidebarManager、RouterManager 和事件处理
- **数据类型**：全面的财务数据模型（Account、Activity、Asset、Holding 等）
- **权限系统**：基于风险的权限分类，带验证和安全控制
- **清单管理**：验证、兼容性检查和元数据处理
- **主机 API 接口**：插件和 Wealthfolio 之间的安全通信层
- **实用函数**：插件验证、版本兼容性、ID 生成和大小格式化
- **开发工具**：完整的 TypeScript 定义和开发实用程序

### 功能

- 使用完整数据类型定义增强类型安全
- 基于风险的权限系统分类（低、中、高）
- 插件生命周期管理（安装、验证、更新、启用/禁用）
- 开发和运行时清单类型
- 所有插件功能的全面导出定义
- 支持插件商店列表和元数据

### 技术

- 带 TypeScript 声明的 ESM 模块格式
- React peer dependency 支持（^18.0.0）
- Node.js 20+ 兼容性
- MIT 许可证，以获得最大兼容性
- 使用 tsup 的完整构建管道
- 全面的类型导出和模块结构
