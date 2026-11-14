//! The static board: cities and sea zones, roads, sea links and state borders.
//!
//! Transcribed from the board scan (`img/board.jpg`); the pixel positions are those of the
//! city dots on that picture, and a sea zone's position is an open-water spot of the zone.
//! A road fork or crossing without a city joins every road meeting there, so the cities at
//! their ends are neighbours.

use crate::domain::location_id::{LOCATION_COUNT, LocationId};
use crate::domain::location_kind::LocationKind;

use LocationId::*;

/// Fixed data of one location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocationInfo {
    /// The location.
    pub id: LocationId,
    /// French display name.
    pub name: &'static str,
    /// City or sea zone.
    pub kind: LocationKind,
    /// A port (green dot): ships leave from it to its sea zone.
    pub port: bool,
    /// Horizontal pixel position on the board picture.
    pub x: u32,
    /// Vertical pixel position on the board picture.
    pub y: u32,
}

const fn city(id: LocationId, name: &'static str, x: u32, y: u32) -> LocationInfo {
    LocationInfo {
        id,
        name,
        kind: LocationKind::City,
        port: false,
        x,
        y,
    }
}

const fn port(id: LocationId, name: &'static str, x: u32, y: u32) -> LocationInfo {
    LocationInfo {
        id,
        name,
        kind: LocationKind::City,
        port: true,
        x,
        y,
    }
}

const fn sea(id: LocationId, name: &'static str, x: u32, y: u32) -> LocationInfo {
    LocationInfo {
        id,
        name,
        kind: LocationKind::Sea,
        port: false,
        x,
        y,
    }
}

/// Width of the board picture, in pixels.
pub const BOARD_WIDTH: u32 = 2242;
/// Height of the board picture, in pixels.
pub const BOARD_HEIGHT: u32 = 2732;

/// Every location, in [`LocationId::ALL`] order.
const LOCATIONS: [LocationInfo; LOCATION_COUNT] = [
    city(Nashua, "Nashua", 607, 399),
    city(Dunwich, "Dunwich", 492, 512),
    city(Lowell, "Lowell", 767, 566),
    city(Lawrence, "Lawrence", 884, 496),
    city(Fitchburg, "Fitchburg", 310, 626),
    city(Concord, "Concord", 696, 814),
    city(Oakdale, "Oakdale", 337, 857),
    city(Sudbury, "Sudbury", 672, 941),
    city(Worcester, "Worcester", 317, 1057),
    city(Westboro, "Westboro", 476, 1060),
    city(Ashland, "Ashland", 646, 1066),
    city(Dedham, "Dedham", 887, 1057),
    city(Milford, "Milford", 561, 1217),
    city(Stoughton, "Stoughton", 934, 1249),
    city(Rockland, "Rockland", 1101, 1239),
    city(Greenbush, "Greenbush", 1260, 1166),
    city(Oxford, "Oxford", 250, 1266),
    city(Foxboro, "Foxboro", 790, 1338),
    city(Woonsocket, "Woonsocket", 560, 1411),
    city(BridgewaterJunction, "Bridgewater Junction", 1059, 1420),
    city(Pascoag, "Pascoag", 379, 1477),
    city(Attleboro, "Attleboro", 771, 1487),
    city(Taunton, "Taunton", 951, 1525),
    city(Middleboro, "Middleboro", 1105, 1550),
    city(Providence, "Providence", 653, 1621),
    city(Tremont, "Tremont", 1248, 1692),
    city(Plainfield, "Plainfield", 201, 1844),
    city(Barnstable, "Barnstable", 1670, 1788),
    city(Falmouth, "Falmouth", 1388, 1972),
    port(Newburyport, "Newburyport", 1121, 336),
    port(Innsmouth, "Innsmouth", 1186, 502),
    port(Gloucester, "Gloucester", 1342, 603),
    port(Arkham, "Arkham", 1084, 718),
    port(Lynn, "Lynn", 1067, 790),
    port(Boston, "Boston", 987, 896),
    port(Plymouth, "Plymouth", 1319, 1443),
    port(Provincetown, "Provincetown", 1771, 1298),
    port(Chatham, "Chatham", 1999, 1791),
    port(FallRiver, "Fall River", 913, 1796),
    port(Bristol, "Bristol", 796, 1841),
    port(Apponaug, "Apponaug", 630, 1801),
    port(Newport, "Newport", 780, 2069),
    port(NarragansettPier, "Narragansett Pier", 642, 2154),
    port(Westerly, "Westerly", 307, 2252),
    port(NewBedford, "New Bedford", 1104, 1888),
    port(Edgartown, "Edgartown", 1477, 2200),
    port(Nantucket, "Nantucket", 1857, 2345),
    sea(GulfOfMaine, "Golfe du Maine", 1800, 420),
    sea(MassachusettsBay, "Baie du Massachusetts", 1450, 1050),
    sea(CapeCodBay, "Baie du cap Cod", 1560, 1600),
    sea(BuzzardsBay, "Baie de Buzzards", 1180, 2080),
    sea(NantucketSound, "Nantucket Sound", 1700, 2060),
    sea(AtlanticOcean, "Océan Atlantique", 1150, 2560),
];

