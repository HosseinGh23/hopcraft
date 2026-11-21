<div align="center">
  <img src="assets/logo.png" alt="لوگوی HopCraft" width="110" />
  <h1>HopCraft</h1>
  <p><a href="README.md">English</a> • <strong>فارسی</strong> • <a href="README.zh.md">中文 (简体)</a> • <a href="README.ru.md">Русский</a></p>
  <p><strong>اپلیکیشن سریع و نیتیو ویندوز برای زنجیره‌سازی (Chain) کانفیگ‌های V2Ray / Xray و تولید فایل JSON تمیز</strong></p>

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

<div dir="rtl">

## HopCraft چیست و چه مشکلی را حل می‌کند؟

فرض کنید یک **کانفیگ اول** دارید که روی اینترنت شما بدون مشکل وصل می‌شود، و یک **کانفیگ دوم** دارید که یا مستقیم وصل نمی‌شود یا آی‌پی خروجی تمیز و ثابتی دارد که می‌خواهید از آن استفاده کنید.

**HopCraft** این دو کانفیگ را با استفاده از قابلیت استاندارد `dialerProxy` در هسته **Xray** به هم زنجیر (Chain) می‌کند و یک فایل `config.json` آماده تحویل می‌دهد. ترافیک شما از طریق **Hop 1** به **Hop 2** می‌رسد، بنابراین **آی‌پی نهایی خروجی شما همیشه آی‌پی کانفیگ دوم است**:

</div>

```text
Your Device  ──▶  Hop 1 (Bridge / کانفیگ اول)  ──▶  Hop 2 (Exit Server / کانفیگ دوم)  ──▶  Internet (آی‌پی نهایی)
```

<div dir="rtl">

این برنامه با **Rust** و **Tauri v2** ساخته شده، بدون نیاز به پیش‌نیاز اضافه به‌سرعت اجرا می‌شود و حجم نهایی آن حدود **۳ تا ۴ مگابایت** است.

---

## امکانات

- **زنجیره‌سازی چندپروتکلی:** ترکیب آزادانه `VLESS`، `VMess`، `Trojan` و `Shadowsocks (ss://)` در Hop 1 و Hop 2.
- **تضمین آی‌پی خروجی:** تنظیم خودکار `sockopt.dialerProxy` به‌طوری که اتصال Hop 2 همیشه از مسیر Hop 1 برقرار شود.
- **تنظیم انعطاف‌پذیر پورت محلی:** پشتیبانی از حالت تک‌پورت **Mixed (SOCKS5 + HTTP)** یا پورت‌های جداگانه `SOCKS5` و `HTTP` (در صورت یکسان بودن دو پورت، خودکار به `mixed` تبدیل می‌شود).
- **آپدیتر داخلی و هوشمند:** بررسی خودکار نسخه جدید هنگام باز شدن برنامه همراه با پنجره مدرن اطلاع‌رسانی، پشتیبانی از نسخه‌های آزمایشی (**Pre-release**) و دانلود داخلی با نوار پیشرفت برای هر دو نسخه پرتابل و نصبی.
- **رابط کاربری ۴ زبانه:** پشتیبانی کامل از **فارسی (راست‌به‌چپ)**، **انگلیسی**، **چینی** و **روسی**.
- **تجربه کاملاً نیتیو ویندوز ۱۱:** طراحی Fluent با هماهنگی خودکار تم تاریک/روشن نوار عنوان، اعتبارسنجی لحظه‌ای لینک‌ها و نمایش رنگی (Syntax Highlight) خروجی JSON.

---

## دانلود

