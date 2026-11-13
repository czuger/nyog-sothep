use polodb_core::bson::doc;
use polodb_core::{CollectionT, Transaction};
use serde::{Deserialize, Serialize};

use crate::db::document_source::DocumentSource;
use crate::domain::turn::ReplayStep;
use crate::error::AppError;

/// A document of `replay_batches`: the replay steps one save added to a game. Kept apart from
/// the log since their boards are only read by the replay of a finished game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayBatch {
    game_id: i64,
    /// The game's version once saved: orders the batches.
    version: i64,
    steps: Vec<ReplayStep>,
}

impl ReplayBatch {
    /// The collection's name.
    pub const COLLECTION: &'static str = "replay_batches";

    /// Appends the steps of a save, if there are any.
    ///
    /// # Errors
    /// A database error.
    pub fn append(
        transaction: &Transaction,
        game_id: i64,
        version: i64,
        steps: &[ReplayStep],
    ) -> Result<(), AppError> {
        if steps.is_empty() {
            return Ok(());
        }
        transaction
            .collection::<ReplayBatch>(ReplayBatch::COLLECTION)
            .insert_one(ReplayBatch {
                game_id,
                version,
                steps: steps.to_vec(),
            })?;
        Ok(())
    }

    /// Every step of a game, oldest first.
    ///
    /// # Errors
    /// A database error.
    pub fn oldest_first(
        source: &impl DocumentSource,
        game_id: i64,
    ) -> Result<Vec<ReplayStep>, AppError> {
        let mut cursor = source
            .documents::<ReplayBatch>(ReplayBatch::COLLECTION)
            .find(doc! { "game_id": game_id })
            .run()?;
        let mut batches = Vec::new();
        while cursor.advance()? {
            batches.push(cursor.deserialize_current()?);
        }
        batches.sort_by_key(|batch| batch.version);
        Ok(batches.into_iter().flat_map(|batch| batch.steps).collect())
    }
}
