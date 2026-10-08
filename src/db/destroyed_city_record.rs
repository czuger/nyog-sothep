use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::LocationId;
use crate::error::AppError;

/// A row of `destroyed_cities`.
pub struct DestroyedCityRecord;

impl DestroyedCityRecord {
    /// Loads the destroyed cities of a game, in order.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<LocationId>, AppError> {
        let codes: Vec<String> = sqlx::query_scalar(
            "SELECT city FROM destroyed_cities WHERE game_id = ? ORDER BY position",
        )
        .bind(game_id)
        .fetch_all(connection)
        .await?;
        codes
            .iter()
            .map(|code| codec::location("destroyed_cities.city", code))
            .collect()
    }

    /// Writes the cities not stored yet; destruction is never undone.
    pub async fn append_new(
        connection: &mut SqliteConnection,
        game_id: i64,
        cities: &[LocationId],
    ) -> Result<(), AppError> {
        let stored: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM destroyed_cities WHERE game_id = ?")
                .bind(game_id)
                .fetch_one(&mut *connection)
                .await?;
        for (position, city) in cities.iter().enumerate().skip(stored as usize) {
            sqlx::query("INSERT INTO destroyed_cities (game_id, position, city) VALUES (?, ?, ?)")
                .bind(game_id)
                .bind(position as i64)
                .bind(city.code())
                .execute(&mut *connection)
                .await?;
        }
        Ok(())
    }
}
