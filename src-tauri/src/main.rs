#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn set_click_through(window: tauri::Window, flag: bool) -> Result<(), String> {
    match window.set_ignore_cursor_events(flag) {
        Ok(_) => Ok(()),
        Err(e) => Err("123".to_string()),
    }
}
#[tauri::command]
fn set_decorations(window: tauri::Window, flag: bool) -> Result<(), String> {
    match window.set_always_on_top(flag) {
        Ok(_) => Ok(()),
        Err(e) => Err("123".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, set_click_through,set_decorations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
