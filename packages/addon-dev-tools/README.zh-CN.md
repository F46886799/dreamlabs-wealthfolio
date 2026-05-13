# @wealthfolio/addon-dev-tools

Wealthfolio 插件的开发工具，包括热重载服务器和 CLI。

## 安装

```bash
npm install -g @wealthfolio/addon-dev-tools
```

## CLI 命令

### 创建新插件

```bash
wealthfolio create my-awesome-addon
```

### 启动开发服务器

```bash
# 在您的插件目录中
wealthfolio dev
```

### 构建插件

```bash
wealthfolio build
```

### 打包以供分发

```bash
wealthfolio package
```

### 测试设置

```bash
wealthfolio test
```

## 开发服务器

开发服务器提供：

- 热重载功能
- 文件监视
- 自动构建
- 健康检查端点

### API 端点

- `GET /health` - 健康检查
- `GET /status` - 插件状态和最后修改时间
- `GET /manifest.json` - 插件清单
- `GET /addon.js` - 构建的插件代码
- `GET /files` - 构建文件列表
- `GET /test` - 测试连接性

## 在插件项目中使用

添加到您的插件的 `package.json`：

```json
{
  "scripts": {
    "dev:server": "wealthfolio dev"
  },
  "devDependencies": {
    "@wealthfolio/addon-dev-tools": "^1.0.0"
  }
}
```

## 架构

此包与 `@wealthfolio/addon-sdk` 分开，以：

- 保持 SDK 轻量级用于生产
- 避免插件包中的不必要依赖项
- 提供可选的开发工具

## 许可证

MIT
