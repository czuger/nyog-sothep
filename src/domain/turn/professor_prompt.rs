use serde::{Deserialize, Serialize};

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::turn::professor_brain::{ProfessorMoveOptions, Teleport};

/// A decision Professor Petersen must take before the game goes on.
///
/// The options are computed when the prompt is asked, after any die the rules roll first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessorPrompt {
    /// Choose the start city and the city where Nyog'Sothep will appear.
    Setup,
    /// Cast a Teleportation before moving, or not.
    Teleport {
        /// The Teleportations he may cast.
        options: Vec<Teleport>,
    },
    /// Move.
    Movement {
        /// The legal destinations.
        options: ProfessorMoveOptions,
    },
    /// Attack an investigator met on arrival, or not.
    Attack {
        /// The investigators in the city, by index.
        candidates: Vec<usize>,
        /// The die made him reveal himself: he must attack.
        forced: bool,
    },
    /// Carry an investigator who faced him unarmed.
    Carry {
        /// The investigator carried.
        investigator: usize,
        /// Where they may be carried.
        options: Vec<LocationId>,
    },
    /// Retreat (or be pushed back) one location after a fight.
    Relocate {
        /// Where he may go.
        options: Vec<LocationId>,
        /// Pushed back by a protective sign rather than retreating from a shot.
        pushed_back: bool,
    },
    /// Summon Nyog'Sothep now, or wait.
    Summon,
    /// Place a counter where he stands, or not.
    Placement {
        /// The counters of the hand that may be placed here.
        options: Vec<CounterKind>,
    },
    /// Pick one of the two counters a Table 1 roll offers.
    Table1 {
        /// The counters offered.
        options: Vec<CounterKind>,
    },
    /// Discard a counter: the hand holds six.
    Discard {
        /// The hand.
        hand: Vec<CounterKind>,
    },
    /// Choose Nyog'Sothep's next location.
    NyogStep {
        /// The locations it may enter.
        options: Vec<LocationId>,
    },
    /// Name cities, one of which must be his position.
    NameCities {
        /// How many cities to name.
        count: usize,
    },
    /// Name two cities, one of which must hold Fanatics.
    NameFanaticsCities,
}

/// The professor's answer to a [`ProfessorPrompt`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfessorAnswer {
    /// The start city and Nyog'Sothep's city.
    Setup {
        /// Where he starts.
        start: LocationId,
        /// Where Nyog'Sothep will appear.
        nyog_city: LocationId,
    },
    /// The Teleportation cast, if any.
    Teleport(Option<Teleport>),
    /// The investigator attacked, if any.
    Attack(Option<usize>),
    /// A location: his move (the Flying Horror is used only when it is not an ordinary step),
    /// where he carries an investigator, where he retreats, or Nyog'Sothep's step.
    Location(LocationId),
    /// Whether he summons Nyog'Sothep.
    Summon(bool),
    /// The counter placed, if any.
    Placement(Option<CounterKind>),
    /// The counter kept from Table 1, or discarded.
    Counter(CounterKind),
    /// The named cities.
    Cities(Vec<LocationId>),
}
