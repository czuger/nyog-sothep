use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::turn::{ReplayBoard, ReplayStep};
use crate::error::AppError;

/// A row of `replay_steps`: one event of a game, for its replay.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ReplayStepRecord {
    turn: i64,
    text: Option<String>,
    dice: String,
    board: Option<String>,
}

impl ReplayStepRecord {
    /// Appends replay steps.
    pub async fn insert_all(
        connection: &mut SqliteConnection,
        game_id: i64,
        steps: &[ReplayStep],
    ) -> Result<(), AppError> {
        for step in steps {
            let board = step
                .board
                .as_ref()
                .map(|board| codec::to_json("replay_steps.board", board))
                .transpose()?;
            sqlx::query(
                "INSERT INTO replay_steps (game_id, turn, text, dice, board) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(game_id)
            .bind(i64::from(step.turn))
            .bind(&step.text)
            .bind(codec::to_json("replay_steps.dice", &step.dice)?)
            .bind(board)
            .execute(&mut *connection)
            .await?;
        }
        Ok(())
    }

    /// Every step of a game, oldest first.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<ReplayStep>, AppError> {
        let records: Vec<ReplayStepRecord> = sqlx::query_as(
            "SELECT turn, text, dice, board FROM replay_steps WHERE game_id = ? ORDER BY id",
        )
        .bind(game_id)
        .fetch_all(connection)
        .await?;
        records
            .into_iter()
            .map(ReplayStepRecord::into_step)
            .collect()
    }

    fn into_step(self) -> Result<ReplayStep, AppError> {
        Ok(ReplayStep {
            turn: u32::try_from(self.turn).unwrap_or(0),
            text: self.text,
            dice: codec::from_json("replay_steps.dice", &self.dice)?,
            board: self
                .board
                .map(|json| codec::from_json::<ReplayBoard>("replay_steps.board", &json))
                .transpose()?,
        })
    }
}
