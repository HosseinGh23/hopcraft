<div align="center">
  <img src="assets/logo.png" alt="HopCraft Logo" width="110" />
  <h1>HopCraft</h1>
  <p><strong>English</strong> • <a href="README.fa.md">فارسی</a> • <a href="README.zh.md">中文 (简体)</a> • <a href="README.ru.md">Русский</a></p>
  <p><strong>Fast, Native Windows Desktop App for Chaining V2Ray / Xray Proxy Configs into Clean JSON</strong></p>

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

## What is HopCraft?

**HopCraft** solves a common proxy routing problem: suppose you have **Config 1** (which connects reliably on your network) and **Config 2** (which is blocked directly or has a clean residential/exit IP you want to use).

HopCraft chains both configs together using Xray's native `dialerProxy` feature and outputs a ready-to-use `config.json`. Your traffic travels through **Hop 1** to reach **Hop 2**, so your **final outgoing IP is always Hop 2**:

```text
Your Device  ──▶  Hop 1 (Bridge / Working Config)  ──▶  Hop 2 (Exit Server)  ──▶  Internet (Hop 2 IP)
```

Built with **Rust** and **Tauri v2**, HopCraft launches instantly, requires zero runtime installations, and stays around **3–4 MB**.

---

## Features

- **Multi-Protocol Chaining:** Mix and match `VLESS`, `VMess`, `Trojan`, and `Shadowsocks (ss://)` across Hop 1 and Hop 2.
- **Guaranteed Exit IP:** Automatically configures `sockopt.dialerProxy` so Hop 2 always dials through Hop 1.
- **Flexible Local Inbounds:** Use a single **Mixed (SOCKS5 + HTTP)** port or separate custom `SOCKS5` and `HTTP` ports (matching ports auto-merge into `mixed`).
- **Built-in Updater:** Automatic startup update popup plus manual checking for stable or **Pre-release** builds with live download progress for both Portable and Inno Setup Installer editions.
- **Multi-Language UI:** Switch instantly between **English**, **中文 (简体)**, **فارسی (RTL)**, and **Русский**.
- **Native Windows 11 Experience:** Fluent Design UI with synced Light/Dark title bar, live URI validation pills, and syntax-highlighted JSON output.

---

## Download

Download either the **Inno Setup Installer** or the **Portable** executable for your Windows architecture from the [**Releases**](https://github.com/HosseinGh23/hopcraft/releases) page:

| Architecture | Inno Setup Installer (Recommended) | Portable (No Install) |
| :--- | :--- | :--- |
| **Windows x64** | `HopCraft-windows-x64-setup.exe` | `HopCraft-windows-x64-portable.exe` |
| **Windows x86** | `HopCraft-windows-x86-setup.exe` | `HopCraft-windows-x86-portable.exe` |
| **Windows ARM64** | `HopCraft-windows-arm64-setup.exe` | `HopCraft-windows-arm64-portable.exe` |

> **Note:** macOS and Linux builds are planned for future releases.

---

## Documentation

### 1. Quick Start
1. Launch **HopCraft**.
2. Paste your working/bridge proxy link into **Hop 1 — Entry / Bridge**.
3. Paste your target exit proxy link into **Hop 2 — Exit Server**.
   - The top status pills validate both URIs in real time.
4. Configure your local proxy port:
   - **Mixed Mode (Default):** One port (e.g. `10808`) handles both SOCKS5 and HTTP traffic (`"protocol": "mixed"`).
   - **Separate Ports:** Toggle off Mixed to set independent `SOCKS5` and `HTTP` ports, or disable either one.
5. Click **Generate JSON** (or press `Ctrl + Enter`), then use **Copy JSON** or **Save as .json** (`Ctrl + S`).

### 2. Built-in Update Checker
- Click the **Download / Update** icon in the top header to check GitHub Releases.
- Enable **Include Pre-releases** to receive beta / pre-release builds (tags like `v1.1.0-beta.1`).
- HopCraft automatically detects whether you are running the **NSIS Installed** or **Portable** build and downloads the matching architecture binary with live progress.

### 3. Supported URI Formats & Transports
- **Protocols:** `vless://`, `vmess://` (Base64 JSON), `trojan://`, `ss://` (SIP002, legacy Base64, and `v2ray-plugin`)
- **Transports:** `tcp` (raw / HTTP header), `ws`, `grpc`, `xhttp` / `splithttp`, `httpupgrade`, `h2`
- **Security:** `none`, `tls` (with SNI, fingerprint, ALPN, `allowInsecure`), `reality` (with `pbk`, `sid`, `spx`)

### 4. Using the Generated `config.json`
- **With Xray-core CLI:**
  ```powershell
  xray run -c config.json
  ```
- **With v2rayN / v2rayNG:** Import the saved `config.json` as a **Custom Configuration Server** (or paste directly from clipboard).
- **Routing Behavior:** Local/LAN IPs (`127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `::1/128`) bypass the proxy (`direct`), while all external traffic routes through `proxy-exit` via `proxy-entry`.

### 5. Keyboard Shortcuts
| Shortcut | Action |
| :--- | :--- |
| `Ctrl + Enter` | Generate JSON |
| `Ctrl + S` | Save as `.json` |

---

## Building from Source

Requires [Rust](https://rustup.rs/) (stable), Tauri CLI v2 (`cargo install tauri-cli`), and Windows MSVC build tools:

```powershell
cd src-tauri
cargo tauri build 
```

---

## Roadmap

- [x] Windows builds (`x64`, `x86`, `ARM64`) — Portable & Inno Setup Installer
- [x] 4-language UI (`EN`, `ZH`, `FA`, `RU`) + Light/Dark theme
- [x] In-app GitHub updater with Pre-release channel support
- [ ] macOS & Linux support
- [ ] Multi-hop chains (3+ hops)

---

## License

Released under the [MIT License](LICENSE).
