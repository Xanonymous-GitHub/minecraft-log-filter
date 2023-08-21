use crate::show_help_msg::show_usage;
use crate::show_online_status::show_online_status;

/// [Job] indicates the type of job to be executed by the whole program.
pub(crate) enum JobKind {
    /// Use a command-line UI to show who is online.
    ShowOnlineStatus,
    /// Use a command-line UI to show the messages sent by players.
    /// Players who are not online will be included.
    ShowPlayerMsgs,
    /// Show the help message.
    Help,
    /// Unknown job.
    Unknown,
}

impl JobKind {
    /// Returns the job kind from the argument name.
    pub fn from_arg_name(arg_name: &str) -> JobKind {
        match arg_name {
            "ps" => JobKind::ShowOnlineStatus,
            "msg" => JobKind::ShowPlayerMsgs,
            "help" => JobKind::Help,
            _ => JobKind::Unknown,
        }
    }
}

pub(crate) struct Job {
    pub(crate) kind: JobKind,
    pub(crate) raw_log: String,
}

pub(crate) trait Executable {
    fn execute(&self);
}

impl Executable for Job {
    fn execute(&self) {
        match self.kind {
            JobKind::ShowOnlineStatus => show_online_status(self.raw_log.clone()),
            JobKind::ShowPlayerMsgs => {
                todo!("Show player messages");
            }
            JobKind::Help => show_usage(),
            JobKind::Unknown => show_usage(),
        }
    }
}
