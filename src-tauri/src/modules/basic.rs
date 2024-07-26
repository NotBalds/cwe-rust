use std::fs::{File, OpenOptions, create_dir_all as mkAllDirs};
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::modules::config;

    // let plain_text = "Educative Accelerates Developer Productivity";
    // let key = "Edu_key";
    
    // basic::echo("hello", &Path::new("key.txt")).unwrap_or_else(|why| {
    //     println!("! {:?}", why.kind());
    // });
    
    // match basic::cat(&Path::new("key.txt")) {
    //     Err(why) => println!("! {:?}", why.kind()),
    //     Ok(s) => println!("> {}", s),
    // }
    
    // let encrypted_text = xor(plain_text, key);
    // println!("Encrypted Text: {}", encrypted_text);

fn get_unix_time() -> String {
    let start = SystemTime::now();
    
    match start.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => 0.to_string(),
    }
}

fn gen_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn new_path(path: &str) -> PathBuf {
    config::path().join(path)
}

fn exist(path: &str) -> bool {
    let path = new_path(path);
    if !path.exists() {
        return false;
    }
    true 
}

pub fn prepare() -> io::Result<()> {
    if !exist(".") { // Checking existing of ~/.local/share/cwe/ 
        mkAllDirs(new_path("").as_path())?;
    }
    if !exist("uuid") { // Checking existing of ~/.local/share/cwe/uuid 
        let _ = echo(&gen_uuid(), &new_path("uuid"));
    }
    if !exist("keys") { // Checking existing of ~/.local/share/cwe/keys/ 
        mkAllDirs(new_path("keys").as_path())?;
    }
    println!("hello world!");
    Ok(())
}

pub fn echo(s: &str, path: &PathBuf) -> io::Result<()> {
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
