use sqlx::SqliteConnection;

use crate::error::AppError;

/// A row of `token_tilts`: the rotation a token is drawn with, and where it stood when it was drawn.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct TokenTiltRecord {
    /// The token's key.
    pub token: String,
    /// Location code of the token when its tilt was drawn.
    pub location: String,
    /// Rotation, in tenths of a degree.
    pub tenths: i64,
}

impl TokenTiltRecord {
    /// Loads the tilts of a game's tokens.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<TokenTiltRecord>, AppError> {
        let records = sqlx::query_as(
            "SELECT token, location, tenths FROM token_tilts WHERE game_id = ? ORDER BY token",
        )
        .bind(game_id)
        .fetch_all(connection)
        .await?;
        Ok(records)
    }

    /// Writes the tilt of a token, replacing the previous one.
    pub async fn save(
        &self,
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO token_tilts (game_id, token, location, tenths) VALUES (?, ?, ?, ?)
             ON CONFLICT (game_id, token)
             DO UPDATE SET location = excluded.location, tenths = excluded.tenths",
        )
        .bind(game_id)
        .bind(&self.token)
        .bind(&self.location)
        .bind(self.tenths)
        .execute(connection)
        .await?;
        Ok(())
    }

    /// Forgets the tilt of a token no longer on the board.
    pub async fn delete(
        connection: &mut SqliteConnection,
        game_id: i64,
        token: &str,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM token_tilts WHERE game_id = ? AND token = ?")
            .bind(game_id)
            .bind(token)
            .execute(connection)
            .await?;
        Ok(())
    }
}
