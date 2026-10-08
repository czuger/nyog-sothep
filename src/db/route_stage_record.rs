use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::CounterKind;
use crate::domain::turn::RouteStage;
use crate::error::AppError;

/// A row of `route_sheet`: one of the professor's stage notes.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RouteStageRecord {
    turn: i64,
    location: String,
    counter: Option<String>,
}

impl RouteStageRecord {
    /// Loads the route sheet of a game, in order.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<RouteStage>, AppError> {
        let records: Vec<RouteStageRecord> = sqlx::query_as(
            "SELECT turn, location, counter FROM route_sheet WHERE game_id = ? ORDER BY position",
        )
        .bind(game_id)
        .fetch_all(connection)
        .await?;
        records
            .into_iter()
            .map(|record| {
                let counter = record
                    .counter
                    .as_deref()
                    .map(|code| {
                        CounterKind::from_code(code)
                            .ok_or_else(|| codec::corrupt("route_sheet.counter", code))
                    })
                    .transpose()?;
                Ok(RouteStage {
                    turn: u32::try_from(record.turn).unwrap_or(0),
                    location: codec::location("route_sheet.location", &record.location)?,
                    counter,
                })
            })
            .collect()
    }

    /// Writes the stages not stored yet; the route sheet only ever grows.
    pub async fn append_new(
        connection: &mut SqliteConnection,
        game_id: i64,
        stages: &[RouteStage],
    ) -> Result<(), AppError> {
        let stored: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM route_sheet WHERE game_id = ?")
            .bind(game_id)
            .fetch_one(&mut *connection)
            .await?;
        for (position, stage) in stages.iter().enumerate().skip(stored as usize) {
            sqlx::query(
                "INSERT INTO route_sheet (game_id, position, turn, location, counter) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(game_id)
            .bind(position as i64)
            .bind(i64::from(stage.turn))
            .bind(stage.location.code())
            .bind(stage.counter.map(CounterKind::code))
            .execute(&mut *connection)
            .await?;
        }
        Ok(())
    }
}
