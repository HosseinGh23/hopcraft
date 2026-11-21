#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod builder;
mod error;
mod parser;
mod updater;

use builder::{build_chain_config, ChainRequest, ChainResponse};
use error::AppError;
use parser::{parse_v2ray_link, LinkMeta};
use updater::{
    check_github_updates, download_and_apply, runtime_info, AppRuntimeInfo, UpdateCheckResult,
};

#[tauri::command]
fn get_app_info() -> AppRuntimeInfo {
    runtime_info()
}

#[tauri::command]
fn open_external_url(url: String) -> Result<(), String> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return Err("Invalid URL scheme".to_string());
    }
    open::that(url).map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_for_updates(include_prerelease: bool) -> Result<UpdateCheckResult, String> {
    check_github_updates(include_prerelease).await
}

#[tauri::command]
async fn download_and_install_update(
    app: tauri::AppHandle,
    window: tauri::Window,
    download_url: String,
    asset_name: String,
    is_installer: bool,
) -> Result<(), String> {
    download_and_apply(app, window, download_url, asset_name, is_installer).await
}

#[tauri::command]
fn inspect_link(uri: String) -> Result<LinkMeta, AppError> {
    let (_, meta) = parse_v2ray_link(&uri, "tmp")?;
    Ok(meta)
}

#[tauri::command]
fn build_chain(req: ChainRequest) -> Result<ChainResponse, AppError> {
    build_chain_config(&req)
}

#[tauri::command]
fn read_clipboard() -> Result<String, String> {
    let mut cb = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    cb.get_text().map_err(|e| e.to_string())
}

#[tauri::command]
fn write_clipboard(text: String) -> Result<(), String> {
    let mut cb = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    cb.set_text(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_json_file(
    content: String,
    title: String,
    filter_json: String,
    filter_all: String,
) -> Result<Option<String>, String> {
    let file = rfd::FileDialog::new()
        .set_title(&title)
        .set_file_name("config.json")
        .add_filter(&filter_json, &["json"])
        .add_filter(&filter_all, &["*"])
        .save_file();

    if let Some(path) = file {
        std::fs::write(&path, content.as_bytes()).map_err(|e| e.to_string())?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("config.json")
            .to_string();
        return Ok(Some(name));
    }
    Ok(None)
}

#[tauri::command]
fn set_window_title(window: tauri::Window, title: String) -> Result<(), String> {
    window.set_title(&title).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_window_theme(window: tauri::Window, theme: String) -> Result<(), String> {
    let t = match theme.as_str() {
        "light" => Some(tauri::Theme::Light),
        _ => Some(tauri::Theme::Dark),
    };
    window.set_theme(t).map_err(|e| e.to_string())
}

#[tauri::command]
fn show_main_window(window: tauri::Window) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    let _ = window.set_focus();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            open_external_url,
            check_for_updates,
            download_and_install_update,
            inspect_link,
            build_chain,
            read_clipboard,
            write_clipboard,
            save_json_file,
            set_window_title,
            set_window_theme,
            show_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running HopCraft");
}
