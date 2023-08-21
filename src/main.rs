use std::env;

use crate::jobs::{Executable, JobKind};

mod jobs;
mod log_keys;
mod participate_records;
mod show_help_msg;
mod show_online_status;

fn read_all_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path of the executable file.
    // The second argument is the name of the job.
    // The third argument is the raw log.
    if args.len() < 3 {
        Vec::new()
    } else {
        args[1..].to_vec()
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
