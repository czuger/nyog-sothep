use serde::{Deserialize, Serialize};

use crate::domain::turn::replay_board::ReplayBoard;

/// One event of a game as the replay shows it: what happened, the dice rolled for it and the
/// board after it. Unlike the log, it reveals the professor's secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayStep {
    /// The turn it happened.
    pub turn: u32,
    /// French description, if the step has one (a change of the board alone has none).
    pub text: Option<String>,
    /// The dice rolled since the previous step, one value per die.
    pub dice: Vec<u8>,
    /// The board after the step, only when it changed since the previous step.
    pub board: Option<ReplayBoard>,
}
