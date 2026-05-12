# E2E 测试

Wealthfolio E2E 测试使用 [Playwright](https://playwright.dev/)，并针对 **Web 应用**（而非 Tauri 桌面应用）运行。**没有模拟**——前端和后端都必须在全新数据库上运行。

---

## 先决条件

- 已安装 `pnpm`
- 已安装 Rust 工具链（用于后端服务器）
- 已安装 Chrome（Playwright 使用系统 Chrome）

---

## 运行 E2E 测试

### 自动化——完整套件

运行整个套件最简单的方法。自动处理所有事情：准备全新数据库，启动 Web 应用，等待两个服务器，运行 Playwright，然后关闭所有内容。

```bash
pnpm test:e2e
```

要打开 Playwright UI：

```bash
pnpm test:e2e:ui
```

---

### 手动——特定测试或调试

当您想运行测试子集或快速迭代而无需在每次运行时重新启动服务器时使用此方法。

#### 步骤 1——准备全新数据库

```bash
node scripts/prep-e2e.mjs
```

这会创建一个新的带时间戳的 SQLite 数据库（例如 `db/app-testing-20260411T120000Z.db`）并将其路径写入 `.env.web`。**每次在启动服务器之前运行此命令**——它确保测试隔离。

#### 步骤 2——启动 Web 应用

**选项 A——直接查看终端输出：**

```bash
pnpm run dev:web
```

等待直到您看到 Vite 的"ready in Xms"和 Rust 服务器绑定消息，然后在单独的终端中继续步骤 3。

**选项 B——将输出重定向到日志文件并使用等待脚本：**

```bash
pnpm run dev:web > /tmp/wealthfolio-dev2.log 2>&1 &
./scripts/wait-for-both-servers-to-be-ready.sh
```

`wait-for-both-servers-to-be-ready.sh` 轮询日志文件，直到检测到"ready in"（Vite）和 Axum 服务器在端口 8088 上绑定，然后打印最后几行并退出。输出重定向是必需的——脚本从文件读取，而不是从实时终端读取。

> **如果 Web 应用已经在运行：** 先停止它（Ctrl+C），然后重新运行 `prep-e2e.mjs` 并重新启动。正在运行的实例使用的是过时的数据库——测试假设数据库为空，如果数据已经存在，将默默跳过资产创建，导致无关原因的失败。

#### 步骤 3——运行特定测试

```bash
# 运行特定的 spec 文件
npx playwright test e2e/10-symbol-mapping-validation.spec.ts

# 使浏览器可见运行（用于调试）
npx playwright test e2e/10-symbol-mapping-validation.spec.ts --headed

# 运行所有测试
npx playwright test

# 运行并在之后打开 HTML 报告
npx playwright test && npx playwright show-report
```

---

## 重要规则

- **在启动服务器之前始终运行 `prep-e2e.mjs`。** 测试假设数据库为空。如果您针对现有数据库运行，设置步骤可能会默默跳过资产创建，测试可能会因无关原因而失败。
- **不要针对 Tauri 桌面应用运行 E2E 测试。** 测试硬编码为 `http://localhost:1420`。
- **在 Tauri 开发服务器（`pnpm tauri dev`）运行时不要运行 E2E 测试**，如果它们在相同端口上——它们会冲突。
- 测试**串行运行**（1 个 worker，串行模式）。不要尝试并行化它们。

---

## 测试文件

| 文件                                   | 测试内容                                                                                                         |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `01-happy-path.spec.ts`                | 入门、账户、存款、交易                                                                                           |
| `02-activities.spec.ts`                | 所有活动类型                                                                                                     |
| `03-fx-cash-balance.spec.ts`           | 外汇现金余额                                                                                                     |
| `04-csv-import.spec.ts`                | CSV 活动导入                                                                                                     |
| `05-form-validation.spec.ts`           | 表单字段验证错误                                                                                                 |
| `06-activity-data-grid.spec.ts`        | 活动数据网格交互                                                                                                 |
| `07-asset-creation.spec.ts`            | 手动资产创建和编辑                                                                                               |
| `08-holdings-and-performance.spec.ts`  | 持仓和绩效视图                                                                                                   |
| `09-bulk-holdings.spec.ts`             | 批量持仓 CSV 导入                                                                                                |
| `10-symbol-mapping-validation.spec.ts` | 符号映射实时验证（Yahoo Finance、法兰克福证券交易所）                                                            |
| `13-multi-exchange-import.spec.ts`     | 多交易所 CSV 导入：XETRA/LSE/TSX/NASDAQ 解析、地区和工具类型分类（问题 #855）                                    |

---

## 调试失败的测试

```bash
# 使用 Playwright 检查器运行（逐步执行操作）
npx playwright test e2e/<spec>.spec.ts --debug

# 显示最后一个 HTML 报告
npx playwright show-report

# 为失败的测试记录跟踪（在重试时保存跟踪）
# 已在 playwright.config.ts 中配置：trace: "on-first-retry"
```
