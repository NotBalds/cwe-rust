pub use std::fs::{File, create_dir_all as mkAllDirs};
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use crate::modules::config;

pub fn new_path(path: &str) -> PathBuf {
    config::path().join(path)
}

pub fn exist(path: &str) -> bool {
    let path = new_path(path);
    if !path.exists() {
        return false;
    }
    true 
}

pub fn echo(s: String, path: &PathBuf) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