/// Roads between two cities, each listed once.
pub const ROADS: [(LocationId, LocationId); 82] = [
    (Nashua, Dunwich),
    (Nashua, Lowell),
    (Dunwich, Lowell),
    (Dunwich, Fitchburg),
    (Dunwich, Concord),
    (Lowell, Fitchburg),
    (Lowell, Concord),
    (Lowell, Lawrence),
    (Lawrence, Newburyport),
    (Lawrence, Arkham),
    (Fitchburg, Oakdale),
    (Fitchburg, Concord),
    (Concord, Sudbury),
    (Concord, Boston),
    (Oakdale, Worcester),
    (Oakdale, Sudbury),
    (Sudbury, Ashland),
    (Sudbury, Boston),
    (Worcester, Oxford),
    (Worcester, Westboro),
    (Worcester, Milford),
    (Westboro, Ashland),
    (Ashland, Milford),
    (Ashland, Boston),
    (Dedham, Woonsocket),
    (Dedham, Stoughton),
    (Dedham, Boston),
    (Milford, Woonsocket),
    (Stoughton, Foxboro),
    (Stoughton, Taunton),
    (Rockland, BridgewaterJunction),
    (Rockland, Greenbush),
    (Rockland, Boston),
    (Greenbush, BridgewaterJunction),
    (Greenbush, Plymouth),
    (Greenbush, Boston),
    (Oxford, Plainfield),
    (Oxford, Pascoag),
    (Foxboro, Attleboro),
    (Foxboro, Taunton),
    (Woonsocket, Pascoag),
    (Woonsocket, Providence),
    (Woonsocket, Attleboro),
    (Woonsocket, FallRiver),
    (Woonsocket, Taunton),
    (Woonsocket, Bristol),
    (BridgewaterJunction, Middleboro),
    (BridgewaterJunction, Boston),
    (Attleboro, Providence),
    (Attleboro, Taunton),
    (Attleboro, Bristol),
    (Attleboro, FallRiver),
    (Taunton, Providence),
    (Taunton, Middleboro),
    (Taunton, FallRiver),
    (Taunton, Bristol),
    (Middleboro, Tremont),
    (Middleboro, NewBedford),
    (Middleboro, Plymouth),
    (Providence, Plainfield),
    (Providence, Apponaug),
    (Providence, Bristol),
    (Providence, FallRiver),
    (Tremont, Falmouth),
    (Tremont, Barnstable),
    (Tremont, Plymouth),
    (Plainfield, Westerly),
    (Barnstable, Falmouth),
    (Barnstable, Chatham),
    (Barnstable, Provincetown),
    (Newburyport, Innsmouth),
    (Newburyport, Arkham),
    (Innsmouth, Arkham),
    (Arkham, Lynn),
    (Lynn, Boston),
    (Provincetown, Chatham),
    (FallRiver, Bristol),
    (FallRiver, Newport),
    (FallRiver, NewBedford),
    (Apponaug, NarragansettPier),
    (Apponaug, Westerly),
    (NarragansettPier, Westerly),
];

/// Sea links: a port and its sea zone, or two neighbouring sea zones.
pub const SEA_LINKS: [(LocationId, LocationId); 25] = [
    (Newburyport, GulfOfMaine),
    (Innsmouth, GulfOfMaine),
    (Gloucester, GulfOfMaine),
    (Arkham, MassachusettsBay),
    (Lynn, MassachusettsBay),
    (Boston, MassachusettsBay),
    (Plymouth, CapeCodBay),
    (Provincetown, CapeCodBay),
    (Chatham, NantucketSound),
    (FallRiver, AtlanticOcean),
    (Bristol, AtlanticOcean),
    (Apponaug, AtlanticOcean),
    (Newport, AtlanticOcean),
    (NarragansettPier, AtlanticOcean),
    (Westerly, AtlanticOcean),
    (NewBedford, BuzzardsBay),
    (Edgartown, NantucketSound),
    (Nantucket, NantucketSound),
    (GulfOfMaine, MassachusettsBay),
    (GulfOfMaine, NantucketSound),
    (GulfOfMaine, CapeCodBay),
    (MassachusettsBay, CapeCodBay),
    (BuzzardsBay, AtlanticOcean),
    (BuzzardsBay, NantucketSound),
    (NantucketSound, AtlanticOcean),
];

