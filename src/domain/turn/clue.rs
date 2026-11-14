use serde::{Deserialize, Serialize};

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;

/// Something the investigators learned about the professor, noted in their notebook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clue {
    /// The turn it was learned.
    pub turn: u32,
    /// What was learned.
    pub kind: ClueKind,
}

/// The kinds of clues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClueKind {
    /// He was seen there.
    Spotted {
        /// His position.
        at: LocationId,
    },
    /// He named these cities, one of which is his position.
    NamedCities {
        /// The named cities.
        cities: Vec<LocationId>,
    },
    /// He said he is at sea.
    AtSea,
    /// From now on he announces his movements at sea.
    SeaAnnouncements,
    /// He arrived in a city holding an investigator: one of these.
    MetInvestigator {
        /// The cities holding an investigator at that moment.
        cities: Vec<LocationId>,
    },
    /// After a fight there, he left for a neighbouring location.
    Retreated {
        /// Where the fight took place.
        from: LocationId,
    },
    /// He used the Flying Horror: he may be anywhere within three cities.
    FlyingHorror,
    /// He teleported an investigator: he is at most two cities from them.
    Teleported {
        /// Where the investigator stood.
        near: LocationId,
    },
    /// He will make his next move in the open.
    MoveInTheOpen,
    /// The number of Fanatics counters in place.
    FanaticsCount {
        /// The count.
        count: usize,
    },
    /// He named two cities, one of which holds Fanatics.
    FanaticsAmong {
        /// The named cities.
        cities: Vec<LocationId>,
    },
    /// No Fanatics are in place.
    NoFanatics,
    /// Whether a city holds an encounter.
    Encounter {
        /// The city asked about.
        city: LocationId,
        /// Whether a counter is placed there.
        present: bool,
    },
    /// The encounter of a city, specified.
    EncounterKind {
        /// The city asked about.
        city: LocationId,
        /// The counter there, if any.
        kind: Option<CounterKind>,
    },
    /// Whether Nyog'Sothep is to appear in one of two cities.
    NyogSothepAmong {
        /// The named cities.
        cities: Vec<LocationId>,
        /// Whether one of them is the appearance city.
        present: bool,
    },
    /// Whether he stood within a few steps of a location.
    Within {
        /// The location the distance is counted from.
        from: LocationId,
        /// The most steps.
        steps: u8,
        /// Whether he stood that close.
        inside: bool,
    },
    /// Nyog'Sothep was summoned there.
    Summoned {
        /// The appearance city.
        city: LocationId,
    },
}

impl ClueKind {
    /// French sentence for the notebook.
    pub fn describe(&self) -> String {
        match self {
            ClueKind::Spotted { at } => format!("Le professeur est repéré {}.", at.at_phrase()),
            ClueKind::NamedCities { cities } => {
                format!("Le professeur est à {}.", alternatives(cities))
            }
            ClueKind::AtSea => "Le professeur est en mer.".to_string(),
            ClueKind::SeaAnnouncements => {
                "Le professeur doit désormais annoncer ses déplacements en mer.".to_string()
            }
            ClueKind::MetInvestigator { cities } => format!(
                "Le professeur arrive dans une ville où se trouve un investigateur : {}.",
                alternatives(cities)
            ),
            ClueKind::Retreated { from } => format!(
                "Après le combat {}, le professeur s'est replié d'un cran.",
                from.at_phrase()
            ),
            ClueKind::FlyingHorror => {
                "Le professeur utilise l'Horreur volante : il peut être à trois villes de là."
                    .to_string()
            }
            ClueKind::Teleported { near } => format!(
                "Le professeur a téléporté un investigateur : il était à moins de trois villes de {}.",
                near
            ),
            ClueKind::MoveInTheOpen => {
                "Le professeur fera son prochain déplacement à découvert.".to_string()
            }
            ClueKind::FanaticsCount { count } => {
                format!("{count} groupe(s) de Fanatiques sont en place.")
            }
            ClueKind::FanaticsAmong { cities } => {
                format!("Des Fanatiques sont à {}.", alternatives(cities))
            }
            ClueKind::NoFanatics => "Aucun groupe de Fanatiques n'est en place.".to_string(),
            ClueKind::Encounter { city, present } => match present {
                true => format!("Il y a une rencontre à {city}."),
                false => format!("Il n'y a pas de rencontre à {city}."),
            },
            ClueKind::EncounterKind { city, kind } => match kind {
                Some(kind) => format!("La rencontre de {city} : {}.", kind.name()),
                None => format!("Il n'y a pas de rencontre à {city}."),
            },
            ClueKind::Within {
                from,
                steps,
                inside,
            } => match inside {
                true => format!("Le professeur est à {steps} villes ou moins de {from}."),
                false => format!("Le professeur est à plus de {steps} villes de {from}."),
            },
            ClueKind::NyogSothepAmong { cities, present } => match present {
                true => format!("Nyog'Sothep doit apparaître à {}.", alternatives(cities)),
                false => format!("Nyog'Sothep n'apparaîtra ni à {}.", both(cities)),
            },
            ClueKind::Summoned { city } => format!("Nyog'Sothep est apparu à {city}."),
        }
    }
}

/// « Boston ou Lynn », « Boston, Lynn ou Arkham ».
fn alternatives(cities: &[LocationId]) -> String {
    join(cities, " ou ")
}

/// « Boston ni à Lynn ».
fn both(cities: &[LocationId]) -> String {
    join(cities, " ni à ")
}

fn join(cities: &[LocationId], last_separator: &str) -> String {
    let names: Vec<String> = cities.iter().map(|city| city.to_string()).collect();
    match names.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{}{last_separator}{last}", rest.join(", ")),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_cities_read_as_alternatives() {
        let clue = ClueKind::NamedCities {
            cities: vec![LocationId::Boston, LocationId::Lynn, LocationId::Arkham],
        };
        assert_eq!(
            clue.describe(),
            "Le professeur est à Boston, Lynn ou Arkham."
        );
        let clue = ClueKind::NyogSothepAmong {
            cities: vec![LocationId::Boston, LocationId::Lynn],
            present: false,
        };
        assert_eq!(
            clue.describe(),
            "Nyog'Sothep n'apparaîtra ni à Boston ni à Lynn."
        );
    }
}
