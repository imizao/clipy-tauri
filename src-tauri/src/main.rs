#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use livesplit_hotkey::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let hook = Hook::new().expect("Failed to create hook");
    hook.register(KeyCode::KeyC.with_modifiers(Modifiers::META), || {
        println!("C")
    })
    .unwrap();
    hook.register(KeyCode::KeyX.with_modifiers(Modifiers::META), || {
        println!("X")
    })
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
