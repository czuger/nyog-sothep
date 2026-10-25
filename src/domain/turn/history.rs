use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::replay_step::ReplayStep;

/// What an engine wrote while it played: the public log and the replay steps.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    /// The public log lines.
    pub log: Vec<LogLine>,
    /// Every event, secrets included.
    pub replay: Vec<ReplayStep>,
}
