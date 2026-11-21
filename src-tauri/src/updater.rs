use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::Emitter;

pub const REPO_OWNER: &str = "HosseinGh23";
pub const REPO_NAME: &str = "hopcraft";
pub const REPO_URL: &str = "https://github.com/HosseinGh23/hopcraft";

#[derive(Debug, Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

#[derive(Debug, Deserialize)]
struct GhRelease {
    tag_name: String,
    prerelease: bool,
    draft: bool,
    html_url: String,
    body: Option<String>,
    assets: Vec<GhAsset>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppRuntimeInfo {
    pub version: String,
    pub arch: String,
    pub is_installed: bool,
    pub repo_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub tag_name: String,
    pub is_prerelease: bool,
    pub has_update: bool,
    pub release_url: String,
    pub release_notes: String,
    pub asset_name: Option<String>,
    pub download_url: Option<String>,
    pub asset_size: Option<u64>,
    pub is_installer: bool,
    pub arch: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percent: u8,
}

pub fn current_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        "x64"
    }
    #[cfg(target_arch = "x86")]
    {
        "x86"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "arm64"
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    {
        "x64"
    }
}

pub fn is_installed_mode() -> bool {
    if let Ok(exe) = std::env::current_exe() {
        let s = exe.to_string_lossy().to_lowercase();
        if s.contains("windowsapps") || s.contains("hopcraft.msix") {
            return true;
        }
        if let Some(dir) = exe.parent() {
            return dir.join("uninstall.exe").exists();
        }
    }
    false
}

pub fn runtime_info() -> AppRuntimeInfo {
    AppRuntimeInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        arch: current_arch().to_string(),
        is_installed: is_installed_mode(),
        repo_url: REPO_URL.to_string(),
    }
}

#[cfg(target_os = "windows")]
pub fn show_native_update_popup(title: &str, message: &str) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let safe_title = title.replace('\'', "''");
    let safe_msg = message.replace('\'', "''");

    let ps_script = format!(
        "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null; \
         [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null; \
         $template = @\"\n\
         <toast>\n\
           <visual>\n\
             <binding template=\"ToastText02\">\n\
               <text id=\"1\">{title}</text>\n\
               <text id=\"2\">{msg}</text>\n\
             </binding>\n\
           </visual>\n\
         </toast>\n\
         \"@; \
         $xml = New-Object Windows.Data.Xml.Dom.XmlDocument; \
         $xml.LoadXml($template); \
         $toast = New-Object Windows.UI.Notifications.ToastNotification $xml; \
         [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('HopCraft').Show($toast)",
        title = safe_title,
        msg = safe_msg
    );

    let _ = Command::new("powershell")
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_script])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

#[cfg(not(target_os = "windows"))]
pub fn show_native_update_popup(_title: &str, _message: &str) {}

fn parse_semver_tag(tag: &str) -> Option<Version> {
    let trimmed = tag.trim().trim_start_matches('v').trim_start_matches('V');
    Version::parse(trimmed).ok()
}

fn select_asset<'a>(assets: &'a [GhAsset], arch: &str, prefer_package: bool) -> Option<(&'a GhAsset, bool)> {
    let arch_lower = arch.to_lowercase();
    let pkg_assets: Vec<&GhAsset> = assets
        .iter()
        .filter(|a| {
            let n = a.name.to_lowercase();
            n.ends_with(".msix") || n.ends_with(".exe")
        })
        .collect();

    let matching_arch: Vec<&GhAsset> = pkg_assets
        .iter()
        .copied()
        .filter(|a| {
            let n = a.name.to_lowercase();
            n.contains(&arch_lower) || (arch_lower == "x64" && n.contains("x86_64"))
        })
        .collect();

    let is_pkg = |a: &GhAsset| {
        let n = a.name.to_lowercase();
        n.ends_with(".msix") || n.contains("setup") || n.contains("installer")
    };

    let is_portable = |a: &GhAsset| {
        let n = a.name.to_lowercase();
        n.ends_with(".exe") && (n.contains("portable") || !is_pkg(a))
    };

    if prefer_package {
        if let Some(a) = matching_arch.iter().copied().find(|a| a.name.to_lowercase().ends_with(".msix")) {
            return Some((a, true));
        }
        if let Some(a) = matching_arch.iter().copied().find(|a| is_pkg(a)) {
            return Some((a, true));
        }
        if let Some(a) = matching_arch.iter().copied().find(|a| is_portable(a)) {
            return Some((a, false));
        }
    } else {
        if let Some(a) = matching_arch.iter().copied().find(|a| is_portable(a)) {
            return Some((a, false));
        }
        if let Some(a) = matching_arch.iter().copied().find(|a| is_pkg(a)) {
            return Some((a, true));
        }
    }

    None
}

