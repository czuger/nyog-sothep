use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError};

use polodb_core::bson::doc;
use polodb_core::{CollectionT, Database, IndexModel, Transaction};

use crate::db::log_batch::LogBatch;
use crate::db::replay_batch::ReplayBatch;
use crate::error::AppError;

/// The PoloDB database, shared by every request.
///
/// Writes are serialized: PoloDB's transactions lock what they write but not what they read, so
/// two answers to the same game would otherwise both play from the same state.
#[derive(Clone)]
pub struct Store {
    inner: Arc<StoreInner>,
}

struct StoreInner {
    database: Database,
    writes: Mutex<()>,
}

impl Store {
    /// Opens the database folder, creating it if needed, and its indexes.
    ///
    /// # Errors
    /// Fails when the folder cannot be created or the database cannot be opened, for instance
    /// because another process holds it.
    pub fn open(path: &Path) -> Result<Store, AppError> {
        std::fs::create_dir_all(path)?;
        let database = Database::open_path(path)?;
        create_indexes(&database)?;
        Ok(Store {
            inner: Arc::new(StoreInner {
                database,
                writes: Mutex::new(()),
            }),
        })
    }

    /// The database, for reads outside a transaction.
    pub fn database(&self) -> &Database {
        &self.inner.database
    }

    /// Runs `work` in a transaction, alone among writers, and commits it when it succeeds.
    ///
    /// # Errors
    /// The error of `work` (the transaction is then rolled back), or a database error.
    pub fn write<T>(
        &self,
        work: impl FnOnce(&Transaction) -> Result<T, AppError>,
    ) -> Result<T, AppError> {
        // The lock guards no data, so a writer that panicked leaves nothing inconsistent behind.
        let _alone = self
            .inner
            .writes
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let transaction = self.inner.database.start_transaction()?;
        match work(&transaction) {
            Ok(value) => {
                transaction.commit()?;
                Ok(value)
            }
            Err(error) => {
                if let Err(rollback) = transaction.rollback() {
                    tracing::error!(%rollback, "could not roll back a failed write");
                }
                Err(error)
            }
        }
    }

    /// Runs blocking database work off the async runtime's threads.
    ///
    /// # Errors
    /// The error of `work`, or `Task` when it panicked.
    pub async fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce(&Store) -> Result<T, AppError> + Send + 'static,
    ) -> Result<T, AppError> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || work(&store)).await?
    }
}

/// The batches are looked up by game.
fn create_indexes(database: &Database) -> Result<(), AppError> {
    for collection in [LogBatch::COLLECTION, ReplayBatch::COLLECTION] {
        database
            .collection::<polodb_core::bson::Document>(collection)
            .create_index(IndexModel {
                keys: doc! { "game_id": 1 },
                options: None,
            })?;
    }
    Ok(())
}
