//! The places of the board: 47 cities and 6 sea zones.

use serde::{Deserialize, Serialize};

/// Declares `LocationId` with its stable snake_case codes, in board order.
macro_rules! locations {
    ($($variant:ident => $code:literal,)*) => {
        /// A city or a sea zone of the board.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum LocationId {
            $(
                #[allow(missing_docs)]
                $variant,
            )*
        }

        impl LocationId {
            /// Every location, in the order of the map table.
            pub const ALL: [LocationId; LOCATION_COUNT] = [$(LocationId::$variant,)*];

            /// The stable code stored in the database and used in forms.
            pub fn code(self) -> &'static str {
                match self {
                    $(LocationId::$variant => $code,)*
                }
            }

            /// Reads a code written by [`LocationId::code`].
            pub fn from_code(code: &str) -> Option<LocationId> {
                match code {
                    $($code => Some(LocationId::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

/// The number of locations of the board.
pub const LOCATION_COUNT: usize = 53;

locations! {
    Nashua => "nashua",
    Dunwich => "dunwich",
    Lowell => "lowell",
    Lawrence => "lawrence",
    Fitchburg => "fitchburg",
    Concord => "concord",
    Oakdale => "oakdale",
    Sudbury => "sudbury",
    Worcester => "worcester",
    Westboro => "westboro",
    Ashland => "ashland",
    Dedham => "dedham",
    Milford => "milford",
    Stoughton => "stoughton",
    Rockland => "rockland",
    Greenbush => "greenbush",
    Oxford => "oxford",
    Foxboro => "foxboro",
    Woonsocket => "woonsocket",
    BridgewaterJunction => "bridgewater_junction",
    Pascoag => "pascoag",
    Attleboro => "attleboro",
    Taunton => "taunton",
    Middleboro => "middleboro",
    Providence => "providence",
    Tremont => "tremont",
    Plainfield => "plainfield",
    Barnstable => "barnstable",
    Falmouth => "falmouth",
    Newburyport => "newburyport",
    Innsmouth => "innsmouth",
    Gloucester => "gloucester",
    Arkham => "arkham",
    Lynn => "lynn",
    Boston => "boston",
    Plymouth => "plymouth",
    Provincetown => "provincetown",
    Chatham => "chatham",
    FallRiver => "fall_river",
    Bristol => "bristol",
    Apponaug => "apponaug",
    Newport => "newport",
    NarragansettPier => "narragansett_pier",
    Westerly => "westerly",
    NewBedford => "new_bedford",
    Edgartown => "edgartown",
    Nantucket => "nantucket",
    GulfOfMaine => "gulf_of_maine",
    MassachusettsBay => "massachusetts_bay",
    CapeCodBay => "cape_cod_bay",
    BuzzardsBay => "buzzards_bay",
    NantucketSound => "nantucket_sound",
    AtlanticOcean => "atlantic_ocean",
}

impl LocationId {
    /// The position of the location in [`LocationId::ALL`], for table lookups.
    pub fn index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for LocationId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(crate::domain::map::location(*self).name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for (index, location) in LocationId::ALL.into_iter().enumerate() {
            assert_eq!(location.index(), index);
            assert_eq!(LocationId::from_code(location.code()), Some(location));
        }
        assert_eq!(LocationId::from_code("paris"), None);
    }
}
