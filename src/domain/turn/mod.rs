//! The turn engine: a pure state machine that plays the rounds and pauses on the human's
//! decisions, while the computer answers the prompts of its own side.

mod action_phase;
mod answer_error;
mod clue;
mod computer_players;
mod engine;
mod fights;
mod game_state;
mod history;
mod investigator_brain;
mod log_line;
mod move_options;
mod moves;
mod nyog_turn;
mod outcome;
mod phase;
mod professor;
mod professor_answers;
mod professor_brain;
mod professor_counters;
mod professor_prompt;
mod professor_turn;
mod prompt;
mod public_state;
mod questions;
mod recorded_dice;
mod replay;
mod replay_board;
mod replay_step;
mod sanity;
mod setup;
mod side;

pub use answer_error::AnswerError;
pub use clue::{Clue, ClueKind};
pub use engine::Engine;
pub use game_state::GameState;
pub use history::History;
pub use investigator_brain::InvestigatorBrain;
pub use log_line::LogLine;
pub use move_options::MoveOptions;
pub use outcome::Outcome;
pub use phase::{ActionStep, Phase};
pub use professor::{Professor, RouteStage};
pub use professor_brain::{ProfessorBrain, ProfessorMove, ProfessorMoveOptions, Teleport};
pub use professor_prompt::{ProfessorAnswer, ProfessorPrompt};
pub use prompt::{Answer, MoveOrder, Prompt};
pub use replay_board::{ReplayBoard, ReplayCounter, ReplayInvestigator};
pub use replay_step::ReplayStep;
pub use side::Side;

#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod scripted_professor;
