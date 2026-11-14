use std::collections::HashMap;

use rand::RngExt;

use crate::db::TiltDocument;
use crate::domain::turn::GameState;
use crate::domain::{CounterKind, InvestigatorId, LocationId};
use crate::service::game_view::last_sighting;

/// Largest tilt of a token, in tenths of a degree either way.
const MAX_TILT_TENTHS: i32 = 50;

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
    tenths: HashMap<String, i32>,
}

impl TokenTilts {
    /// The tilts stored with a game.
    pub fn of_documents(documents: &[TiltDocument]) -> TokenTilts {
        TokenTilts {
            tenths: documents
                .iter()
                .map(|document| (document.token.clone(), document.tenths))
                .collect(),
        }
    }

    /// The rotation of a token, in degrees (none for a token never tilted yet).
    pub fn of(&self, token: TiltedToken) -> f64 {
        self.tenths
            .get(&token.key())
            .map_or(0.0, |tenths| f64::from(*tenths) / 10.0)
    }

    /// Drops the tilt of a token the human must not follow.
    pub fn forget(&mut self, token: TiltedToken) {
        self.tenths.remove(&token.key());
    }
}

/// The tilts to store with a game: kept for the tokens that stayed put, drawn again for those
/// that moved or appeared since `stored`, and dropped for those that left the board.
pub fn refresh_tilts(stored: &[TiltDocument], state: &GameState) -> Vec<TiltDocument> {
    let mut rng = rand::rng();
    placements(state)
        .into_iter()
        .map(|(token, location)| {
            let key = token.key();
            stored
                .iter()
                .find(|document| document.token == key && document.location == location)
                .cloned()
                .unwrap_or_else(|| TiltDocument {
                    token: key,
                    location,
                    tenths: rng.random_range(-MAX_TILT_TENTHS..=MAX_TILT_TENTHS),
                })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::RandomDice;
    use crate::domain::investigator_ai::InvestigatorsAi;
    use crate::domain::professor_ai::PetersenAi;
    use crate::domain::turn::{Engine, RuleSet, Side};

    const IDS: [InvestigatorId; 4] = [
        InvestigatorId::Armitage,
        InvestigatorId::Malone,
        InvestigatorId::Johansen,
        InvestigatorId::Walker,
    ];

    #[test]
    fn tilts_are_kept_until_the_token_moves() {
        let (mut state, _) = Engine::new_game(
            &IDS,
            Side::Investigators,
            RuleSet::Original,
            &mut RandomDice::seeded(2),
            &mut PetersenAi::seeded(2),
            &mut InvestigatorsAi::seeded(2),
        );
        let first = refresh_tilts(&[], &state);
        assert!(
            first
                .iter()
                .all(|document| document.tenths.abs() <= MAX_TILT_TENTHS)
        );
        let walker = TiltedToken::Investigator(InvestigatorId::Walker).key();
        assert!(first.iter().any(|document| document.token == walker));

        state.investigators[3].location = LocationId::NantucketSound;
        let second = refresh_tilts(&first, &state);
        assert_eq!(first.len(), second.len());
        for (before, after) in first.iter().zip(&second) {
            if after.token == walker {
                assert_eq!(after.location, LocationId::NantucketSound);
            } else {
                assert_eq!(before, after);
            }
        }
    }

    #[test]
    fn tokens_off_the_board_lose_their_tilt() {
        let (state, _) = Engine::new_game(
            &IDS,
            Side::Investigators,
            RuleSet::Original,
            &mut RandomDice::seeded(2),
            &mut PetersenAi::seeded(2),
            &mut InvestigatorsAi::seeded(2),
        );
        let gone = TiltDocument {
            token: TiltedToken::Destroyed(LocationId::Boston).key(),
            location: LocationId::Boston,
            tenths: 7,
        };
        let refreshed = refresh_tilts(std::slice::from_ref(&gone), &state);
        assert!(!refreshed.contains(&gone));
    }
}
