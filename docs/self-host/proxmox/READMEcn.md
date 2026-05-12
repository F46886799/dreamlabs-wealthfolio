# Wealthfolio 在 Proxmox VE 上

Proxmox 上的三种合理安装路径：通过 [community-scripts](https://community-scripts.github.io/ProxmoxVE/) 项目的原生 LXC、LXC 内的 Docker 或 VM 内的 Docker。LXC 路径符合 Proxmox 约定（无 Docker-in-LXC），但在每次安装时从源构建（约 15-25 分钟，在 [#563](https://github.com/wealthfolio/wealthfolio/issues/563) 中跟踪）。Docker 路径更快但引入嵌套。

📘 **完整设置指南：**
[wealthfolio.app/docs/guide/self-hosting](https://wealthfolio.app/docs/guide/self-hosting)

## 入门：LXC（推荐）

在 **Proxmox 主机**上打开 shell（不是在现有容器内）并运行：

```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/ct/wealthfolio.sh)"
```

默认值：Debian 13，4 CPU / 4 GB RAM / 10 GB 磁盘，端口 `8080`。凭据写入容器内的 `/root/wealthfolio.creds`。

## 入门：Docker

如果您已经在 Proxmox 上运行 Docker 主机（LXC 或 VM），只需在那里部署容器，就像任何其他服务一样。有关完整的 Compose 演练，请参见上面的网站指南。
