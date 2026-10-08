use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::turn::Professor;
use crate::error::AppError;

/// The row of `professor`: hit points and position.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProfessorRecord {
    hp: i64,
    location: String,
}

impl ProfessorRecord {
    /// Loads the professor of a game.
    pub async fn load(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Professor, AppError> {
        let record: ProfessorRecord =
            sqlx::query_as("SELECT hp, location FROM professor WHERE game_id = ?")
                .bind(game_id)
                .fetch_one(connection)
                .await?;
        Ok(Professor {
            hp: i32::try_from(record.hp)
                .map_err(|_| codec::corrupt("professor.hp", &record.hp.to_string()))?,
            location: codec::location("professor.location", &record.location)?,
        })
    }

    /// Writes the professor of a game.
    pub async fn save(
        connection: &mut SqliteConnection,
        game_id: i64,
        professor: &Professor,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO professor (game_id, hp, location) VALUES (?, ?, ?)
             ON CONFLICT (game_id) DO UPDATE SET hp = excluded.hp, location = excluded.location",
        )
        .bind(game_id)
        .bind(i64::from(professor.hp))
        .bind(professor.location.code())
        .execute(connection)
        .await?;
        Ok(())
    }
}
