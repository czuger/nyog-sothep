use std::collections::HashMap;

use rand::RngExt;
use sqlx::SqliteConnection;

use crate::db::TokenTiltRecord;
use crate::domain::turn::GameState;
use crate::domain::{CounterKind, InvestigatorId, LocationId};
use crate::error::AppError;
use crate::service::game_view::last_sighting;

/// Largest tilt of a token, in tenths of a degree either way.
const MAX_TILT_TENTHS: i64 = 50;

/// A token drawn tilted on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiltedToken {
    /// An investigator.
    Investigator(InvestigatorId),
    /// The professor, where he really stands.
    Professor,
    /// The professor's last sighting.
    Sighting,
    /// Nyog'Sothep.
    Nyog,
    /// A counter placed in a city.
    Counter(LocationId, CounterKind),
    /// A city destroyed by Nyog'Sothep.
    Destroyed(LocationId),
}

impl TiltedToken {
    /// The key the tilt is stored under.
    pub fn key(self) -> String {
        match self {
            TiltedToken::Investigator(id) => format!("investigator:{}", id.code()),
            TiltedToken::Professor => "professor".to_string(),
            TiltedToken::Sighting => "sighting".to_string(),
            TiltedToken::Nyog => "nyog".to_string(),
            TiltedToken::Counter(city, kind) => format!("counter:{}:{}", city.code(), kind.code()),
            TiltedToken::Destroyed(city) => format!("destroyed:{}", city.code()),
        }
    }
}

/// The rotations the tokens of a game are drawn with, kept until they move.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenTilts {
    tenths: HashMap<String, i64>,
}

impl TokenTilts {
    /// Loads the tilts of a game.
    pub async fn load(
        connection: &mut SqliteConnection,
        game_id: i64,
    ) -> Result<TokenTilts, AppError> {
        let records = TokenTiltRecord::load_all(connection, game_id).await?;
        Ok(TokenTilts {
            tenths: records
                .into_iter()
                .map(|record| (record.token, record.tenths))
                .collect(),
        })
    }

    /// The rotation of a token, in degrees (none for a token never tilted yet).
    pub fn of(&self, token: TiltedToken) -> f64 {
        self.tenths
            .get(&token.key())
            .map_or(0.0, |tenths| *tenths as f64 / 10.0)
    }

    /// Drops the tilt of a token the human must not follow.
    pub fn forget(&mut self, token: TiltedToken) {
        self.tenths.remove(&token.key());
    }
}

/// Draws a new tilt for every token that moved or appeared since the last save, and forgets
/// the tokens that left the board.
///
/// # Errors
/// A database error.
pub async fn refresh_tilts(
    connection: &mut SqliteConnection,
    game_id: i64,
    state: &GameState,
) -> Result<(), AppError> {
    let stored = TokenTiltRecord::load_all(connection, game_id).await?;
    let placements = placements(state);
    for record in new_tilts(&stored, &placements) {
        record.save(connection, game_id).await?;
    }
    for record in &stored {
        if !placements
            .iter()
            .any(|(token, _)| token.key() == record.token)
        {
            TokenTiltRecord::delete(connection, game_id, &record.token).await?;
        }
    }
    Ok(())
}

/// A new random tilt for every token that moved; drawn before any write since the generator
/// cannot be held across an `.await`.
fn new_tilts(
    stored: &[TokenTiltRecord],
    placements: &[(TiltedToken, LocationId)],
) -> Vec<TokenTiltRecord> {
    let mut rng = rand::rng();
    moved(stored, placements)
        .into_iter()
        .map(|(token, location)| TokenTiltRecord {
            token: token.key(),
            location: location.code().to_string(),
            tenths: rng.random_range(-MAX_TILT_TENTHS..=MAX_TILT_TENTHS),
        })
        .collect()
}

