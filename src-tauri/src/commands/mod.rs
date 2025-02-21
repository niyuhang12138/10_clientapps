use crate::utils;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_app_dir() -> String {
    utils::app_dir().to_string_lossy().to_string()
}
