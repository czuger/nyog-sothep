use crate::domain::investigator_ai::banishers::{rally_point, ready_to_cast};
use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::investigator_ai::paths::{UNREACHABLE, steps, steps_to_nearest};
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::turn::GameState;

/// Nyog'Sothep this close to an investigator without a sign could kill them on its way back.
const UNSIGNED_MARGIN: u32 = 2;

/// Where to send Nyog'Sothep after a partial banishment.
///
/// When enough banishers can cast again, it goes close to their rally point, so that it comes
/// back at once for another attempt, while staying clear of those without a sign. Otherwise it
/// goes as far as possible from the investigators it would kill, or from all of them, to give
/// them time.
pub(super) fn choose_banishment_city(
    state: &GameState,
    weights: &InvestigatorWeights,
) -> LocationId {
    let open: Vec<LocationId> = map::cities()
        .filter(|city| !state.is_destroyed(*city) && state.investigators_at(*city).is_empty())
        .collect();
    let unsigned = locations(state, false);
    if ready_to_cast(state, weights)
        && let Some(rally) = rally_point(state, weights)
        && let Some(near) = open
            .iter()
            .copied()
            .filter(|city| steps_to_nearest(*city, &unsigned) > UNSIGNED_MARGIN)
            .min_by_key(|city| steps(*city, rally))
    {
        return near;
    }
    let mut exposed = unsigned;
    if exposed.is_empty() {
        exposed = locations(state, true);
    }
    open.into_iter()
        .max_by_key(|city| steps_to_nearest(*city, &exposed).min(UNREACHABLE - 1))
        .unwrap_or(LocationId::Nantucket)
}

/// Where the active investigators stand: all of them, or only those without a sign.
fn locations(state: &GameState, with_sign: bool) -> Vec<LocationId> {
    state
        .active_investigators()
        .into_iter()
        .map(|index| &state.investigators[index])
        .filter(|investigator| with_sign || !investigator.sign)
        .map(|investigator| investigator.location)
        .collect()
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
        let city = choose_banishment_city(&state, &InvestigatorWeights::default());
        assert!(
            steps_to_nearest(city, &[LocationId::Boston]) >= 5,
            "{city:?}"
        );
    }

    #[test]
    fn fit_banishers_call_it_back_at_once() {
        let mut state = test_state();
        for investigator in &mut state.investigators {
            investigator.location = LocationId::Boston;
            investigator.spell = true;
            investigator.sign = true;
        }
        let city = choose_banishment_city(&state, &InvestigatorWeights::default());
        assert_eq!(steps(city, LocationId::Boston), 1, "{city:?}");
    }
}
