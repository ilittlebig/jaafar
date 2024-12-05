use tauri::{path::BaseDirectory, Manager};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::fs;

pub fn resolve_path(app_handle: &tauri::AppHandle, relative_path: &str) -> Result<PathBuf, String> {
    app_handle
        .path()
        .resolve(relative_path, BaseDirectory::AppData)
        .map_err(|_| format!("Failed to resolve path: {}", relative_path))
}

pub fn read_json_file<T>(path: PathBuf) -> Result<T, String> where T: DeserializeOwned {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
    serde_json::from_str(&content) .map_err(|e| format!("Failed to parse JSON in {:?}: {}", path, e))
}
