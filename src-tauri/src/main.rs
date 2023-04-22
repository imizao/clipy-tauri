// main.rs
use tauri::Manager;
use serde::{Serialize, Deserialize};
use livesplit_hotkey::*;
#[derive(Serialize, Deserialize, Debug)]
struct Greet {
    name: String
}

fn main() {
    let hook = Hook::new().expect("Failed to create hook");

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    hook.register(KeyCode::KeyC.with_modifiers(Modifiers::META), move || {
        // println!("C");
        // 发送事件到web
        let _ = tx.send(());
    })
    .unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let callback_app_handle = app.app_handle().clone();
           
            let _ = std::thread::spawn(move || {
                loop {
                    if let Ok(()) = rx.recv() {
                        callback_app_handle.emit_all("clipboard", ()).expect("Failed to emit event");
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
