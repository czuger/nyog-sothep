use crate::domain::investigator_ai::paths::{UNREACHABLE, steps_to_nearest};
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::turn::GameState;

/// Where to send Nyog'Sothep after a partial banishment: the open city farthest from the
/// investigators it would kill (those without a sign), or from all of them.
pub(super) fn choose_banishment_city(state: &GameState) -> LocationId {
    let active = state.active_investigators();
    let mut exposed: Vec<LocationId> = active
        .iter()
        .filter(|index| !state.investigators[**index].sign)
        .map(|index| state.investigators[*index].location)
        .collect();
    if exposed.is_empty() {
        exposed = active
            .iter()
            .map(|index| state.investigators[*index].location)
            .collect();
    }
    map::cities()
        .filter(|city| !state.is_destroyed(*city) && state.investigators_at(*city).is_empty())
        .max_by_key(|city| steps_to_nearest(*city, &exposed).min(UNREACHABLE - 1))
        .unwrap_or(LocationId::Nantucket)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;

    #[test]
    fn nyog_sothep_is_sent_far_from_the_investigators() {
        let mut state = test_state();
        for investigator in &mut state.investigators {
            investigator.location = LocationId::Boston;
        }
        let city = choose_banishment_city(&state);
        assert!(
            steps_to_nearest(city, &[LocationId::Boston]) >= 5,
            "{city:?}"
        );
    }
}
