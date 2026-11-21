<div align="center">
  <img src="assets/logo.png" alt="HopCraft Logo" width="110" />
  <h1>HopCraft</h1>
  <p><a href="README.md">English</a> • <a href="README.fa.md">فارسی</a> • <a href="README.zh.md">中文 (简体)</a> • <strong>Русский</strong></p>
  <p><strong>Быстрое нативное Windows-приложение для объединения прокси-конфигов V2Ray / Xray в цепочку JSON</strong></p>

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

## Зачем нужен HopCraft?

**HopCraft** решает распространённую задачу маршрутизации: допустим, у вас есть **Конфиг 1**, который стабильно подключается в вашей сети, и **Конфиг 2**, который напрямую недоступен или даёт нужный вам выходной IP-адрес.

HopCraft связывает оба конфига через встроенный механизм `dialerProxy` в Xray и формирует готовый `config.json`. Трафик идёт через **Hop 1** до **Hop 2**, поэтому **итоговый внешний IP всегда принадлежит Hop 2**:

```text
Your Device  ──▶  Hop 1 (Bridge / Промежуточный узел)  ──▶  Hop 2 (Exit Server / Выходной узел)  ──▶  Internet (IP Hop 2)
```

Приложение написано на **Rust** с использованием **Tauri v2**, запускается мгновенно без дополнительных зависимостей и весит около **3–4 МБ**.

---

## Возможности

- **Цепочки из разных протоколов:** свободное сочетание `VLESS`, `VMess`, `Trojan` и `Shadowsocks (ss://)` для Hop 1 и Hop 2.
- **Гарантированный выходной IP:** автоматическая настройка `sockopt.dialerProxy`, чтобы Hop 2 подключался через Hop 1.
- **Гибкие локальные порты:** единый порт **Mixed (SOCKS5 + HTTP)** или раздельные порты `SOCKS5` и `HTTP` (одинаковые порты автоматически объединяются в `mixed`).
- **Встроенный обновлятор:** автоматическое уведомление при запуске, ручная проверка релизов и **Pre-release** с индикатором загрузки для Portable и Пакет MSIXа.
- **4 языка интерфейса:** мгновенное переключение между **Русский**, **English**, **فارسی** и **中文 (简体)**.
- **Нативный стиль Windows 11:** дизайн Fluent, синхронизация тёмной/светлой темы заголовка окна, проверка ссылок в реальном времени и подсветка JSON.

---

## Скачать

Выберите **Пакет MSIX** или **Portable-версию** под архитектуру вашего Windows на странице [**Releases**](https://github.com/HosseinGh23/hopcraft/releases):

| Архитектура Windows | Установщик NSIS (рекомендуется) | Portable (без установки) |
| :--- | :--- | :--- |
| **Windows x64** | `HopCraft-windows-x64.msix` | `HopCraft-windows-x64-portable.exe` |
| **Windows x86** | `HopCraft-windows-x86.msix` | `HopCraft-windows-x86-portable.exe` |
| **Windows ARM64** | `HopCraft-windows-arm64.msix` | `HopCraft-windows-arm64-portable.exe` |

> **Примечание:** поддержка macOS и Linux появится в будущих версиях.

---

## Документация

### 1. Быстрый старт
1. Запустите **HopCraft**.
2. Вставьте рабочий/промежуточный конфиг в **Hop 1 — Entry / Bridge**.
3. Вставьте целевой выходной конфиг в **Hop 2 — Exit Server** (индикаторы сверху проверяют ссылки в реальном времени).
4. Настройте локальный порт:
   - **Режим Mixed (по умолчанию):** один порт (например `10808`) работает одновременно для SOCKS5 и HTTP.
   - **Раздельные порты:** отключите Mixed, чтобы задать или выключить `SOCKS5` и `HTTP` по отдельности.
5. Нажмите **Generate JSON** (`Ctrl + Enter`), затем скопируйте результат (**Copy JSON**) или сохраните в файл (**Save as .json**, `Ctrl + S`).

### 2. Поддерживаемые протоколы и транспорты
- **Протоколы:** `vless://`, `vmess://` (Base64 JSON), `trojan://`, `ss://` (SIP002, старый Base64 и `v2ray-plugin`)
- **Транспорты:** `tcp` (raw / HTTP-заголовок), `ws`, `grpc`, `xhttp` / `splithttp`, `httpupgrade`, `h2`
- **Безопасность:** `none`, `tls` (SNI, fingerprint, ALPN, `allowInsecure`), `reality` (`pbk`, `sid`, `spx`)

### 3. Использование `config.json`
- **Запуск через Xray-core:**
  ```powershell
  xray run -c config.json
  ```
- **В v2rayN / v2rayNG:** импортируйте `config.json` как пользовательскую конфигурацию или вставьте из буфера обмена.
- **Маршрутизация:** локальные и частные адреса (`127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `::1/128`) идут напрямую (`direct`), остальной трафик идёт через `proxy-entry` в `proxy-exit`.

---

## Сборка из исходного кода

Требуются [Rust](https://rustup.rs/) (stable), Tauri CLI (`cargo install tauri-cli`) и инструменты сборки MSVC для Windows:

```powershell
cd src-tauri
cargo tauri build 
```

---

## Лицензия

Распространяется по лицензии [MIT License](LICENSE).
