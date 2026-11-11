//! How dangerous or useful the investigators make each location for the professor.

use crate::domain::location_id::{LOCATION_COUNT, LocationId};
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::travel::{self, Network};
use crate::domain::turn::GameState;

/// Steps from each active investigator to every location, by investigator index.
pub type Reach = Vec<(usize, [Option<u8>; LOCATION_COUNT])>;

/// Steps from each active investigator to every location, along the moves they may make.
pub fn investigator_reach(state: &GameState) -> Reach {
    state
        .active_investigators()
        .into_iter()
        .map(|index| {
            let from = state.investigators[index].location;
            let reach = travel::distances(from, Network::All, |location| {
                !(location.is_city() && state.is_destroyed(location))
            });
            (index, reach)
        })
        .collect()
}

/// Risk of ending a turn in a location: an armed investigator there, or one move away, will
/// shoot at the professor.
pub fn danger(state: &GameState, weights: &ProfessorWeights, location: LocationId) -> f64 {
    danger_with(state, weights, &investigator_reach(state), location)
}

/// [`danger`] with the investigators' reach already computed.
pub fn danger_with(
    state: &GameState,
    weights: &ProfessorWeights,
    reach: &Reach,
    location: LocationId,
) -> f64 {
    if location.is_sea() {
        return 0.0;
    }
    reach
        .iter()
        .filter(|(index, _)| state.investigators[*index].weapon)
        .map(|(_, reach)| match reach[location.index()] {
            Some(0) => weights.danger_here,
            Some(1) => weights.danger_next,
            Some(2) => weights.danger_far,
            _ => 0.0,
        })
        .sum()
}

/// Steps from the nearest active investigator to a location.
pub fn nearest_investigator(state: &GameState, location: LocationId) -> Option<u8> {
    nearest_with(&investigator_reach(state), location)
}

/// [`nearest_investigator`] with the investigators' reach already computed.
pub fn nearest_with(reach: &Reach, location: LocationId) -> Option<u8> {
    reach
        .iter()
        .filter_map(|(_, steps)| steps[location.index()])
        .min()
}

/// Whether a location holds an active investigator.
pub fn has_investigator(state: &GameState, location: LocationId) -> bool {
    !state.investigators_at(location).is_empty()
}
