use serde::{Deserialize, Serialize};

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;

/// Professor Petersen's own state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Professor {
    /// Hit points left; he dies at 0.
    pub hp: i32,
    /// His position (secret until revealed).
    pub location: LocationId,
}

/// One line of the professor's route sheet: where he went and what he placed there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteStage {
    /// The turn.
    pub turn: u32,
    /// Where he ended his move.
    pub location: LocationId,
    /// The counter he placed there, if any.
    pub counter: Option<CounterKind>,
}
