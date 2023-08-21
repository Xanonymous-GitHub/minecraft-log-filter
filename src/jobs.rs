/// [Job] indicates the type of job to be executed by the whole program.
pub enum JobKind {
    /// Use a command-line UI to show who is online.
    ShowOnlineStatus,
    /// Use a command-line UI to show the messages sent by players.
    /// Players who are not online will be included.
    ShowPlayerMsgs,
    /// Unknown job.
    Unknown,
}

impl JobKind {
    /// Returns the job kind from the argument name.
    pub fn from_arg_name(arg_name: &str) -> JobKind {
        match arg_name {
            "ps" => JobKind::ShowOnlineStatus,
            "msg" => JobKind::ShowPlayerMsgs,
            _ => JobKind::Unknown,
        }
    }
}

pub(crate) struct Job {
    pub(crate) kind: JobKind,
    pub(crate) raw_log: String,
}

pub trait Executable {
    fn execute(&self);
}

impl Executable for Job {
    fn execute(&self) {
        match self.kind {
            JobKind::ShowOnlineStatus => {
                todo!("Show online status");
            }
            JobKind::ShowPlayerMsgs => {
                todo!("Show player messages");
            }
            JobKind::Unknown => {
                todo!("Unknown job");
            }
        }
    }
}
