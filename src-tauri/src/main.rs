#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
mod base;

use modules::{network, crypting};
use base::filesystem;

fn main() {
    let uuid = filesystem::cat(
        &filesystem::new_path("uuid")
        ).unwrap();
    let status_code = network::register(uuid, "Hello".to_string());
    println!("{}", status_code);

    let enc = crypting::xor("Hello world!".to_string(), "HUI".to_string());

    println!("{}", enc);

    let keys = crypting::gen_keys("hello world!".to_string());
    println!("{}", keys.0);
    println!("{}", keys.1);

    // let _ = basic::prepare();

	// tauri::Builder::default()
	// 	.run(tauri::generate_context!())
	// 	.expect("error while running tauri application");
}
