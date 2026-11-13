use polodb_core::bson::doc;
use polodb_core::{CollectionT, Transaction};
use serde::{Deserialize, Serialize};

use crate::db::document_source::DocumentSource;
use crate::domain::turn::LogLine;
use crate::error::AppError;

/// A document of `log_batches`: the lines one save added to a game's log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogBatch {
    game_id: i64,
    /// The game's version once saved: orders the batches.
    version: i64,
    entries: Vec<LogEntry>,
}

/// A French line of the game log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    /// The turn it happened.
    pub turn: u32,
    /// The text.
    pub text: String,
}

impl LogBatch {
    /// The collection's name.
    pub const COLLECTION: &'static str = "log_batches";

    /// Appends the lines of a save, if there are any.
    ///
    /// # Errors
    /// A database error.
    pub fn append(
        transaction: &Transaction,
        game_id: i64,
        version: i64,
        lines: &[LogLine],
    ) -> Result<(), AppError> {
        if lines.is_empty() {
            return Ok(());
        }
        let entries = lines
            .iter()
            .map(|line| LogEntry {
                turn: line.turn,
                text: line.text.clone(),
            })
            .collect();
        transaction
            .collection::<LogBatch>(LogBatch::COLLECTION)
            .insert_one(LogBatch {
                game_id,
                version,
                entries,
            })?;
        Ok(())
    }

    /// The whole log of a game, newest first.
    ///
    /// # Errors
    /// A database error.
    pub fn newest_first(
        source: &impl DocumentSource,
        game_id: i64,
    ) -> Result<Vec<LogEntry>, AppError> {
        let mut cursor = source
            .documents::<LogBatch>(LogBatch::COLLECTION)
            .find(doc! { "game_id": game_id })
            .run()?;
        let mut batches = Vec::new();
        while cursor.advance()? {
            batches.push(cursor.deserialize_current()?);
        }
        batches.sort_by_key(|batch| std::cmp::Reverse(batch.version));
        Ok(batches
            .into_iter()
            .flat_map(|batch| batch.entries.into_iter().rev())
            .collect())
    }
}