/// Roads crossing a state border (dotted lines), each listed once.
pub const BORDER_CROSSINGS: [(LocationId, LocationId); 18] = [
    (Nashua, Dunwich),
    (Nashua, Lowell),
    (Plainfield, Westerly),
    (Oxford, Plainfield),
    (Oxford, Pascoag),
    (Providence, Plainfield),
    (Providence, FallRiver),
    (Woonsocket, Attleboro),
    (Woonsocket, FallRiver),
    (Woonsocket, Taunton),
    (Milford, Woonsocket),
    (Dedham, Woonsocket),
    (Attleboro, Providence),
    (Attleboro, Bristol),
    (FallRiver, Bristol),
    (FallRiver, Newport),
    (Taunton, Providence),
    (Taunton, Bristol),
];

/// The fixed data of a location.
pub fn location(id: LocationId) -> &'static LocationInfo {
    &LOCATIONS[id.index()]
}

/// Every city, in board order.
pub fn cities() -> impl Iterator<Item = LocationId> {
    LocationId::ALL
        .into_iter()
        .filter(|id| location(*id).kind == LocationKind::City)
}

impl LocationId {
    /// Whether the location is a city (as opposed to a sea zone).
    pub fn is_city(self) -> bool {
        location(self).kind == LocationKind::City
    }

    /// Whether the location is a sea zone.
    pub fn is_sea(self) -> bool {
        location(self).kind == LocationKind::Sea
    }

    /// Whether the location is a port.
    pub fn is_port(self) -> bool {
        location(self).port
    }

    /// French phrase locating something there: « à Boston », « dans le golfe du Maine ».
    pub fn at_phrase(self) -> String {
        match self {
            GulfOfMaine => "dans le golfe du Maine".to_string(),
            MassachusettsBay => "dans la baie du Massachusetts".to_string(),
            CapeCodBay => "dans la baie du cap Cod".to_string(),
            BuzzardsBay => "dans la baie de Buzzards".to_string(),
            NantucketSound => "dans le Nantucket Sound".to_string(),
            AtlanticOcean => "dans l'océan Atlantique".to_string(),
            city => format!("à {}", location(city).name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked(links: &[(LocationId, LocationId)], a: LocationId, b: LocationId) -> bool {
        links.contains(&(a, b)) || links.contains(&(b, a))
    }

    #[test]
    fn the_table_follows_the_enum_order() {
        for id in LocationId::ALL {
            assert_eq!(location(id).id, id);
        }
        assert_eq!(cities().count(), 47);
        assert_eq!(cities().filter(|id| id.is_port()).count(), 18);
    }

    #[test]
    fn roads_join_two_distinct_cities_once() {
        for (index, (a, b)) in ROADS.iter().enumerate() {
            assert!(a.is_city() && b.is_city() && a != b, "{a:?}-{b:?}");
            assert!(
                !linked(&ROADS[index + 1..], *a, *b),
                "duplicate {a:?}-{b:?}"
            );
        }
    }

    #[test]
    fn sea_links_touch_a_sea_zone_from_a_port_or_a_zone() {
        for (a, b) in SEA_LINKS {
            assert!(b.is_sea(), "{b:?}");
            assert!(a.is_sea() || a.is_port(), "{a:?}");
        }
        for port in cities().filter(|id| id.is_port()) {
            assert!(
                SEA_LINKS.iter().any(|(a, _)| *a == port),
                "{port:?} touches no sea zone"
            );
        }
    }

    #[test]
    fn border_crossings_are_roads() {
        for (a, b) in BORDER_CROSSINGS {
            assert!(linked(&ROADS, a, b), "{a:?}-{b:?}");
        }
    }

    #[test]
    fn positions_are_on_the_board() {
        for id in LocationId::ALL {
            let info = location(id);
            assert!(info.x < BOARD_WIDTH && info.y < BOARD_HEIGHT, "{id:?}");
        }
    }
}
