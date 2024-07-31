#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod base;
mod modules;

#[tauri::command]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // base::prepare::run("test-passphrase".to_string(), false)
    //     .expect("Error while running prepare function");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
