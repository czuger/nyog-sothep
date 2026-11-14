use serde::{Deserialize, Serialize};

/// The kinds of counters Professor Petersen draws on Table 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterKind {
    /// Fanatiques: five cities of them let the professor summon Nyog'Sothep.
    Fanatics,
    /// Goules.
    Ghouls,
    /// Profonds, placed in ports only.
    DeepOnes,
    /// Choses de la Brume.
    ThingsOfTheMist,
    /// Habitants.
    Inhabitants,
    /// Rêves.
    Dreams,
    /// Tempête.
    Storm,
    /// Horreur volante, used by the professor before moving.
    FlyingHorror,
    /// Téléportation, used by the professor before moving.
    Teleportation,
}

impl CounterKind {
    /// Every kind, in the order of the rules' counter list.
    pub const ALL: [CounterKind; 9] = [
        CounterKind::Fanatics,
        CounterKind::Ghouls,
        CounterKind::DeepOnes,
        CounterKind::ThingsOfTheMist,
        CounterKind::Inhabitants,
        CounterKind::Dreams,
        CounterKind::FlyingHorror,
        CounterKind::Teleportation,
        CounterKind::Storm,
    ];

    /// Number of counters of this kind in the box.
    pub fn quantity(self) -> usize {
        match self {
            CounterKind::Fanatics => 10,
            CounterKind::Ghouls | CounterKind::DeepOnes => 8,
            CounterKind::Dreams => 6,
            CounterKind::Inhabitants => 4,
            CounterKind::ThingsOfTheMist => 3,
            CounterKind::Teleportation | CounterKind::Storm => 2,
            CounterKind::FlyingHorror => 1,
        }
    }

    /// Whether the professor places this counter in a city (the others are used before moving).
    pub fn is_placeable(self) -> bool {
        !matches!(self, CounterKind::FlyingHorror | CounterKind::Teleportation)
    }

    /// Whether the counter may only be placed in a port.
    pub fn port_only(self) -> bool {
        self == CounterKind::DeepOnes
    }

    /// The stable code stored in the database and used for image names.
    pub fn code(self) -> &'static str {
        match self {
            CounterKind::Fanatics => "fanatics",
            CounterKind::Ghouls => "ghouls",
            CounterKind::DeepOnes => "deep_ones",
            CounterKind::ThingsOfTheMist => "things_of_the_mist",
            CounterKind::Inhabitants => "inhabitants",
            CounterKind::Dreams => "dreams",
            CounterKind::Storm => "storm",
            CounterKind::FlyingHorror => "flying_horror",
            CounterKind::Teleportation => "teleportation",
        }
    }

    /// Reads a code written by [`CounterKind::code`].
    pub fn from_code(code: &str) -> Option<CounterKind> {
        CounterKind::ALL
            .into_iter()
            .find(|kind| kind.code() == code)
    }

    /// French name, as printed on the counter.
    pub fn name(self) -> &'static str {
        match self {
            CounterKind::Fanatics => "Fanatiques",
            CounterKind::Ghouls => "Goules",
            CounterKind::DeepOnes => "Profonds",
            CounterKind::ThingsOfTheMist => "Choses de la Brume",
            CounterKind::Inhabitants => "Habitants",
            CounterKind::Dreams => "Rêves",
            CounterKind::Storm => "Tempête",
            CounterKind::FlyingHorror => "Horreur volante",
            CounterKind::Teleportation => "Téléportation",
        }
    }

    /// French name with its article, for log sentences: « des Goules », « une Tempête ».
    pub fn with_article(self) -> &'static str {
        match self {
            CounterKind::Fanatics => "des Fanatiques",
            CounterKind::Ghouls => "des Goules",
            CounterKind::DeepOnes => "des Profonds",
            CounterKind::ThingsOfTheMist => "des Choses de la Brume",
            CounterKind::Inhabitants => "des Habitants",
            CounterKind::Dreams => "des Rêves",
            CounterKind::Storm => "une Tempête",
            CounterKind::FlyingHorror => "l'Horreur volante",
            CounterKind::Teleportation => "une Téléportation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_box_holds_44_counters() {
        let total: usize = CounterKind::ALL.iter().map(|kind| kind.quantity()).sum();
        assert_eq!(total, 44);
        for kind in CounterKind::ALL {
            assert_eq!(CounterKind::from_code(kind.code()), Some(kind));
        }
    }
}
