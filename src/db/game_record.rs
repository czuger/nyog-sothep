use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::NyogSothep;
use crate::domain::turn::{GameState, Outcome, Phase, Prompt};
use crate::error::AppError;

/// A row of `games`: the engine's cursor, the public flags and Nyog'Sothep.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameRecord {
    /// Game id.
    pub id: i64,
    /// Secret part of the game's private URL.
    pub token: String,
    /// Bumped on every save.
    pub version: i64,
    turn: i64,
    phase: String,
    prompt: Option<String>,
    outcome: Option<String>,
    nyog_city: String,
    nyog_location: Option<String>,
    nyog_joined: bool,
    nyog_target: Option<String>,
    sea_announcements: bool,
    move_in_the_open: bool,
}

impl GameRecord {
    /// Inserts a new game and returns its id.
    pub async fn insert(
        connection: &mut SqliteConnection,
        token: &str,
        state: &GameState,
    ) -> Result<i64, AppError> {
        let id = sqlx::query(
            "INSERT INTO games (token, turn, phase, nyog_city, nyog_joined, sea_announcements, move_in_the_open)
             VALUES (?, 0, '\"Moves\"', ?, 0, 0, 0)",
        )
        .bind(token)
        .bind(state.nyog.city.code())
        .execute(&mut *connection)
        .await?
        .last_insert_rowid();
        let mut record = GameRecord::load(connection, id, token)
            .await?
            .ok_or(AppError::GameNotFound)?;
        record.save(connection, state).await?;
        Ok(id)
    }

    /// Loads a game by id and token.
    pub async fn load(
        connection: &mut SqliteConnection,
        id: i64,
        token: &str,
    ) -> Result<Option<GameRecord>, AppError> {
        Ok(
            sqlx::query_as("SELECT * FROM games WHERE id = ? AND token = ?")
                .bind(id)
                .bind(token)
                .fetch_optional(connection)
                .await?,
        )
    }

    /// Writes the game's own columns from the state and bumps the version.
    pub async fn save(
        &mut self,
        connection: &mut SqliteConnection,
        state: &GameState,
    ) -> Result<(), AppError> {
        let prompt = state
            .prompt
            .as_ref()
            .map(|prompt| codec::to_json("games.prompt", prompt))
            .transpose()?;
        sqlx::query(
            "UPDATE games SET version = version + 1, turn = ?, phase = ?, prompt = ?, outcome = ?,
             nyog_city = ?, nyog_location = ?, nyog_joined = ?, nyog_target = ?,
             sea_announcements = ?, move_in_the_open = ? WHERE id = ?",
        )
        .bind(i64::from(state.turn))
        .bind(codec::to_json("games.phase", &state.phase)?)
        .bind(prompt)
        .bind(state.outcome.map(Outcome::code))
        .bind(state.nyog.city.code())
        .bind(state.nyog.location.map(|city| city.code()))
        .bind(state.nyog.joined)
        .bind(state.nyog.target.map(|city| city.code()))
        .bind(state.sea_announcements)
        .bind(state.move_in_the_open)
        .bind(self.id)
        .execute(connection)
        .await?;
        self.version += 1;
        Ok(())
    }

    /// The round number.
    pub fn turn(&self) -> u32 {
        u32::try_from(self.turn).unwrap_or(0)
    }

    /// The engine's cursor.
    pub fn phase(&self) -> Result<Phase, AppError> {
        codec::from_json("games.phase", &self.phase)
    }

    /// The decision waited for, if any.
    pub fn prompt(&self) -> Result<Option<Prompt>, AppError> {
        self.prompt
            .as_deref()
            .map(|json| codec::from_json("games.prompt", json))
            .transpose()
    }

    /// The result, once over.
    pub fn outcome(&self) -> Result<Option<Outcome>, AppError> {
        self.outcome
            .as_deref()
            .map(|code| {
                Outcome::from_code(code).ok_or_else(|| codec::corrupt("games.outcome", code))
            })
            .transpose()
    }

    /// Nyog'Sothep's state.
    pub fn nyog(&self) -> Result<NyogSothep, AppError> {
        Ok(NyogSothep {
            city: codec::location("games.nyog_city", &self.nyog_city)?,
            location: codec::optional_location(
                "games.nyog_location",
                self.nyog_location.as_deref(),
            )?,
            joined: self.nyog_joined,
            target: codec::optional_location("games.nyog_target", self.nyog_target.as_deref())?,
        })
    }

    /// The professor must announce his movements at sea.
    pub fn sea_announcements(&self) -> bool {
        self.sea_announcements
    }

    /// The professor's next move is made in the open.
    pub fn move_in_the_open(&self) -> bool {
        self.move_in_the_open
    }
}
