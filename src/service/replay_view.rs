use sqlx::SqlitePool;

use crate::db::{GameRecord, ReplayStepRecord};
use crate::domain::turn::{Outcome, ReplayStep, Side};
use crate::error::AppError;

/// A finished game as its replay shows it: every recorded event, with every token in sight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayView {
    /// Game id.
    pub game_id: i64,
    /// Secret part of the game's private URL.
    pub token: String,
    /// The side the human played.
    pub human: Side,
    /// How the game ended.
    pub outcome: Outcome,
    /// The events, oldest first; the first one holds a board.
    pub steps: Vec<ReplayStep>,
}

impl ReplayView {
    /// Loads the replay of a game.
    ///
    /// # Errors
    /// `GameNotFound` for a wrong id or token, `GameNotOver` while the game goes on (the replay
    /// would give the professor's secrets away), or a database error.
    pub async fn load(
        pool: &SqlitePool,
        game_id: i64,
        token: &str,
    ) -> Result<ReplayView, AppError> {
        let mut connection = pool.acquire().await?;
        let game = GameRecord::load(&mut connection, game_id, token)
            .await?
            .ok_or(AppError::GameNotFound)?;
        let outcome = game.outcome()?.ok_or(AppError::GameNotOver)?;
        let human = game.human()?;
        let steps = ReplayStepRecord::load_all(&mut connection, game_id).await?;
        Ok(ReplayView {
            game_id,
            token: game.token,
            human,
            outcome,
            steps: from_first_board(steps),
        })
    }

    /// Whether the game was played before its events were recorded.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// Drops the steps before the first recorded board: a game started before the replay was
/// recorded only has the end of it, which cannot be drawn without a board to start from.
fn from_first_board(steps: Vec<ReplayStep>) -> Vec<ReplayStep> {
    let first = steps
        .iter()
        .position(|step| step.board.is_some())
        .unwrap_or(steps.len());
    steps.into_iter().skip(first).collect()
}
