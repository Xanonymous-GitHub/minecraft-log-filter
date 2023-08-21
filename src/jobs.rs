/// [Job] indicates the type of job to be executed by the whole program.
pub enum JobKind {
    /// Use a command-line UI to show who is online.
    ShowOnlineStatus,
    /// Use a command-line UI to show the messages sent by players.
    /// Players who are not online will be included.
    ShowPlayerMsgs,
}

impl JobKind {
    /// Returns the name of the argument that indicates the job.
    pub fn as_arg_name(&self) -> &'static str {
        match self {
            JobKind::ShowOnlineStatus => "ps",
            JobKind::ShowPlayerMsgs => "msg",
        }
    }
}

struct Job {
    kind: JobKind,
    raw_log: String,
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
        }
    }
}