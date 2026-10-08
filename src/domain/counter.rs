use serde::{Deserialize, Serialize};

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;

/// Where a counter is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CounterPlace {
    /// Not drawn yet: Table 1 may give it.
    Reserve,
    /// In the professor's hand, behind his screen.
    Hand,
    /// Placed by the professor in a city, on his route sheet; `discovered` once met and left
    /// on the board.
    Placed {
        /// The city.
        city: LocationId,
        /// Whether the counter lies face up on the board.
        discovered: bool,
    },
    /// Out of the game for good.
    Removed,
}

/// One counter of the box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counter {
    /// Kind of counter.
    pub kind: CounterKind,
    /// Where it is.
    pub place: CounterPlace,
}

impl Counter {
    /// The city where the counter is placed, if any.
    pub fn city(&self) -> Option<LocationId> {
        match self.place {
            CounterPlace::Placed { city, .. } => Some(city),
            _ => None,
        }
    }
}
