use crate::domain::turn::Prompt;
use crate::service::GameView;

/// The screen the page shows: one per prompt, plus the end of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Move the four investigators, on the map.
    Moves,
    /// Move the investigator who plays again, on the map.
    PlayAgain,
    /// Choose Table 2 or 3.
    Table,
    /// Name cities to the professor, on the map.
    Cities,
    /// Face the professor.
    Fight,
    /// Place Nyog'Sothep after a partial banishment, on the map.
    Banishment,
    /// The game is over.
    Finished,
}

impl Screen {
    /// The screen matching the pending prompt.
    pub fn of(view: &GameView) -> Screen {
        match &view.prompt {
            _ if view.is_finished() => Screen::Finished,
            Some(Prompt::Moves) => Screen::Moves,
            Some(Prompt::PlayAgain { .. }) => Screen::PlayAgain,
            Some(Prompt::Table { .. }) => Screen::Table,
            Some(Prompt::Cities { .. }) => Screen::Cities,
            Some(Prompt::Fight { .. }) => Screen::Fight,
            Some(Prompt::BanishmentCity) | None => Screen::Banishment,
        }
    }

    /// French title of the screen.
    pub fn title(self) -> &'static str {
        match self {
            Screen::Moves => "Déplacements",
            Screen::PlayAgain => "Rejouer",
            Screen::Table => "Événement",
            Screen::Cities => "Questions au professeur",
            Screen::Fight => "Combat",
            Screen::Banishment => "Renvoi partiel",
            Screen::Finished => "Fin de la partie",
        }
    }
}
