use serde::{Deserialize, Serialize};

/// Whether an investigator is still in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestigatorStatus {
    /// Playing.
    Active,
    /// Lost all their SAN and became a Fanatics counter.
    Mad,
    /// Killed by Nyog'Sothep.
    Dead,
}

impl InvestigatorStatus {
    /// The stable code stored in the database.
    pub fn code(self) -> &'static str {
        match self {
            InvestigatorStatus::Active => "active",
            InvestigatorStatus::Mad => "mad",
            InvestigatorStatus::Dead => "dead",
        }
    }

    /// Reads a code written by [`InvestigatorStatus::code`].
    pub fn from_code(code: &str) -> Option<InvestigatorStatus> {
        [
            InvestigatorStatus::Active,
            InvestigatorStatus::Mad,
            InvestigatorStatus::Dead,
        ]
        .into_iter()
        .find(|status| status.code() == code)
    }
}
