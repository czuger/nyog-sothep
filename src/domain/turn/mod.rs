//! The turn engine: a pure state machine that plays the rounds and pauses on the human's
//! decisions.

mod action_phase;
mod answer_error;
mod clue;
mod engine;
mod fights;
mod game_state;
mod log_line;
mod move_options;
mod moves;
mod nyog_turn;
mod outcome;
mod phase;
mod professor;
mod professor_brain;
mod professor_turn;
mod prompt;
mod questions;
mod sanity;
mod setup;

pub use answer_error::AnswerError;
pub use clue::{Clue, ClueKind};
pub use engine::Engine;
pub use game_state::GameState;
pub use log_line::LogLine;
pub use move_options::MoveOptions;
pub use outcome::Outcome;
pub use phase::{ActionStep, Phase};
pub use professor::{Professor, RouteStage};
pub use professor_brain::{ProfessorBrain, ProfessorMove, ProfessorMoveOptions, Teleport};
pub use prompt::{Answer, MoveOrder, Prompt};

#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod scripted_professor;
