mod jobs;
mod log_keys;
mod participate_records;

use crate::jobs::JobKind;
use std::env;

fn show_usage() {
    println!("Usage: Please give me your Minecraft server log");
}

fn decide_job() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        None
    } else {
        args.get(1).map(|s| s.to_string())
    }
}

fn read_all_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Vec::new()
    } else {
        args[1..].to_vec()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_usage();
        return;
    }

    println!("Hello, {}!", args[1]);
}
