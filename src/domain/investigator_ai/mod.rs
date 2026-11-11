//! The investigators' AI, for games where the human plays Professor Petersen.
//!
//! It only reads [`GameState::public_copy`](crate::domain::turn::GameState::public_copy): what
//! the investigators may know.

mod banishers;
mod banishment_choice;
mod fight_preference;
mod hunt;
mod investigator_weights;
mod investigators_ai;
mod move_choice;
mod paths;
mod question_choice;
mod table_choice;

pub use investigator_weights::InvestigatorWeights;
pub use investigators_ai::InvestigatorsAi;
