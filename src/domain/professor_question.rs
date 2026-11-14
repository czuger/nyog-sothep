use serde::{Deserialize, Serialize};

/// What an event asks of the professor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessorQuestion {
    /// He names this many cities, one of which is his position (or says he is at sea).
    NameCities(u8),
    /// He reveals his position.
    RevealPosition,
    /// He reveals his position and the number of Fanatics counters in place.
    RevealPositionAndFanatics,
    /// He reveals himself and makes his next move in the open.
    RevealAndMoveInTheOpen,
    /// From now on he announces his movements at sea.
    AnnounceSeaMoves,
    /// He names two cities, one of which holds Fanatics.
    NameFanaticsCities,
    /// The investigator names two cities; he says whether they hold encounters.
    EncountersInTwoCities,
    /// The investigator names two cities; he says whether Nyog'Sothep is to appear in one.
    NyogSothepInTwoCities {
        /// The investigator may decline to ask.
        optional: bool,
    },
    /// The investigator names a city; he specifies its encounter, if any.
    EncounterInOneCity,
    /// He says whether he stands three cities or less from the investigator.
    WithinThreeCities,
    /// He says in how many cities Fanatics are in place.
    CountFanaticsCities,
}

impl ProfessorQuestion {
    /// How many cities the investigator names, when the human must choose them.
    pub fn cities_named_by_investigator(self) -> Option<usize> {
        match self {
            ProfessorQuestion::EncountersInTwoCities
            | ProfessorQuestion::NyogSothepInTwoCities { .. } => Some(2),
            ProfessorQuestion::EncounterInOneCity => Some(1),
            _ => None,
        }
    }
}
