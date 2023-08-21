use std::env;
use std::io::{self, BufRead};

use crate::jobs::{Executable, JobKind};

mod jobs;
mod log_keys;
mod participate_records;
mod show_help_msg;
mod show_online_status;

fn read_from_stdin() -> String {
    let stdin = io::stdin();

    // Ensure that no other part of the code can read from stdin simultaneously.
    let handle = stdin.lock();

    let mut raw_log = String::new();

    for line in handle.lines() {
        raw_log.push_str(&line.unwrap());
        raw_log.push('\n');
    }

    raw_log
}

fn read_all_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path of the executable file.
    // The second argument is the name of the job.
    // The third argument is the raw log.
    match args.len() {
        0 | 1 => Vec::new(),
        2 => {
            // get stdin from the user for receiving data from pipe command.
            let raw_log = read_from_stdin();

            if raw_log.is_empty() {
                return Vec::new();
            }

            vec![args[1].clone(), raw_log]
        }
        _ => args[1..].to_vec(),
    }
}

fn main() {
    let args: Vec<String> = read_all_args();

    let (job_kind, raw_log) = if args.is_empty() {
        (JobKind::Unknown, String::new())
    } else {
        // The index `1` is absolutely safe because we have checked the length of `args`.
        (JobKind::from_arg_name(&args[0]), args[1].clone())
    };

    let job = jobs::Job {
        kind: job_kind,
        raw_log,
    };

    job.execute();
}
