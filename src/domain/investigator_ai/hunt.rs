//! Where the armed investigators strike: the professor's likely positions now, and after his
//! next move, shared out so that hunters cover different ones.

use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::location_id::{LOCATION_COUNT, LocationId};
use crate::domain::professor_belief::{ProfessorBelief, normalized};
use crate::domain::travel;
use crate::domain::turn::GameState;

/// The professor's whereabouts as the hunters share them out.
pub(super) struct Hunt {
    /// Worth of the chance of ending the move in his city, where the fight is unavoidable.
    now_worth: f64,
    /// Worth of the chance of having him within one step after his next move.
    next_worth: f64,
    /// Chance that he stands at each location now.
    now: [f64; LOCATION_COUNT],
    /// Chance that he stands at each location after his next move.
    next: [f64; LOCATION_COUNT],
    /// Locations a hunter already threatens next turn.
    covered: [bool; LOCATION_COUNT],
}

impl Hunt {
    /// The hunt from the investigators' notebook, nothing covered yet.
    pub(super) fn of(state: &GameState, weights: &InvestigatorWeights) -> Hunt {
        let now = ProfessorBelief::of(state).weights(state);
        Hunt {
            now_worth: weights.hunt_now_worth,
            next_worth: weights.hunt_next_worth,
            now,
            next: after_one_move(&now),
            covered: [false; LOCATION_COUNT],
        }
    }

    /// What ending a hunter's move in `to` is worth: catching him now, or threatening the
    /// locations he may reach next that no other hunter threatens yet.
    pub(super) fn worth(&self, to: LocationId) -> f64 {
        let threatened: f64 = threatened_from(to)
            .filter(|at| !self.covered[at.index()])
            .map(|at| self.next[at.index()])
            .sum();
        self.now_worth * self.now[to.index()] + self.next_worth * threatened
    }

    /// A hunter ends the move in `to`: the locations around are threatened.
    pub(super) fn cover(&mut self, to: LocationId) {
        for at in threatened_from(to) {
            self.covered[at.index()] = true;
        }
    }
}

/// The locations a hunter standing in `from` reaches on the next turn.
fn threatened_from(from: LocationId) -> impl Iterator<Item = LocationId> {
    std::iter::once(from).chain(travel::neighbours(from).iter().copied())
}

/// Spreads each location's chance evenly over staying and every neighbour.
fn after_one_move(now: &[f64; LOCATION_COUNT]) -> [f64; LOCATION_COUNT] {
    let mut next = [0.0; LOCATION_COUNT];
    for from in LocationId::ALL {
        let chance = now[from.index()];
        if chance == 0.0 {
            continue;
        }
        let reached: Vec<LocationId> = threatened_from(from).collect();
        let share = chance / reached.len() as f64;
        for at in reached {
            next[at.index()] += share;
        }
    }
    normalized(next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;
    use crate::domain::turn::{Clue, ClueKind};
    use LocationId::*;

    #[test]
    fn a_covered_area_is_worth_less_to_the_next_hunter() {
        let mut state = test_state();
        state.clues = vec![Clue {
            turn: 1,
            kind: ClueKind::Spotted { at: Boston },
        }];
        let mut hunt = Hunt::of(&state, &InvestigatorWeights::default());
        let before = hunt.worth(Boston);
        assert!(before > hunt.worth(Worcester));
        hunt.cover(Boston);
        assert!(hunt.worth(Boston) < before);
    }
}
