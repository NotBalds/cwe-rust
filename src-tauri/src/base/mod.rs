pub mod filesystem;
pub mod prepare;
pub mod uuid;

use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str, t: i8) {
    match t {
        0 => {
            let sign = "[+]";
            println!("{} {}", sign.green(), msg.white());
        }
        1 => {
            let sign = "[-]";
            println!("{} {}", sign.red(), msg.red());
            panic!();
        }
        2 => {
            let sign = "[.]";
            println!("{} {}", sign.bold(), msg.white());
        }
        3 => {
            let sign = "[!]";
            println!("{} {}", sign.yellow(), msg.yellow());
        }
        4 => {
            let sign = "[?]";
            println!("{} {}", sign.bold(), msg.white());
        }
        _ => {}
    }
}

pub fn unix_time() -> u64 {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let unix_time = duration.as_secs();
    unix_time
}
