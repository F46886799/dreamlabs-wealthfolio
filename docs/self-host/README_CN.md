# 自托管 Wealthfolio

Wealthfolio 提供官方 Docker 镜像，因此您可以在自己的硬件上运行 Web 版本。完整的自托管指南在网站上：

📘
**[wealthfolio.app/docs/guide/self-hosting](https://wealthfolio.app/docs/guide/self-hosting)**

此目录仅保存存储库内的文件（Unraid CA 模板）和每个平台的简短指南。

## 镜像

多架构（`linux/amd64`、`linux/arm64`），在每个 `v*.*.*` 标签上发布：

| 注册表     | 镜像                                            |
| ---------- | ----------------------------------------------- |
| Docker Hub | `wealthfolio/wealthfolio:latest` _（主要）_     |
| Docker Hub | `afadil/wealthfolio:latest` _（旧版镜像）_      |
| GHCR       | `ghcr.io/wealthfolio/wealthfolio:latest`        |

```bash
docker pull wealthfolio/wealthfolio:latest
```

将 `afadil/wealthfolio:latest` 固定的现有部署继续工作——两个 Docker Hub 存储库都从 CI 接收相同的多架构构建。新部署应优先使用 `wealthfolio/wealthfolio`。

## 权限

容器以非 root 用户（UID/GID **1000:1000**）运行。

**全新安装：** Docker 命名卷开箱即用。对于绑定挂载，使主机目录可由 UID 1000 写入：

```bash
mkdir -p ./data && sudo chown -R 1000:1000 ./data
```

**从旧镜像升级：** 现有数据由 `root` 拥有，必须 chowned 一次。选择与您的设置匹配的行：

```bash
# 命名卷
docker run --rm -v <your-volume>:/data alpine chown -R 1000:1000 /data
# 绑定挂载
sudo chown -R 1000:1000 /path/to/your/data
```

## 平台指南

- [**Docker / Docker Compose**](https://wealthfolio.app/docs/guide/self-hosting)：规范路径。网站上的完整演练。
- [**Unraid**](./unraid/)：通过社区应用安装。CA 模板位于此存储库的 [`unraid/template.xml`](./unraid/template.xml)。
- [**Proxmox VE**](./proxmox/)：通过 community-scripts 的 LXC，Docker-in-LXC 或 Docker VM。
