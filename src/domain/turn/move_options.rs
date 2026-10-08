use crate::domain::location_id::LocationId;
use crate::domain::travel;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::prompt::MoveOrder;

/// The moves open to an investigator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveOptions {
    /// Whether staying to consult a psychoanalyst is allowed.
    pub psychoanalyst: bool,
    /// Neighbouring locations the investigator may enter.
    pub destinations: Vec<LocationId>,
}

impl MoveOptions {
    /// The moves of an investigator, or `None` when they do not choose this turn (out of the
    /// game, or losing their turn).
    ///
    /// # Arguments
    /// * `replay` - a « play again » move, where the psychoanalyst is not offered.
    pub fn of(state: &GameState, investigator: usize, replay: bool) -> Option<MoveOptions> {
        let mover = &state.investigators[investigator];
        if !mover.is_active() || mover.skips_action {
            return None;
        }
        let from = mover.location;
        let destinations = travel::neighbours(from)
            .iter()
            .copied()
            .filter(|to| !(to.is_city() && state.is_destroyed(*to)))
            .filter(|to| mover.forbidden_city != Some(*to))
            .filter(|to| mover.blocked_crossing != Some(*to))
            .filter(|to| !mover.car_broken || to.is_sea())
            .collect();
        Some(MoveOptions {
            psychoanalyst: !replay && from.is_city() && !mover.car_broken,
            destinations,
        })
    }

    /// Whether an order is allowed.
    pub fn allows(&self, order: MoveOrder) -> bool {
        match order {
            MoveOrder::Stay => true,
            MoveOrder::Psychoanalyst => self.psychoanalyst,
            MoveOrder::Go(to) => self.destinations.contains(&to),
        }
    }
}