/// Every token on the board and where it stands, the secret ones included.
fn placements(state: &GameState) -> Vec<(TiltedToken, LocationId)> {
    let mut placements: Vec<_> = state
        .investigators
        .iter()
        .map(|investigator| {
            (
                TiltedToken::Investigator(investigator.id),
                investigator.location,
            )
        })
        .collect();
    placements.push((TiltedToken::Professor, state.professor.location));
    if let Some((at, _)) = last_sighting(state) {
        placements.push((TiltedToken::Sighting, at));
    }
    if let Some(at) = state.nyog.location {
        placements.push((TiltedToken::Nyog, at));
    }
    placements.extend(
        state
            .counters
            .placed()
            .map(|(city, counter)| (TiltedToken::Counter(city, counter.kind), city)),
    );
    placements.extend(
        state
            .destroyed
            .iter()
            .map(|city| (TiltedToken::Destroyed(*city), *city)),
    );
    placements
}

/// The tokens whose stored tilt was drawn elsewhere, or that have none yet.
fn moved(
    stored: &[TokenTiltRecord],
    placements: &[(TiltedToken, LocationId)],
) -> Vec<(TiltedToken, LocationId)> {
    placements
        .iter()
        .filter(|(token, location)| {
            !stored
                .iter()
                .any(|record| record.token == token.key() && record.location == location.code())
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{GameRecord, connect};
    use crate::domain::RandomDice;
    use crate::domain::investigator_ai::InvestigatorsAi;
    use crate::domain::professor_ai::PetersenAi;
    use crate::domain::turn::{Engine, Side};

    const IDS: [InvestigatorId; 4] = [
        InvestigatorId::Armitage,
        InvestigatorId::Malone,
        InvestigatorId::Johansen,
        InvestigatorId::Walker,
    ];

    #[test]
    fn only_moved_or_new_tokens_get_a_new_tilt() {
        let stored = vec![
            TokenTiltRecord {
                token: TiltedToken::Investigator(InvestigatorId::Armitage).key(),
                location: LocationId::Boston.code().to_string(),
                tenths: 25,
            },
            TokenTiltRecord {
                token: TiltedToken::Professor.key(),
                location: LocationId::Worcester.code().to_string(),
                tenths: -10,
            },
        ];
        let placements = [
            (
                TiltedToken::Investigator(InvestigatorId::Armitage),
                LocationId::Boston,
            ),
            (TiltedToken::Professor, LocationId::Lynn),
            (TiltedToken::Nyog, LocationId::Concord),
        ];
        assert_eq!(
            moved(&stored, &placements),
            vec![
                (TiltedToken::Professor, LocationId::Lynn),
                (TiltedToken::Nyog, LocationId::Concord),
            ]
        );
    }

    #[tokio::test]
    async fn tilts_are_kept_until_the_token_moves() {
        let pool = connect("sqlite::memory:").await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let (mut state, _) = Engine::new_game(
            &IDS,
            Side::Investigators,
            &mut RandomDice::seeded(2),
            &mut PetersenAi::seeded(2),
            &mut InvestigatorsAi::seeded(2),
        );
        let game_id = GameRecord::insert(&mut connection, "token", &state)
            .await
            .unwrap();
        refresh_tilts(&mut connection, game_id, &state)
            .await
            .unwrap();
        let first = TokenTiltRecord::load_all(&mut connection, game_id)
            .await
            .unwrap();
        assert!(
            first
                .iter()
                .all(|record| record.tenths.abs() <= MAX_TILT_TENTHS)
        );
        assert!(
            first
                .iter()
                .any(|record| record.token == "investigator:walker")
        );

        state.investigators[3].location = LocationId::NantucketSound;
        refresh_tilts(&mut connection, game_id, &state)
            .await
            .unwrap();
        let second = TokenTiltRecord::load_all(&mut connection, game_id)
            .await
            .unwrap();
        assert_eq!(first.len(), second.len());
        for (before, after) in first.iter().zip(&second) {
            if after.token == "investigator:walker" {
                assert_eq!(after.location, LocationId::NantucketSound.code());
            } else {
                assert_eq!(before, after);
            }
        }
    }
}
