#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use modules::{network, basic};
use std::collections::HashMap;

fn main() {
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("uuid", "test");
    body.insert("publickey", "test");
    let _ = network::post("http://bald.su:1337/register", body);

    let _ = basic::prepare();

	// tauri::Builder::default()
	// 	.run(tauri::generate_context!())
	// 	.expect("error while running tauri application");
}
