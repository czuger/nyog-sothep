//! Where Nyog'Sothep goes once the professor has joined it.

use crate::domain::location_id::LocationId;
use crate::domain::travel::{self, Network};
use crate::domain::turn::GameState;

/// Hunts the investigators: those without a protective sign die on meeting the Foul One.
pub fn choose_nyog_step(state: &GameState, options: &[LocationId]) -> LocationId {
    let score = |city: LocationId| {
        let here = state.investigators_at(city);
        let unsigned = here
            .iter()
            .filter(|index| !state.investigators[**index].sign)
            .count() as f64;
        let signed = here.len() as f64 - unsigned;
        let nearest = state
            .active_investigators()
            .into_iter()
            .filter_map(|index| {
                let at = state.investigators[index].location;
                travel::distance(city, at, Network::All, |_| true)
            })
            .min()
            .map_or(10.0, f64::from);
        10.0 * unsigned + 4.0 * signed - nearest
    };
    options
        .iter()
        .copied()
        .max_by(|a, b| score(*a).total_cmp(&score(*b)))
        .unwrap_or(state.professor.location)
}
