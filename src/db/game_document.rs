use polodb_core::bson::{Bson, doc, to_bson};
use polodb_core::{CollectionT, Transaction};
use serde::{Deserialize, Serialize};

use crate::db::document_source::DocumentSource;
use crate::db::log_batch::LogBatch;
use crate::db::replay_batch::ReplayBatch;
use crate::db::sequence;
use crate::db::tilt_document::TiltDocument;
use crate::domain::turn::{GameState, History};
use crate::error::AppError;

/// A document of `games`: a whole game, its state embedded as it is, with the tilts of its
/// tokens. Its log and replay grow in their own collections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameDocument {
    /// Game id.
    #[serde(rename = "_id")]
    pub id: i64,
    /// Secret part of the game's private URL.
    pub token: String,
    /// 1 when created, bumped on every save.
    pub version: i64,
    /// Everything the engine needs to play on, secrets included.
    pub state: GameState,
    /// The rotations the tokens are drawn with.
    pub tilts: Vec<TiltDocument>,
}

impl GameDocument {
    /// The collection's name.
    pub const COLLECTION: &'static str = "games";

    /// Stores a new game under the next game id.
    ///
    /// # Errors
    /// A database error.
    pub fn create(
        transaction: &Transaction,
        token: String,
        state: GameState,
        tilts: Vec<TiltDocument>,
    ) -> Result<GameDocument, AppError> {
        let game = GameDocument {
            id: sequence::next_number(transaction, GameDocument::COLLECTION)?,
            token,
            version: 1,
            state,
            tilts,
        };
        transaction
            .collection::<GameDocument>(GameDocument::COLLECTION)
            .insert_one(&game)?;
        Ok(game)
    }

    /// Loads a game by id and token.
    ///
    /// # Errors
    /// A database error.
    pub fn find(
        source: &impl DocumentSource,
        id: i64,
        token: &str,
    ) -> Result<Option<GameDocument>, AppError> {
        Ok(source
            .documents::<GameDocument>(GameDocument::COLLECTION)
            .find_one(doc! { "_id": id, "token": token })?)
    }

    /// Writes the state and the tilts, and bumps the version.
    ///
    /// # Errors
    /// A database error.
    pub fn save(&mut self, transaction: &Transaction) -> Result<(), AppError> {
        self.version += 1;
        let update = doc! {
            "$set": {
                "version": self.version,
                "state": encode(&self.state)?,
                "tilts": encode(&self.tilts)?,
            }
        };
        transaction
            .collection::<GameDocument>(GameDocument::COLLECTION)
            .update_one(doc! { "_id": self.id }, update)?;
        Ok(())
    }

    /// Appends what an engine wrote to the game's log and replay, as of the current version.
    ///
    /// # Errors
    /// A database error.
    pub fn record_history(
        &self,
        transaction: &Transaction,
        history: &History,
    ) -> Result<(), AppError> {
        LogBatch::append(transaction, self.id, self.version, &history.log)?;
        ReplayBatch::append(transaction, self.id, self.version, &history.replay)
    }
}

fn encode<T: Serialize>(value: &T) -> Result<Bson, AppError> {
    Ok(to_bson(value).map_err(polodb_core::Error::from)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Store;
    use crate::domain::investigator_ai::InvestigatorsAi;
    use crate::domain::professor_ai::PetersenAi;
    use crate::domain::turn::{
        Answer, Engine, MoveOrder, ProfessorAnswer, ProfessorPrompt, Prompt, RuleSet, Side,
    };
    use crate::domain::{InvestigatorId, LocationId, RandomDice};

    const IDS: [InvestigatorId; 4] = [
        InvestigatorId::Armitage,
        InvestigatorId::Malone,
        InvestigatorId::Johansen,
        InvestigatorId::Walker,
    ];

    /// Stores a new game and checks that it reads back unchanged.
    fn store_new(store: &Store, state: &GameState) -> GameDocument {
        let game = store
            .write(|transaction| {
                GameDocument::create(transaction, "token".to_string(), state.clone(), Vec::new())
            })
            .unwrap();
        let found = GameDocument::find(store.database(), game.id, "token").unwrap();
        assert_eq!(found.as_ref(), Some(&game));
        game
    }

    #[test]
    fn a_state_survives_a_round_trip() {
        let folder = tempfile::tempdir().unwrap();
        let store = Store::open(folder.path()).unwrap();
        let mut dice = RandomDice::seeded(11);
        let mut ai = PetersenAi::seeded(11);
        let mut investigators = InvestigatorsAi::seeded(11);
        let (state, _) = Engine::new_game(
            &IDS,
            Side::Investigators,
            RuleSet::Original,
            &mut dice,
            &mut ai,
            &mut investigators,
        );
        let mut game = store_new(&store, &state);
        assert_eq!(store_new(&store, &state).id, game.id + 1);

        let sail = (0..4)
            .map(|index| (index, MoveOrder::Go(LocationId::NantucketSound)))
            .collect();
        Engine::new(&mut game.state, &mut dice, &mut ai, &mut investigators)
            .answer(Answer::Moves(sail))
            .unwrap();
        game.state.destroyed.push(LocationId::Boston);
        game.tilts.push(TiltDocument {
            token: "nyog".to_string(),
            location: LocationId::Boston,
            tenths: -12,
        });
        store.write(|transaction| game.save(transaction)).unwrap();
        let found = GameDocument::find(store.database(), game.id, "token")
            .unwrap()
            .unwrap();
        assert_eq!(found.version, 2);
        assert_eq!(found, game);
        assert!(
            GameDocument::find(store.database(), game.id, "wrong")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn a_professor_game_survives_a_round_trip() {
        let folder = tempfile::tempdir().unwrap();
        let store = Store::open(folder.path()).unwrap();
        let mut dice = RandomDice::seeded(3);
        let mut ai = PetersenAi::seeded(3);
        let mut investigators = InvestigatorsAi::seeded(3);
        let (mut state, _) = Engine::new_game(
            &IDS,
            Side::Professor,
            RuleSet::Original,
            &mut dice,
            &mut ai,
            &mut investigators,
        );
        assert_eq!(
            state.prompt,
            Some(Prompt::Professor(ProfessorPrompt::Setup))
        );
        store_new(&store, &state);

        let setup = ProfessorAnswer::Setup {
            start: LocationId::Worcester,
            nyog_city: LocationId::Concord,
        };
        Engine::new(&mut state, &mut dice, &mut ai, &mut investigators)
            .answer(Answer::Professor(setup))
            .unwrap();
        assert!(matches!(state.prompt, Some(Prompt::Professor(_))));
        store_new(&store, &state);
    }

    #[test]
    fn a_failed_write_leaves_nothing_behind() {
        let folder = tempfile::tempdir().unwrap();
        let store = Store::open(folder.path()).unwrap();
        let (state, history) = Engine::new_game(
            &IDS,
            Side::Investigators,
            RuleSet::Original,
            &mut RandomDice::seeded(4),
            &mut PetersenAi::seeded(4),
            &mut InvestigatorsAi::seeded(4),
        );
        let refused: Result<(), AppError> = store.write(|transaction| {
            let game = GameDocument::create(transaction, "token".to_string(), state, Vec::new())?;
            game.record_history(transaction, &history)?;
            Err(AppError::GameNotFound)
        });
        assert!(refused.is_err());
        assert_eq!(
            GameDocument::find(store.database(), 1, "token").unwrap(),
            None
        );
        assert!(
            LogBatch::newest_first(store.database(), 1)
                .unwrap()
                .is_empty()
        );
    }
}
