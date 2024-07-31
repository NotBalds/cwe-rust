pub mod prepare;
pub mod uuid;
pub mod filesystem;

use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str, t: i8) -> u16 {
    let mut sign = "[?]";

    if t == 0 {
        sign = &"[+]";
        println!("{} {}", sign.green(), msg.white());
        return 0;
    } else if t == 1 {
        sign = &"[-]";
        println!("{} {}", sign.red(), msg.red());
        panic!()
    } else if t == 2 {
        sign = &"[.]";
    } else if t == 3 {
        sign = &"[!]";
        println!("{} {}", sign.yellow(), msg.yellow());
        return 3;
    } else if t == 4 {
        sign = &"[?]";
    }

    println!("{} {}", sign.bold(), msg.white());
    0
}

pub fn unix_time() -> u64 {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let unix_time = duration.as_secs();
    unix_time
}
