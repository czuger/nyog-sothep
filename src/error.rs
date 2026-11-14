//! Application errors.

use thiserror::Error;

use crate::domain::turn::AnswerError;

/// Everything that can go wrong while handling a request.
///
/// The `Display` text is shown to the player, so it is written in French.
#[derive(Debug, Error)]
pub enum AppError {
    /// The database failed, or a stored document could not be read back.
    #[error("Erreur de base de données.")]
    Database(#[from] polodb_core::Error),
    /// The folder holding the database could not be created.
    #[error("Impossible de créer le dossier de la base de données.")]
    DatabaseFolder(#[from] std::io::Error),
    /// The blocking database work panicked.
    #[error("Erreur interne du serveur.")]
    Task(#[from] tokio::task::JoinError),
    /// A page could not be rendered.
    #[error("Impossible d'afficher la page.")]
    Template(#[from] askama::Error),
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
                | AppError::DatabaseFolder(_)
                | AppError::Task(_)
                | AppError::Template(_)
        )
    }
}
