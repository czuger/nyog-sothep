//! Application errors.

use thiserror::Error;

use crate::domain::turn::AnswerError;

/// Everything that can go wrong while handling a request.
///
/// The `Display` text is shown to the player, so it is written in French.
#[derive(Debug, Error)]
pub enum AppError {
    /// The database failed.
    #[error("Erreur de base de données.")]
    Database(#[from] sqlx::Error),
    /// The database schema could not be migrated.
    #[error("Impossible de mettre à jour la base de données.")]
    Migration(#[from] sqlx::migrate::MigrateError),
    /// The folder holding the database file could not be created.
    #[error("Impossible de créer le dossier de la base de données.")]
    DatabaseFolder(#[from] std::io::Error),
    /// A page could not be rendered.
    #[error("Impossible d'afficher la page.")]
    Template(#[from] askama::Error),
    /// A stored value does not match any known variant.
    #[error("Donnée invalide en base ({column} = {value}).")]
    CorruptData {
        /// The column holding the value.
        column: &'static str,
        /// The value that could not be read.
        value: String,
    },
    /// No game has this id and token.
    #[error("Cette partie n'existe pas.")]
    GameNotFound,
    /// The replay of a game is only shown once it is over.
    #[error("Le rejeu n'est disponible qu'une fois la partie terminée.")]
    GameNotOver,
    /// A form field could not be read.
    #[error("Choix invalide : {0}.")]
    InvalidForm(String),
    /// The turn engine refused the answer.
    #[error(transparent)]
    Answer(#[from] AnswerError),
}

impl AppError {
    /// Returns `true` for server-side failures, as opposed to invalid player requests.
    pub fn is_internal(&self) -> bool {
        matches!(
            self,
            AppError::Database(_)
                | AppError::Migration(_)
                | AppError::DatabaseFolder(_)
                | AppError::Template(_)
                | AppError::CorruptData { .. }
        )
    }
}
