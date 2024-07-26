use std::env;
use std::path::PathBuf;

pub fn path() -> PathBuf {
    let home_dir = env::home_dir().expect("Не удалось получить домашнюю директорию");
    let custom_path = home_dir.join(".local").join("share").join("cwe-client");

    custom_path
}
