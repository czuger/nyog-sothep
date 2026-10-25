use serde::{Deserialize, Serialize};

/// The six investigator counters; four of them are drawn for each game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvestigatorId {
    /// Doctor Henry Armitage, librarian of Miskatonic University.
    Armitage,
    /// Randolph Carter, the dreamer.
    Carter,
    /// Thomas Malone, detective of the New York police.
    Malone,
    /// Captain Gustaf Johansen, the sailor who saw R'lyeh.
    Johansen,
    /// Jenny Barnes, the dilettante.
    Barnes,
    /// Daisy Walker, the librarian.
    Walker,
}

impl InvestigatorId {
    /// Every investigator counter.
    pub const ALL: [InvestigatorId; 6] = [
        InvestigatorId::Armitage,
        InvestigatorId::Carter,
        InvestigatorId::Malone,
        InvestigatorId::Johansen,
        InvestigatorId::Barnes,
        InvestigatorId::Walker,
    ];

    /// The stable code stored in the database and used for image names.
    pub fn code(self) -> &'static str {
        match self {
            InvestigatorId::Armitage => "armitage",
            InvestigatorId::Carter => "carter",
            InvestigatorId::Malone => "malone",
            InvestigatorId::Johansen => "johansen",
            InvestigatorId::Barnes => "barnes",
            InvestigatorId::Walker => "walker",
        }
    }

    /// Reads a code written by [`InvestigatorId::code`].
    pub fn from_code(code: &str) -> Option<InvestigatorId> {
        InvestigatorId::ALL.into_iter().find(|id| id.code() == code)
    }

    /// French name, as written at the start of a sentence.
    pub fn name(self) -> &'static str {
        match self {
            InvestigatorId::Armitage => "Le docteur Armitage",
            InvestigatorId::Carter => "Randolph Carter",
            InvestigatorId::Malone => "Thomas Malone",
            InvestigatorId::Johansen => "Le capitaine Johansen",
            InvestigatorId::Barnes => "Jenny Barnes",
            InvestigatorId::Walker => "Daisy Walker",
        }
    }

    /// French name inside a sentence (« le capitaine Johansen »).
    pub fn inner_name(self) -> &'static str {
        match self {
            InvestigatorId::Armitage => "le docteur Armitage",
            InvestigatorId::Johansen => "le capitaine Johansen",
            other => other.name(),
        }
    }

    /// French name after « de » (« de Randolph Carter », « du capitaine Johansen »).
    pub fn of_name(self) -> &'static str {
        match self {
            InvestigatorId::Armitage => "du docteur Armitage",
            InvestigatorId::Carter => "de Randolph Carter",
            InvestigatorId::Malone => "de Thomas Malone",
            InvestigatorId::Johansen => "du capitaine Johansen",
            InvestigatorId::Barnes => "de Jenny Barnes",
            InvestigatorId::Walker => "de Daisy Walker",
        }
    }

    /// Whether the investigator is a woman, for French agreement.
    pub fn is_female(self) -> bool {
        matches!(self, InvestigatorId::Barnes | InvestigatorId::Walker)
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
