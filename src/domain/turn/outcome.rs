use serde::{Deserialize, Serialize};

/// How a game ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// The investigators killed the professor before Nyog'Sothep appeared.
    ProfessorKilled,
    /// The investigators banished Nyog'Sothep.
    Banished,
    /// Nyog'Sothep is here and fewer than three investigators remain to banish it.
    NyogSothepReigns,
    /// Every investigator went mad or died.
    InvestigatorsLost,
}

impl Outcome {
    /// Whether the human (the investigators) won.
    pub fn investigators_win(self) -> bool {
        matches!(self, Outcome::ProfessorKilled | Outcome::Banished)
    }

    /// French sentence announcing the result.
    pub fn text(self) -> &'static str {
        match self {
            Outcome::ProfessorKilled => {
                "Victoire ! Le professeur Petersen est mort avant d'avoir pu invoquer Nyog'Sothep."
            }
            Outcome::Banished => {
                "Victoire ! Nyog'Sothep est renvoyé dans l'abîme sans fond pour de nombreuses générations."
            }
            Outcome::NyogSothepReigns => {
                "Défaite… Nyog'Sothep est parmi nous et il ne reste plus assez d'investigateurs pour le renvoyer. Son règne sera sans fin."
            }
            Outcome::InvestigatorsLost => {
                "Défaite… Tous les investigateurs ont perdu la raison ou la vie."
            }
        }
    }

    /// Reads a code written by [`Outcome::code`].
    pub fn from_code(code: &str) -> Option<Outcome> {
        [
            Outcome::ProfessorKilled,
            Outcome::Banished,
            Outcome::NyogSothepReigns,
            Outcome::InvestigatorsLost,
        ]
        .into_iter()
        .find(|outcome| outcome.code() == code)
    }

    /// The stable code stored in the database.
    pub fn code(self) -> &'static str {
        match self {
            Outcome::ProfessorKilled => "professor_killed",
            Outcome::Banished => "banished",
            Outcome::NyogSothepReigns => "nyog_sothep_reigns",
            Outcome::InvestigatorsLost => "investigators_lost",
        }
    }
}
