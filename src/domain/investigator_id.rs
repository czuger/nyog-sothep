use serde::{Deserialize, Serialize};

/// The six investigator counters; four of them are drawn for each game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvestigatorId {
    /// Mr Poirot.
    Poirot,
    /// Hercule.
    Hercule,
    /// Hastings.
    Hastings,
    /// The captain.
    Captain,
    /// Sandy.
    Sandy,
    /// Miss Lemon.
    Lemon,
}

impl InvestigatorId {
    /// Every investigator counter.
    pub const ALL: [InvestigatorId; 6] = [
        InvestigatorId::Poirot,
        InvestigatorId::Hercule,
        InvestigatorId::Hastings,
        InvestigatorId::Captain,
        InvestigatorId::Sandy,
        InvestigatorId::Lemon,
    ];

    /// The stable code stored in the database and used for image names.
    pub fn code(self) -> &'static str {
        match self {
            InvestigatorId::Poirot => "poirot",
            InvestigatorId::Hercule => "hercule",
            InvestigatorId::Hastings => "hastings",
            InvestigatorId::Captain => "captain",
            InvestigatorId::Sandy => "sandy",
            InvestigatorId::Lemon => "lemon",
        }
    }

    /// Reads a code written by [`InvestigatorId::code`].
    pub fn from_code(code: &str) -> Option<InvestigatorId> {
        InvestigatorId::ALL.into_iter().find(|id| id.code() == code)
    }

    /// French name, as written at the start of a sentence.
    pub fn name(self) -> &'static str {
        match self {
            InvestigatorId::Poirot => "Mr Poirot",
            InvestigatorId::Hercule => "Hercule",
            InvestigatorId::Hastings => "Hastings",
            InvestigatorId::Captain => "Le capitaine",
            InvestigatorId::Sandy => "Sandy",
            InvestigatorId::Lemon => "Miss Lemon",
        }
    }

    /// French name inside a sentence (« le capitaine »).
    pub fn inner_name(self) -> &'static str {
        match self {
            InvestigatorId::Captain => "le capitaine",
            other => other.name(),
        }
    }

    /// French name after « de » (« d'Hercule », « du capitaine »).
    pub fn of_name(self) -> &'static str {
        match self {
            InvestigatorId::Poirot => "de Mr Poirot",
            InvestigatorId::Hercule => "d'Hercule",
            InvestigatorId::Hastings => "d'Hastings",
            InvestigatorId::Captain => "du capitaine",
            InvestigatorId::Sandy => "de Sandy",
            InvestigatorId::Lemon => "de Miss Lemon",
        }
    }

    /// Whether the investigator is a woman, for French agreement.
    pub fn is_female(self) -> bool {
        matches!(self, InvestigatorId::Sandy | InvestigatorId::Lemon)
    }

    /// « il » or « elle ».
    pub fn pronoun(self) -> &'static str {
        if self.is_female() { "elle" } else { "il" }
    }

    /// The feminine ending « e », or nothing.
    pub fn agreement(self) -> &'static str {
        if self.is_female() { "e" } else { "" }
    }
}
