#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
mod base;

fn main() {
    base::prepare::run(
        "Hello world!".to_string(), 
        true
    ).expect("Error while running prepare function");

    let sys_public_key = base::filesystem::cat(
        &base::filesystem::new_path("base-keys/sys-key.pub")
    );
    println!("PubKey: {}", sys_public_key);
    
    base::log(
        &format!("PubKey: {}", 
            modules::crypting::public_from_pem_to_base64(sys_public_key)),
        3);
    println!("UUID: {}", base::uuid::get());
    println!("UNIX TIME: {}", base::unix_time().to_string());
    println!("Sign: {}", modules::crypting::sign(
            base::unix_time().to_string(),
            "Hello world!".to_string()
        ));

    println!("Get Status code: {}", modules::network::get(
        base::uuid::get(),
        base::unix_time().to_string(),
        modules::crypting::sign(
            base::unix_time().to_string(),
            "Hello world!".to_string()
        )
    ));

	tauri::Builder::default()
		.run(tauri::generate_context!())
	    .expect("error while running tauri application");
}
