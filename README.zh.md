<div align="center">
  <img src="assets/logo.png" alt="HopCraft Logo" width="110" />
  <h1>HopCraft</h1>
  <p><a href="README.md">English</a> • <a href="README.fa.md">فارسی</a> • <strong>中文 (简体)</strong> • <a href="README.ru.md">Русский</a></p>
  <p><strong>快速、原生的 Windows 桌面应用，用于将 V2Ray / Xray 代理配置串联为干净的 JSON</strong></p>

  <p>
    <a href="https://github.com/HosseinGh23/hopcraft/releases"><img src="https://img.shields.io/github/v/release/HosseinGh23/hopcraft?style=flat-square&color=0078D4" alt="Latest Release" /></a>
    <a href="https://github.com/HosseinGh23/hopcraft/releases"><img src="https://img.shields.io/github/downloads/HosseinGh23/hopcraft/total?style=flat-square&color=1A7F37" alt="Downloads" /></a>
    <a href="https://github.com/HosseinGh23/hopcraft/stargazers"><img src="https://img.shields.io/github/stars/HosseinGh23/hopcraft?style=flat-square&color=D29922" alt="Stars" /></a>
    <a href="https://github.com/HosseinGh23/hopcraft/actions/workflows/build.yml"><img src="https://img.shields.io/github/actions/workflow/status/HosseinGh23/hopcraft/build.yml?style=flat-square&label=build" alt="Build Status" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/HosseinGh23/hopcraft?style=flat-square" alt="License" /></a>
    <img src="https://img.shields.io/badge/platform-Windows%20x64%20%7C%20x86%20%7C%20ARM64-0078D4?style=flat-square" alt="Platform" />
  </p>
</div>

---

## 什么是 HopCraft？

**HopCraft** 用于解决常见的代理链路问题：假设你有一个**配置 1**（在当前网络下连接稳定），以及一个**配置 2**（无法直连，或拥有你希望使用的干净落地 IP）。

HopCraft 利用 Xray 原生的 `dialerProxy` 机制将两个配置串联，并生成可直接使用的 `config.json`。流量先经过 **Hop 1** 再连接到 **Hop 2**，因此**最终出口 IP 始终是 Hop 2**：

```text
Your Device  ──▶  Hop 1 (Bridge / 中转节点)  ──▶  Hop 2 (Exit Server / 落地节点)  ──▶  Internet (Hop 2 IP)
```

基于 **Rust** 与 **Tauri v2** 构建，无需额外运行库，启动迅速，体积仅约 **3–4 MB**。

---

## 功能特性

- **多协议串联：** 支持在 Hop 1 与 Hop 2 中自由组合 `VLESS`、`VMess`、`Trojan` 与 `Shadowsocks (ss://)`。
- **出口 IP 保证：** 自动配置 `sockopt.dialerProxy`，确保 Hop 2 始终通过 Hop 1 拨号。
- **灵活的本地入站：** 支持单一 **Mixed (SOCKS5 + HTTP)** 端口或独立的 `SOCKS5` / `HTTP` 端口（端口相同时自动合并为 `mixed`）。
- **内置更新器：** 支持启动自动弹窗提示、手动检查稳定版或 **Pre-release** 测试版，并提供下载进度条（支持便携版与 NSIS 安装版）。
- **四语言界面：** 支持 **中文 (简体)**、**English**、**فارسی** 与 **Русский** 实时切换。
- **原生 Windows 11 体验：** Fluent 设计风格、标题栏深色/浅色同步、实时链接校验徽章与 JSON 语法高亮。

---

## 下载

前往 [**Releases**](https://github.com/HosseinGh23/hopcraft/releases) 页面下载对应 Windows 架构的 **Inno Setup 安装包** 或 **便携版** 可执行文件：

| Windows 架构 | Inno Setup 安装包（推荐） | 便携版（免安装） |
| :--- | :--- | :--- |
| **Windows x64** | `HopCraft-windows-x64-setup.exe` | `HopCraft-windows-x64-portable.exe` |
| **Windows x86** | `HopCraft-windows-x86-setup.exe` | `HopCraft-windows-x86-portable.exe` |
| **Windows ARM64** | `HopCraft-windows-arm64-setup.exe` | `HopCraft-windows-arm64-portable.exe` |

> **提示：** macOS 与 Linux 支持将在后续版本中提供。

---

## 使用文档

### 1. 快速开始
1. 启动 **HopCraft**。
2. 将可用/中转节点链接粘贴到 **Hop 1 — Entry / Bridge**。
3. 将目标落地节点链接粘贴到 **Hop 2 — Exit Server**（顶部状态徽章会实时校验链接）。
4. 配置本地监听端口：
   - **Mixed 模式（默认）：** 单个端口（如 `10808`）同时提供 SOCKS5 与 HTTP 代理。
   - **独立端口：** 关闭 Mixed 后可分别设置或停用 `SOCKS5` 与 `HTTP` 端口。
5. 点击 **Generate JSON**（或按 `Ctrl + Enter`），随后点击 **Copy JSON** 或 **Save as .json**（`Ctrl + S`）。

### 2. 支持的协议与传输方式
- **协议：** `vless://`、`vmess://`（Base64 JSON）、`trojan://`、`ss://`（SIP002、传统 Base64 及 `v2ray-plugin`）
- **传输：** `tcp`（raw / HTTP 头）、`ws`、`grpc`、`xhttp` / `splithttp`、`httpupgrade`、`h2`
- **安全：** `none`、`tls`（SNI、指纹、ALPN、`allowInsecure`）、`reality`（`pbk`、`sid`、`spx`）

### 3. 使用生成的 `config.json`
- **Xray-core 命令行：**
  ```powershell
  xray run -c config.json
  ```
- **v2rayN / v2rayNG：** 将保存的 `config.json` 作为自定义配置导入（或直接从剪贴板粘贴）。
- **路由规则：** 本地与局域网地址（`127.0.0.0/8`、`10.0.0.0/8`、`172.16.0.0/12`、`192.168.0.0/16`、`::1/128`）直连（`direct`），其余流量经由 `proxy-entry` 连接至 `proxy-exit`。

---

## 从源码构建

需要安装 [Rust](https://rustup.rs/)（stable）、Tauri CLI（`cargo install tauri-cli`）及 Windows MSVC 构建工具：

```powershell
cd src-tauri
cargo tauri build 
```

---

## 许可证

本项目基于 [MIT License](LICENSE) 开源。
