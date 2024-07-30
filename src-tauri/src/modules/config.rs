use std::env;
use std::path::PathBuf;
use crate::base::filesystem;

pub fn path() -> PathBuf {
    let home_dir = env::home_dir()
        .expect("IDK what is happened, but i can't find home dir. That's weird");
    let custom_path = home_dir
        .join(".local")
        .join("share")
        .join("cwe-client");

    custom_path
}

pub fn url(path: &str) -> String {
    let base = filesystem::cat(&filesystem::new_path("server")).expect("IDK what is happened, but i can't read server url");
    base + &path.to_string()
}
