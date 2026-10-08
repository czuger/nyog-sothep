use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::{Counter, CounterKind, CounterPlace, CounterPool};
use crate::error::AppError;

/// A row of `counters`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CounterRecord {
    kind: String,
    state: String,
    city: Option<String>,
    discovered: bool,
}

impl CounterRecord {
    /// Loads every counter of a game, in box order.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<CounterPool, AppError> {
        let records: Vec<CounterRecord> = sqlx::query_as(
            "SELECT kind, state, city, discovered FROM counters WHERE game_id = ? ORDER BY number",
        )
        .bind(game_id)
        .fetch_all(connection)
        .await?;
        let counters = records
            .into_iter()
            .map(CounterRecord::into_domain)
            .collect::<Result<Vec<Counter>, AppError>>()?;
        Ok(CounterPool::from_counters(counters))
    }

    /// Replaces every counter of a game.
    pub async fn replace_all(
        connection: &mut SqliteConnection,
        game_id: i64,
        pool: &CounterPool,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM counters WHERE game_id = ?")
            .bind(game_id)
            .execute(&mut *connection)
            .await?;
        for (number, counter) in pool.all().iter().enumerate() {
            let (state, city, discovered) = match counter.place {
                CounterPlace::Reserve => ("reserve", None, false),
                CounterPlace::Hand => ("hand", None, false),
                CounterPlace::Placed { city, discovered } => {
                    ("placed", Some(city.code()), discovered)
                }
                CounterPlace::Removed => ("removed", None, false),
            };
            sqlx::query(
                "INSERT INTO counters (game_id, number, kind, state, city, discovered)
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(game_id)
            .bind(number as i64)
            .bind(counter.kind.code())
            .bind(state)
            .bind(city)
            .bind(discovered)
            .execute(&mut *connection)
            .await?;
        }
        Ok(())
    }

    fn into_domain(self) -> Result<Counter, AppError> {
        let kind = CounterKind::from_code(&self.kind)
            .ok_or_else(|| codec::corrupt("counters.kind", &self.kind))?;
        let place = match self.state.as_str() {
            "reserve" => CounterPlace::Reserve,
            "hand" => CounterPlace::Hand,
            "removed" => CounterPlace::Removed,
            "placed" => CounterPlace::Placed {
                city: codec::location("counters.city", self.city.as_deref().unwrap_or(""))?,
                discovered: self.discovered,
            },
            other => return Err(codec::corrupt("counters.state", other)),
        };
        Ok(Counter { kind, place })
    }
}
