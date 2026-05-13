# Wealthfolio 在 Unraid 上

Wealthfolio 作为通过 Unraid 的 Docker 标签管理的标准 Docker 容器运行。社区应用（CA）模板位于此 README 旁边的 [`template.xml`](./template.xml)。CA 直接从存储库获取它。

📘 **完整设置指南：**
[wealthfolio.app/docs/guide/self-hosting](https://wealthfolio.app/docs/guide/self-hosting)

## 入门

### 从社区应用（一键）

**Apps** 标签 → 搜索 **Wealthfolio** → **Install** → 填写 `WF_SECRET_KEY`、`WF_AUTH_PASSWORD_HASH`、`WF_CORS_ALLOW_ORIGINS` → **Apply**。

### 手动侧载

如果 CA 尚未获取最新模板，请从此存储库侧载它。SSH 到 Unraid（或使用 WebTerminal）并运行：

```bash
mkdir -p /boot/config/plugins/dockerMan/templates-user
curl -fsSL \
  https://raw.githubusercontent.com/wealthfolio/wealthfolio/main/docs/self-host/unraid/template.xml \
  -o /boot/config/plugins/dockerMan/templates-user/my-wealthfolio.xml
```

然后 **Docker → Add Container → Template → User templates → wealthfolio**。

有关所需值、密码哈希配方、反向代理设置、备份和故障排除，请参见网站指南。

## 权限

镜像默认以非 root 用户（UID 1000）运行。Unraid 模板通过 `<ExtraParams>` 中的 `--user=99:100` 覆盖此设置，因此容器与 Unraid 的标准 `nobody:users` appdata 所有权匹配——全新安装无需任何主机端 `chown` 即可工作。

**从旧镜像升级**（`v3.4.0` 之前）：现有数据以 `root:root` 写入。在 Unraid 主机上运行一次：

```bash
chown -R 99:100 /mnt/user/appdata/wealthfolio
```
