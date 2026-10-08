use thiserror::Error;

/// Why an answer was refused. The texts are shown to the player, in French.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AnswerError {
    /// The game is over.
    #[error("La partie est terminée.")]
    GameOver,
    /// The answer does not match the question asked.
    #[error("Cette action n'est pas possible à ce moment de la partie.")]
    Unexpected,
    /// An investigator who must move has no order.
    #[error("Choisissez le déplacement de {0}.")]
    MissingMove(&'static str),
    /// The move is not allowed.
    #[error("Déplacement impossible pour {0}.")]
    IllegalMove(&'static str),
    /// Wrong number of cities.
    #[error("Citez {0} ville(s) différente(s).")]
    WrongCityCount(usize),
    /// A named place is not a city.
    #[error("Seules des villes peuvent être citées.")]
    NotACity,
    /// The fight choice is not open to the investigator.
    #[error("Ce choix de combat n'est pas possible.")]
    IllegalFightChoice,
}
