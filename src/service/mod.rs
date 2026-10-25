//! Use cases: one per file.

mod answer_prompt;
mod create_game;
mod game_view;
mod write_transaction;

pub use answer_prompt::answer_prompt;
pub use create_game::{CreatedGame, INVESTIGATOR_COUNT, create_game};
pub use game_view::{
    CounterView, GameView, InvestigatorView, NotebookLine, ProfessorSighting, ScoreLine,
};
pub use write_transaction::begin_write;
