use rand::distr::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{Rng, RngExt};

use crate::db::{GameDocument, Store};
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::professor_ai::PetersenAi;
use crate::domain::turn::{Engine, GameState, History, RuleSet, Side};
use crate::domain::{InvestigatorId, RandomDice};
use crate::error::AppError;
use crate::service::token_tilts::refresh_tilts;

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
    /// The game's private URL, relative to the application's root (no leading `/`).
    pub fn path(&self) -> String {
        format!("games/{}/{}", self.game_id, self.token)
    }
}

/// Creates a game: four of the six investigators are drawn and the game runs until the human's
/// first decision: the first moves when the AI plays the professor, the professor's setup
/// otherwise.
///
/// # Arguments
/// * `human` - the side the human plays; the computer plays the other.
/// * `rules` - the rules the game is played with.
///
/// # Errors
/// A database error.
pub async fn create_game(
    store: &Store,
    human: Side,
    rules: RuleSet,
) -> Result<CreatedGame, AppError> {
    let (token, state, history) = set_up(human, rules);
    let tilts = refresh_tilts(&[], &state);
    store
        .run(move |store| {
            store.write(|transaction| {
                let game = GameDocument::create(transaction, token, state, tilts)?;
                game.record_history(transaction, &history)?;
                Ok(CreatedGame {
                    game_id: game.id,
                    token: game.token,
                })
            })
        })
        .await
}

/// Draws the investigators and the private token, then starts the game.
fn set_up(human: Side, rules: RuleSet) -> (String, GameState, History) {
    let mut rng = rand::rng();
    let investigators = draw_investigators(&mut rng);
    let token: String = (0..TOKEN_LENGTH)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect();
    let mut dice = RandomDice::new();
    let mut professor = PetersenAi::new();
    let mut investigator_ai = InvestigatorsAi::new();
    let (state, history) = Engine::new_game(
        &investigators,
        human,
        rules,
        &mut dice,
        &mut professor,
        &mut investigator_ai,
    );
    (token, state, history)
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
