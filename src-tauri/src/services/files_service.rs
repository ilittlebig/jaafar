use tauri::{path::BaseDirectory, Manager};
use std::path::PathBuf;

pub fn resolve_path(app_handle: &tauri::AppHandle, relative_path: &str) -> Result<PathBuf, String> {
    app_handle
        .path()
        .resolve(relative_path, BaseDirectory::AppData)
        .map_err(|_| format!("Failed to resolve path: {}", relative_path))
}
