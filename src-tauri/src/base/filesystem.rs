pub use std::fs::{
    File, 
    create_dir_all as mkAllDirs,
    remove_dir_all as rmDirAll
};
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use crate::modules::config;
use crate::base;

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

pub fn echo(s: String, path: &PathBuf) {
    let mut f = File::create(path)
        .expect(
            &format!("Can't create file {}", path.display())
        );
    f.write_all(s.as_bytes()).expect(
        &format!("Can't write data to file {}", path.display())
    );
}

pub fn cat(path: &Path) -> String {
    let mut f = File::open(path)
        .expect(
            &format!("Can't read file {}", path.display())
        );
    let mut s = String::new();
    let data = match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(err) => base::log(
            &err.to_string(),
            1
        ).to_string()
    };
    data
}


