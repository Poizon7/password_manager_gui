#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::ClipboardManager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn copy(app_handle: tauri::AppHandle, name: String) {
    app_handle.clipboard_manager().write_text(name).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![copy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
