use std::env;

use crate::jobs::JobKind;

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
    if args.is_empty() {
        show_usage();
        return;
    }

    let job_kind = JobKind::from_arg_name(&args[0]);
    let job = jobs::Job::new(job_kind, &args[1]);

    job.execute();
}
