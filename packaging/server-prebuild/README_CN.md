# Wealthfolio 服务器 — Linux amd64 预构建

用于自托管的独立 HTTP 服务器构建（无 Tauri，无桌面运行时）。在 Ubuntu 22.04（glibc 2.35）上构建——在 Debian 12+、Ubuntu 22.04+ 及其衍生版本上运行。

## 布局

- `wealthfolio-server` — 服务器二进制文件（安装到 `/usr/local/bin/`）
- `dist/` — 前端静态资产（将 `WF_STATIC_DIR` 指向此处）
- `wealthfolio.service.example` — systemd 单元示例
- `LICENSE`

## 快速开始

```bash
sudo install -m 755 wealthfolio-server /usr/local/bin/wealthfolio-server
sudo mkdir -p /opt/wealthfolio /opt/wealthfolio_data
sudo cp -r dist /opt/wealthfolio/dist

sudo tee /opt/wealthfolio/.env >/dev/null <<EOF
WF_LISTEN_ADDR=0.0.0.0:8080
WF_DB_PATH=/opt/wealthfolio_data/wealthfolio.db
WF_STATIC_DIR=/opt/wealthfolio/dist
WF_SECRET_KEY=$(openssl rand -base64 32)
WF_AUTH_PASSWORD_HASH=<argon2id 哈希，请参见 docs/self-host>
# 当启用身份验证且您通过与绑定地址不同的方案/主机/端口访问服务器时需要（例如反向代理）。
# 必须与浏览器地址栏中的 URL 完全匹配。设置"*"将被拒绝。
WF_CORS_ALLOW_ORIGINS=http://<your-server-ip>:8080
EOF
sudo chmod 600 /opt/wealthfolio/.env

sudo cp wealthfolio.service.example /etc/systemd/system/wealthfolio.service
sudo systemctl enable --now wealthfolio
```

完整的自托管文档：
<https://github.com/wealthfolio/wealthfolio/blob/main/docs/self-host/>