نسخه **نصب‌کننده (MSIX Package)** یا نسخه **پرتابل (بدون نیاز به نصب)** متناسب با معماری ویندوز خود را از صفحه [**Releases**](https://github.com/HosseinGh23/hopcraft/releases) دانلود کنید:

</div>

| معماری ویندوز | نسخه نصبی NSIS (پیشنهادی) | نسخه پرتابل (بدون نصب) |
| :--- | :--- | :--- |
| **Windows x64** | `HopCraft-windows-x64.msix` | `HopCraft-windows-x64-portable.exe` |
| **Windows x86** | `HopCraft-windows-x86.msix` | `HopCraft-windows-x86-portable.exe` |
| **Windows ARM64** | `HopCraft-windows-arm64.msix` | `HopCraft-windows-arm64-portable.exe` |

<div dir="rtl">

> **توجه:** پشتیبانی از سیستم‌عامل‌های **macOS** و **Linux** در نسخه‌های آینده اضافه خواهد شد.

---

## راهنمای سریع استفاده

### ۱. ساخت کانفیگ زنجیره‌ای
1. برنامه **HopCraft** را اجرا کنید.
2. لینک کانفیگ اول (کانفیگی که وصل می‌شود / پل ارتباطی) را در بخش **Hop 1** وارد کنید.
3. لینک کانفیگ دوم (سروری که می‌خواهید آی‌پی نهایی شما باشد) را در بخش **Hop 2** وارد کنید.
   - وضعیت هر دو لینک به‌صورت لحظه‌ای در نوار بالا بررسی و نمایش داده می‌شود.
4. پورت محلی را تنظیم کنید:
   - **حالت Mixed (پیش‌فرض):** یک پورت (مثلاً `10808`) هم‌زمان برای SOCKS5 و HTTP استفاده می‌شود.
   - **پورت‌های جداگانه:** کلید Mixed را خاموش کنید تا پورت `SOCKS5` و `HTTP` را جداگانه تنظیم یا غیرفعال کنید.
5. روی **تولید JSON** کلیک کنید (یا `Ctrl + Enter` بزنید)، سپس با **کپی JSON** یا **ذخیره فایل `.json`** (`Ctrl + S`) خروجی را بردارید.

### ۲. بررسی و دریافت آپدیت‌ها
- هنگام اجرای برنامه، وجود نسخه جدید به‌صورت خودکار بررسی می‌شود و در صورت انتشار نسخه تازه، پنجره آپدیت در مرکز صفحه نمایش داده می‌شود.
- همچنین می‌توانید هر زمان از آیکون آپدیت در هدر بالا برای بررسی دستی یا فعال کردن گزینه **بررسی نسخه‌های آزمایشی (Pre-release)** استفاده کنید.

### ۳. پروتکل‌ها و ترنسپورت‌های پشتیبانی‌شده
- **پروتکل‌ها:** `vless://`، `vmess://` (فرمت Base64 JSON)، `trojan://`، `ss://` (شامل SIP002، فرمت قدیمی Base64 و `v2ray-plugin`)
- **ترنسپورت‌ها:** `tcp` (خام یا هدر HTTP)، `ws`، `grpc`، `xhttp` / `splithttp`، `httpupgrade`، `h2`
- **امنیت:** `none`، `tls` (پشتیبانی از SNI، fingerprint، ALPN و `allowInsecure`)، `reality` (پشتیبانی از `pbk`، `sid` و `spx`)

### ۴. استفاده از فایل `config.json` خروجی
- **در Xray-core (خط فرمان):**

</div>

```powershell
xray run -c config.json
```

<div dir="rtl">

- **در v2rayN یا v2rayNG:** فایل `config.json` را به‌عنوان **Custom Configuration Server** وارد کنید (یا مستقیماً از کلیپ‌بورد پیست کنید).
- **مسیریابی داخلی:** آی‌پی‌های شبکه محلی (`127.0.0.0/8`، `10.0.0.0/8`، `172.16.0.0/12`، `192.168.0.0/16` و `::1/128`) به‌صورت مستقیم (`direct`) عبور می‌کنند و سایر ترافیک‌ها از مسیر `proxy-entry` به `proxy-exit` هدایت می‌شوند.

### ۵. کلیدهای میانبر

</div>

| کلید میانبر | عملکرد |
| :--- | :--- |
| `Ctrl + Enter` | تولید فایل JSON |
| `Ctrl + S` | ذخیره به‌صورت فایل `.json` |
| `Esc` | بستن پنجره آپدیت |

<div dir="rtl">

---

## ساخت از سورس (Build from Source)

نیازمند [Rust](https://rustup.rs/) (نسخه پایدار)، ابزار Tauri CLI (`cargo install tauri-cli`) و ابزارهای بیلد MSVC ویندوز:

</div>

```powershell
cd src-tauri
cargo tauri build 
```

<div dir="rtl">

---

## نقشه راه (Roadmap)

- [x] بیلد کامل ویندوز (`x64`، `x86`، `ARM64`) — نسخه پرتابل و بسته MSIX
- [x] رابط کاربری ۴ زبانه (`EN`، `ZH`، `FA`، `RU`) + تم تاریک و روشن
- [x] آپدیتر داخلی گیت‌هاب همراه با پنجره اطلاع‌رسانی و پشتیبانی از Pre-release
- [ ] پشتیبانی از macOS و Linux
- [ ] زنجیره‌سازی بیش از ۲ هاپ (Multi-hop)

---

## مجوز (License)

این پروژه تحت مجوز [MIT License](LICENSE) منتشر شده است.

</div>
