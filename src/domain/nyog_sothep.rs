use serde::{Deserialize, Serialize};

use crate::domain::location_id::LocationId;

/// Distinct cities of Fanatics the professor needs to summon Nyog'Sothep.
pub const SUMMONING_FANATICS_CITIES: usize = 5;
/// The professor must be at most this many steps from the appearance city (« less than three
/// cities away »).
pub const SUMMONING_RANGE: u8 = 2;
/// Investigators knowing the spell who must gather to cast it.
pub const BANISHERS_NEEDED: usize = 3;

/// The Foul Fog-One.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NyogSothep {
    /// The city the professor wrote down for its appearance (secret until the summoning).
    pub city: LocationId,
    /// Where it stands once summoned.
    pub location: Option<LocationId>,
    /// The professor has joined it: they move together.
    pub joined: bool,
    /// The city where three spell holders gathered, which it must reach by the shortest path.
    pub target: Option<LocationId>,
}

impl NyogSothep {
    /// Nyog'Sothep to appear in `city`, not summoned yet.
    pub fn new(city: LocationId) -> NyogSothep {
        NyogSothep {
            city,
            location: None,
            joined: false,
            target: None,
        }
    }

    /// Whether it has been summoned.
    pub fn is_summoned(&self) -> bool {
        self.location.is_some()
    }
}

/// SAN lost on meeting Nyog'Sothep with a protective sign.
pub fn meeting_san_loss(seen_before: bool) -> i32 {
    if seen_before { 2 } else { 4 }
}

/// The result of the banishment spell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BanishmentResult {
    /// 3 or more: each participant loses 4 more SAN; the spell fails.
    Disaster,
    /// 2: each participant loses 2 more SAN; the spell fails.
    Backlash,
    /// 1 or 0: the spell fails.
    Fails,
    /// −1: the investigators place Nyog'Sothep in the city of their choice.
    Displaced,
    /// −2 or less: Nyog'Sothep is cast back into the abyss.
    Banished,
}

impl BanishmentResult {
    /// SAN lost by each participant on top of the meeting.
    pub fn san_loss(self) -> i32 {
        match self {
            BanishmentResult::Disaster => 4,
            BanishmentResult::Backlash => 2,
            _ => 0,
        }
    }
}

/// Reads the banishment table: one die minus the number of participants.
pub fn banishment(roll: u8, participants: usize) -> BanishmentResult {
    let result = i64::from(roll) - participants as i64;
    match result {
        3.. => BanishmentResult::Disaster,
        2 => BanishmentResult::Backlash,
        0 | 1 => BanishmentResult::Fails,
        -1 => BanishmentResult::Displaced,
        _ => BanishmentResult::Banished,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_banishment_table() {
        assert_eq!(banishment(6, 3), BanishmentResult::Disaster);
        assert_eq!(banishment(5, 3), BanishmentResult::Backlash);
        assert_eq!(banishment(3, 3), BanishmentResult::Fails);
        assert_eq!(banishment(2, 3), BanishmentResult::Displaced);
        assert_eq!(banishment(1, 3), BanishmentResult::Banished);
        assert_eq!(banishment(2, 4), BanishmentResult::Banished);
        assert_eq!(BanishmentResult::Disaster.san_loss(), 4);
    }
}
