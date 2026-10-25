use rand::distr::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{Rng, RngExt};
use sqlx::SqlitePool;

use crate::db::{EventRecord, GameRecord, save_state};
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::professor_ai::PetersenAi;
use crate::domain::turn::{Engine, GameState, LogLine, Side};
use crate::domain::{InvestigatorId, RandomDice};
use crate::error::AppError;
use crate::service::write_transaction::begin_write;

/// Investigators played in a game (the rules' two-player setup).
pub const INVESTIGATOR_COUNT: usize = 4;
const TOKEN_LENGTH: usize = 24;

/// A game just created: where the human plays it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedGame {
    /// Game id.
    pub game_id: i64,
    /// Secret part of the game's private URL.
    pub token: String,
}

impl CreatedGame {
    /// The game's private URL.
    pub fn path(&self) -> String {
        format!("/games/{}/{}", self.game_id, self.token)
    }
}

/// Creates a game: four of the six investigators are drawn and the game runs until the human's
/// first decision: the first moves when the AI plays the professor, the professor's setup
/// otherwise.
///
/// # Arguments
/// * `human` - the side the human plays; the computer plays the other.
pub async fn create_game(pool: &SqlitePool, human: Side) -> Result<CreatedGame, AppError> {
    let (token, state, log) = set_up(human);
    let mut transaction = begin_write(pool).await?;
    let game_id = GameRecord::insert(&mut transaction, &token, &state).await?;
    let mut game = GameRecord::load(&mut transaction, game_id, &token)
        .await?
        .ok_or(AppError::GameNotFound)?;
    save_state(&mut transaction, &mut game, &state).await?;
    EventRecord::insert_all(&mut transaction, game_id, &log).await?;
    transaction.commit().await?;
    Ok(CreatedGame { game_id, token })
}

/// Draws the investigators and the private token, then starts the game.
fn set_up(human: Side) -> (String, GameState, Vec<LogLine>) {
    let mut rng = rand::rng();
    let investigators = draw_investigators(&mut rng);
    let token: String = (0..TOKEN_LENGTH)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect();
    let mut dice = RandomDice::new();
    let mut professor = PetersenAi::new();
    let mut investigator_ai = InvestigatorsAi::new();
    let (state, log) = Engine::new_game(
        &investigators,
        human,
        &mut dice,
        &mut professor,
        &mut investigator_ai,
    );
    (token, state, log)
}

/// Four investigators at random, in the order of the counter sheet.
fn draw_investigators<R: Rng>(rng: &mut R) -> Vec<InvestigatorId> {
    let mut drawn = InvestigatorId::ALL.to_vec();
    drawn.shuffle(rng);
    drawn.truncate(INVESTIGATOR_COUNT);
    drawn.sort_by_key(|id| InvestigatorId::ALL.iter().position(|other| other == id));
    drawn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_distinct_investigators_are_drawn() {
        let mut rng = rand::rng();
        for _ in 0..20 {
            let mut drawn = draw_investigators(&mut rng);
            assert_eq!(drawn.len(), INVESTIGATOR_COUNT);
            drawn.dedup();
            assert_eq!(drawn.len(), INVESTIGATOR_COUNT);
        }
    }
}
