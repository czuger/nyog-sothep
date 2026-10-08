use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::turn::{Clue, ClueKind};
use crate::error::AppError;

/// A row of `clues`: a line of the investigators' notebook.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClueRecord {
    turn: i64,
    kind: String,
}

impl ClueRecord {
    /// Loads the notebook of a game, in order.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<Clue>, AppError> {
        let records: Vec<ClueRecord> =
            sqlx::query_as("SELECT turn, kind FROM clues WHERE game_id = ? ORDER BY position")
                .bind(game_id)
                .fetch_all(connection)
                .await?;
        records
            .into_iter()
            .map(|record| {
                Ok(Clue {
                    turn: u32::try_from(record.turn).unwrap_or(0),
                    kind: codec::from_json::<ClueKind>("clues.kind", &record.kind)?,
                })
            })
            .collect()
    }

    /// Writes the clues not stored yet; the notebook only ever grows.
    pub async fn append_new(
        connection: &mut SqliteConnection,
        game_id: i64,
        clues: &[Clue],
    ) -> Result<(), AppError> {
        let stored: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM clues WHERE game_id = ?")
            .bind(game_id)
            .fetch_one(&mut *connection)
            .await?;
        for (position, clue) in clues.iter().enumerate().skip(stored as usize) {
            sqlx::query("INSERT INTO clues (game_id, position, turn, kind) VALUES (?, ?, ?, ?)")
                .bind(game_id)
                .bind(position as i64)
                .bind(i64::from(clue.turn))
                .bind(codec::to_json("clues.kind", &clue.kind)?)
                .execute(&mut *connection)
                .await?;
        }
        Ok(())
    }
}
