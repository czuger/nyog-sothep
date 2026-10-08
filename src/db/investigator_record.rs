use sqlx::SqliteConnection;

use crate::db::codec;
use crate::domain::{Investigator, InvestigatorId, InvestigatorStatus};
use crate::error::AppError;

/// A row of `investigators`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct InvestigatorRecord {
    code: String,
    san: i64,
    weapon: bool,
    sign: bool,
    medallion: bool,
    spell: bool,
    location: String,
    previous: String,
    status: String,
    skip_turns: i64,
    skips_action: bool,
    car_broken: bool,
    forbidden_city: Option<String>,
    free_border: bool,
    blocked_crossing: Option<String>,
    seen_nyog: bool,
    fanatics_killed: i64,
    damage_dealt: i64,
    banished: bool,
}

impl InvestigatorRecord {
    /// Loads the investigators of a game, in seat order.
    pub async fn load_all(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<Vec<Investigator>, AppError> {
        let records: Vec<InvestigatorRecord> =
            sqlx::query_as("SELECT * FROM investigators WHERE game_id = ? ORDER BY seat")
                .bind(game_id)
                .fetch_all(connection)
                .await?;
        records
            .into_iter()
            .map(InvestigatorRecord::into_domain)
            .collect()
    }

    /// Replaces the investigators of a game.
    pub async fn replace_all(
        connection: &mut SqliteConnection,
        game_id: i64,
        investigators: &[Investigator],
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM investigators WHERE game_id = ?")
            .bind(game_id)
            .execute(&mut *connection)
            .await?;
        for (seat, investigator) in investigators.iter().enumerate() {
            insert(connection, game_id, seat as i64, investigator).await?;
        }
        Ok(())
    }

    fn into_domain(self) -> Result<Investigator, AppError> {
        let number = |column: &'static str, value: i64| {
            u32::try_from(value).map_err(|_| codec::corrupt(column, &value.to_string()))
        };
        Ok(Investigator {
            id: InvestigatorId::from_code(&self.code)
                .ok_or_else(|| codec::corrupt("investigators.code", &self.code))?,
            san: i32::try_from(self.san)
                .map_err(|_| codec::corrupt("investigators.san", &self.san.to_string()))?,
            weapon: self.weapon,
            sign: self.sign,
            medallion: self.medallion,
            spell: self.spell,
            location: codec::location("investigators.location", &self.location)?,
            previous: codec::location("investigators.previous", &self.previous)?,
            status: InvestigatorStatus::from_code(&self.status)
                .ok_or_else(|| codec::corrupt("investigators.status", &self.status))?,
            skip_turns: u8::try_from(self.skip_turns).map_err(|_| {
                codec::corrupt("investigators.skip_turns", &self.skip_turns.to_string())
            })?,
            skips_action: self.skips_action,
            car_broken: self.car_broken,
            forbidden_city: codec::optional_location(
                "investigators.forbidden_city",
                self.forbidden_city.as_deref(),
            )?,
            free_border: self.free_border,
            blocked_crossing: codec::optional_location(
                "investigators.blocked_crossing",
                self.blocked_crossing.as_deref(),
            )?,
            seen_nyog: self.seen_nyog,
            fanatics_killed: number("investigators.fanatics_killed", self.fanatics_killed)?,
            damage_dealt: number("investigators.damage_dealt", self.damage_dealt)?,
            banished: self.banished,
        })
    }
}

async fn insert(
    connection: &mut SqliteConnection,
    game_id: i64,
    seat: i64,
    investigator: &Investigator,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO investigators (game_id, seat, code, san, weapon, sign, medallion, spell,
         location, previous, status, skip_turns, skips_action, car_broken, forbidden_city,
         free_border, blocked_crossing, seen_nyog, fanatics_killed, damage_dealt, banished)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(game_id)
    .bind(seat)
    .bind(investigator.id.code())
    .bind(i64::from(investigator.san))
    .bind(investigator.weapon)
    .bind(investigator.sign)
    .bind(investigator.medallion)
    .bind(investigator.spell)
    .bind(investigator.location.code())
    .bind(investigator.previous.code())
    .bind(investigator.status.code())
    .bind(i64::from(investigator.skip_turns))
    .bind(investigator.skips_action)
    .bind(investigator.car_broken)
    .bind(investigator.forbidden_city.map(|city| city.code()))
    .bind(investigator.free_border)
    .bind(investigator.blocked_crossing.map(|city| city.code()))
    .bind(investigator.seen_nyog)
    .bind(i64::from(investigator.fanatics_killed))
    .bind(i64::from(investigator.damage_dealt))
    .bind(investigator.banished)
    .execute(connection)
    .await?;
    Ok(())
}
