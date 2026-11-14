use crate::domain::turn::{ProfessorPrompt, Prompt};
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
    /// The professor picks his start and Nyog'Sothep's city.
    Setup,
    /// The professor may cast a Teleportation.
    Teleport,
    /// The professor moves, on the map.
    ProfessorMove,
    /// The professor may attack an investigator met on arrival.
    Attack,
    /// The professor carries an investigator away, on the map.
    Carry,
    /// The professor retreats after a fight, on the map.
    Retreat,
    /// The professor moves Nyog'Sothep, on the map.
    NyogStep,
    /// The professor may summon Nyog'Sothep.
    Summon,
    /// The professor may place a counter.
    Placement,
    /// The professor picks a counter offered by Table 1.
    Table1,
    /// The professor discards a counter.
    Discard,
    /// The professor names cities to the investigators, on the map.
    Names,
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
            Some(Prompt::Professor(prompt)) => Screen::of_professor(prompt),
            Some(Prompt::BanishmentCity) | None => Screen::Banishment,
        }
    }

    fn of_professor(prompt: &ProfessorPrompt) -> Screen {
        match prompt {
            ProfessorPrompt::Setup => Screen::Setup,
            ProfessorPrompt::Teleport { .. } => Screen::Teleport,
            ProfessorPrompt::Movement { .. } => Screen::ProfessorMove,
            ProfessorPrompt::Attack { .. } => Screen::Attack,
            ProfessorPrompt::Carry { .. } => Screen::Carry,
            ProfessorPrompt::Relocate { .. } => Screen::Retreat,
            ProfessorPrompt::NyogStep { .. } => Screen::NyogStep,
            ProfessorPrompt::Summon => Screen::Summon,
            ProfessorPrompt::Placement { .. } => Screen::Placement,
            ProfessorPrompt::Table1 { .. } => Screen::Table1,
            ProfessorPrompt::Discard { .. } => Screen::Discard,
            ProfessorPrompt::NameCities { .. } | ProfessorPrompt::NameFanaticsCities => {
                Screen::Names
            }
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
            Screen::Setup => "Mise en place",
            Screen::Teleport => "Téléportation",
            Screen::ProfessorMove => "Votre déplacement",
            Screen::Attack => "Attaque",
            Screen::Carry => "Enlèvement",
            Screen::Retreat => "Repli",
            Screen::NyogStep => "Nyog'Sothep avance",
            Screen::Summon => "Invocation",
            Screen::Placement => "Pose d'un pion",
            Screen::Table1 => "Tirage sur la table 1",
            Screen::Discard => "Défausse",
            Screen::Names => "Réponse aux investigateurs",
            Screen::Finished => "Fin de la partie",
        }
    }
}
