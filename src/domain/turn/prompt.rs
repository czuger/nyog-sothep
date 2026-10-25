use serde::{Deserialize, Serialize};

use crate::domain::event_table::EventTable;
use crate::domain::fight_choice::FightChoice;
use crate::domain::location_id::LocationId;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::turn::professor_prompt::{ProfessorAnswer, ProfessorPrompt};
use crate::domain::turn::side::Side;

/// A decision one side must take before the game goes on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Prompt {
    /// Move the four investigators.
    Moves,
    /// Choose Table 2 or 3 for an investigator's event.
    Table {
        /// The investigator.
        investigator: usize,
    },
    /// Name cities to the professor.
    Cities {
        /// The investigator asking.
        investigator: usize,
        /// The question.
        question: ProfessorQuestion,
    },
    /// Move an investigator who plays again.
    PlayAgain {
        /// The investigator.
        investigator: usize,
    },
    /// Choose how to face the professor.
    Fight {
        /// The investigator fighting.
        investigator: usize,
        /// The choices open to them.
        options: Vec<FightChoice>,
    },
    /// Choose the city where Nyog'Sothep is placed after a partial banishment.
    BanishmentCity,
    /// A decision of the professor.
    Professor(ProfessorPrompt),
}

impl Prompt {
    /// The side that must answer.
    pub fn side(&self) -> Side {
        match self {
            Prompt::Professor(_) => Side::Professor,
            _ => Side::Investigators,
        }
    }
}

/// An investigator's move order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveOrder {
    /// Stay in place.
    Stay,
    /// Stay in place and consult a psychoanalyst (+1d6 SAN, nothing else this turn).
    Psychoanalyst,
    /// Move to a neighbouring location.
    Go(LocationId),
}

/// The answer to a [`Prompt`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    /// Moves for the investigators, by index.
    Moves(Vec<(usize, MoveOrder)>),
    /// The chosen event table.
    Table(EventTable),
    /// The named cities (none to decline an optional question).
    Cities(Vec<LocationId>),
    /// The « play again » move.
    PlayAgain(MoveOrder),
    /// The fight choice.
    Fight(FightChoice),
    /// Where Nyog'Sothep goes.
    BanishmentCity(LocationId),
    /// The professor's answer.
    Professor(ProfessorAnswer),
}