pub async fn check_github_updates(include_prerelease: bool) -> Result<UpdateCheckResult, String> {
    let current_str = env!("CARGO_PKG_VERSION");
    let current_ver = Version::parse(current_str).map_err(|e| e.to_string())?;
    let arch = current_arch();
    let installed = is_installed_mode();

    let url = format!(
        "https://api.github.com/repos/{}/{}/releases?per_page=20",
        REPO_OWNER, REPO_NAME
    );

    let client = reqwest::Client::builder()
        .user_agent(format!("HopCraft/{}", current_str))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API HTTP {}", resp.status().as_u16()));
    }

    let releases: Vec<GhRelease> = resp.json().await.map_err(|e| e.to_string())?;

    let best = releases
        .into_iter()
        .filter(|r| !r.draft && (include_prerelease || !r.prerelease))
        .filter_map(|r| parse_semver_tag(&r.tag_name).map(|v| (v, r)))
        .max_by(|(v1, _), (v2, _)| v1.cmp(v2));

    let Some((latest_ver, release)) = best else {
        return Ok(UpdateCheckResult {
            current_version: current_str.to_string(),
            latest_version: current_str.to_string(),
            tag_name: format!("v{}", current_str),
            is_prerelease: false,
            has_update: false,
            release_url: format!("{}/releases", REPO_URL),
            release_notes: String::new(),
            asset_name: None,
            download_url: None,
            asset_size: None,
            is_installer: installed,
            arch: arch.to_string(),
        });
    };

    let has_update = latest_ver > current_ver;
    let selected = select_asset(&release.assets, arch, installed);
    let (asset_name, download_url, asset_size, is_installer) = match selected {
        Some((asset, is_inst)) => (
            Some(asset.name.clone()),
            Some(asset.browser_download_url.clone()),
            Some(asset.size),
            is_inst,
        ),
        None => (None, None, None, installed),
    };

    let notes = release
        .body
        .unwrap_or_default()
        .lines()
        .take(8)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(UpdateCheckResult {
        current_version: current_str.to_string(),
        latest_version: latest_ver.to_string(),
        tag_name: release.tag_name,
        is_prerelease: release.prerelease,
        has_update,
        release_url: release.html_url,
        release_notes: notes,
        asset_name,
        download_url,
        asset_size,
        is_installer,
        arch: arch.to_string(),
    })
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub async fn download_and_apply(
    app: tauri::AppHandle,
    window: tauri::Window,
    download_url: String,
    asset_name: String,
    is_installer: bool,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .user_agent(format!("HopCraft/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| e.to_string())?;

    let mut resp = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Download HTTP {}", resp.status().as_u16()));
    }

    let total = resp.content_length().unwrap_or(0);
    let safe_name = sanitize_filename(&asset_name);
    let temp_dir = std::env::temp_dir().join("hopcraft_update");
    std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    let target_file: PathBuf = temp_dir.join(if safe_name.is_empty() {
        "HopCraft-update.exe".to_string()
    } else {
        safe_name
    });

    let mut file = File::create(&target_file).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut last_percent: u8 = 255;

    while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        let percent = if total > 0 {
            ((downloaded.saturating_mul(100)) / total).min(100) as u8
        } else {
            0
        };
        if percent != last_percent || total == 0 {
            last_percent = percent;
            let _ = window.emit(
                "update-progress",
                DownloadProgress {
                    downloaded,
                    total,
                    percent,
                },
            );
        }
    }
    file.flush().map_err(|e| e.to_string())?;
    drop(file);

    let _ = window.emit(
        "update-progress",
        DownloadProgress {
            downloaded,
            total: if total > 0 { total } else { downloaded },
            percent: 100,
        },
    );

    if is_installer {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            if target_file
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("msix"))
                .unwrap_or(false)
            {
                let ps = format!(
                    "Add-AppxPackage -Path '{}'",
                    target_file.display().to_string().replace('\'', "''")
                );
                Command::new("powershell")
                    .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps])
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            } else {
                Command::new(&target_file)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Command::new(&target_file)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        app.exit(0);
        return Ok(());
    }

    // Portable self-replacement via hidden cmd script
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let pid = std::process::id();
    let script_path = temp_dir.join("apply_update.cmd");

    let script = format!(
        "@echo off\r\n\
         :waitloop\r\n\
         tasklist /FI \"PID eq {pid}\" 2>NUL | find /I \"{pid}\" >NUL\r\n\
         if not errorlevel 1 (\r\n\
           timeout /t 1 /nobreak >NUL\r\n\
           goto waitloop\r\n\
         )\r\n\
         copy /Y \"{src}\" \"{dst}\" >NUL\r\n\
         start \"\" \"{dst}\"\r\n\
         del /F /Q \"{src}\" >NUL 2>&1\r\n\
         del /F /Q \"%~f0\" >NUL 2>&1\r\n",
        pid = pid,
        src = target_file.display(),
        dst = current_exe.display()
    );

    std::fs::write(&script_path, script.as_bytes()).map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        Command::new("cmd")
            .args(["/C", &script_path.to_string_lossy()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("sh")
            .arg(&script_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    app.exit(0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prerelease_semver() {
        let v1 = parse_semver_tag("v1.0.0").unwrap();
        let v_pre = parse_semver_tag("v1.0.1-beta.1").unwrap();
        let v_stable = parse_semver_tag("v1.0.1").unwrap();
        assert!(v_pre > v1);
        assert!(v_stable > v_pre);
    }

    #[test]
    fn selects_portable_and_msix_assets() {
        let assets = vec![
            GhAsset {
                name: "HopCraft-windows-x64-portable.exe".to_string(),
                browser_download_url: "https://example.com/portable.exe".to_string(),
                size: 3_000_000,
            },
            GhAsset {
                name: "HopCraft-windows-x64.msix".to_string(),
                browser_download_url: "https://example.com/app.msix".to_string(),
                size: 4_000_000,
            },
        ];
        let (p_asset, p_inst) = select_asset(&assets, "x64", false).unwrap();
        assert_eq!(p_asset.name, "HopCraft-windows-x64-portable.exe");
        assert!(!p_inst);

        let (m_asset, m_inst) = select_asset(&assets, "x64", true).unwrap();
        assert_eq!(m_asset.name, "HopCraft-windows-x64.msix");
        assert!(m_inst);
    }
}
