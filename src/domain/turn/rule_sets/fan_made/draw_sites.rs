//! The fan-made draw tables: the professor rolls 1d6 on the table of the place he stands.

use crate::domain::counter_kind::CounterKind::{self, *};
use crate::domain::location_id::LocationId;
use crate::domain::travel;

/// Roads a city needs to be a crossroads.
const CROSSROADS_ROADS: usize = 4;

/// The kinds of places, each with its draw table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrawSite {
    /// A city of four roads or more.
    Crossroads,
    /// Any other port.
    Port,
    /// Any other city.
    Town,
    /// A sea zone.
    Sea,
}

impl DrawSite {
    /// The kind of a place.
    pub(crate) fn of(location: LocationId) -> DrawSite {
        if location.is_sea() {
            DrawSite::Sea
        } else if travel::road_neighbours(location).len() >= CROSSROADS_ROADS {
            DrawSite::Crossroads
        } else if location.is_port() {
            DrawSite::Port
        } else {
            DrawSite::Town
        }
    }

    /// French name of the table.
    pub(crate) fn name(self) -> &'static str {
        match self {
            DrawSite::Crossroads => "carrefour",
            DrawSite::Port => "port",
            DrawSite::Town => "bourg isolé",
            DrawSite::Sea => "mer",
        }
    }

    /// The counter drawn with a roll (1 to 6) in `location`, if any.
    pub(crate) fn counter(self, roll: u8, location: LocationId) -> Option<CounterKind> {
        let ghouls_or_deep_ones = match location.is_port() {
            true => DeepOnes,
            false => Ghouls,
        };
        let row = match self {
            DrawSite::Crossroads => [
                Inhabitants,
                ghouls_or_deep_ones,
                Dreams,
                Dreams,
                Teleportation,
                Storm,
            ],
            DrawSite::Port => [Storm, DeepOnes, DeepOnes, DeepOnes, ThingsOfTheMist, Dreams],
            DrawSite::Town => [
                ThingsOfTheMist,
                Ghouls,
                Ghouls,
                Dreams,
                Inhabitants,
                FlyingHorror,
            ],
            DrawSite::Sea => return SEA_ROW[usize::from(roll.clamp(1, 6) - 1)],
        };
        Some(row[usize::from(roll.clamp(1, 6) - 1)])
    }
}

const SEA_ROW: [Option<CounterKind>; 6] = [None, None, None, None, Some(Storm), Some(DeepOnes)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::map;

    #[test]
    fn the_places_split_as_in_the_proposal() {
        let count = |site: DrawSite| {
            map::cities()
                .filter(|city| DrawSite::of(*city) == site)
                .count()
        };
        assert_eq!(count(DrawSite::Crossroads), 20);
        assert_eq!(count(DrawSite::Port), 14);
        assert_eq!(count(DrawSite::Town), 13);
        for city in [
            LocationId::Arkham,
            LocationId::Boston,
            LocationId::Bristol,
            LocationId::FallRiver,
        ] {
            assert_eq!(DrawSite::of(city), DrawSite::Crossroads);
            assert!(city.is_port());
        }
    }

    #[test]
    fn the_tables_give_the_proposal_counters() {
        assert_eq!(
            DrawSite::Crossroads.counter(2, LocationId::Boston),
            Some(DeepOnes)
        );
        assert_eq!(
            DrawSite::Crossroads.counter(2, LocationId::Worcester),
            Some(Ghouls)
        );
        assert_eq!(
            DrawSite::Town.counter(6, LocationId::Dunwich),
            Some(FlyingHorror)
        );
        assert_eq!(DrawSite::Sea.counter(3, LocationId::Boston), None);
        assert_eq!(DrawSite::Sea.counter(6, LocationId::Boston), Some(DeepOnes));
    }
}
