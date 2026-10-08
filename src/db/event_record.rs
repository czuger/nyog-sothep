use sqlx::SqliteConnection;

use crate::domain::turn::LogLine;
use crate::error::AppError;

/// A row of `event_log`: a French line of the game log.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct EventRecord {
    /// The turn it happened.
    pub turn: i64,
    /// The text.
    pub text: String,
}

impl EventRecord {
    /// Appends log lines.
    pub async fn insert_all(
        connection: &mut SqliteConnection,
        game_id: i64,
        lines: &[LogLine],
    ) -> Result<(), AppError> {
        for line in lines {
            sqlx::query("INSERT INTO event_log (game_id, turn, text) VALUES (?, ?, ?)")
                .bind(game_id)
                .bind(i64::from(line.turn))
                .bind(&line.text)
                .execute(&mut *connection)
                .await?;
        }
        Ok(())
    }

    /// The whole log of a game, newest first.
    pub async fn newest_first(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<EventRecord>, AppError> {
        Ok(
            sqlx::query_as("SELECT turn, text FROM event_log WHERE game_id = ? ORDER BY id DESC")
                .bind(game_id)
                .fetch_all(connection)
                .await?,
        )
    }
}
