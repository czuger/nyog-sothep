//! The investigators who can banish Nyog'Sothep, and where they gather.

use crate::domain::investigator::Investigator;
use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::investigator_ai::paths::{UNREACHABLE, steps, walking_steps_from};
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::turn::GameState;

/// What each meeting with Nyog'Sothep costs a banisher after the first one.
const MEETING_COST: f64 = 2.0;
/// Nyog'Sothep this close can strike a banisher on the way.
const HARASSING_RANGE: u32 = 2;

/// An investigator who can cast the spell and survive meeting Nyog'Sothep.
pub(super) fn is_banisher(investigator: &Investigator) -> bool {
    investigator.spell && investigator.sign
}

/// Where the active banishers stand.
pub(super) fn banishers(state: &GameState) -> Vec<LocationId> {
    state
        .active_investigators()
        .into_iter()
        .filter(|index| is_banisher(&state.investigators[*index]))
        .map(|index| state.investigators[index].location)
        .collect()
}

/// The open city where three banishers can gather at the least cost.
///
/// Only the cost of the third cheapest banisher counts: three are enough to cast the spell. A
/// tired banisher's steps cost more, so the fit ones come to them rather than the reverse.
pub(super) fn rally_point(state: &GameState, weights: &InvestigatorWeights) -> Option<LocationId> {
    let costs: Vec<Vec<Option<u32>>> = state
        .active_investigators()
        .into_iter()
        .map(|index| &state.investigators[index])
        .filter(|investigator| is_banisher(investigator))
        .map(|investigator| {
            // Rounded to whole steps so that costs can be ordered.
            let step_cost = match f64::from(investigator.san) < weights.fit_san {
                true => weights.tired_step_cost.round().max(1.0) as u32,
                false => 1,
            };
            walking_steps_from(state, investigator.location)
                .into_iter()
                .map(|steps| (steps < UNREACHABLE).then_some(steps * step_cost))
                .collect()
        })
        .collect();
    map::cities()
        .filter(|city| !state.is_destroyed(*city))
        .min_by_key(|city| {
            let needed = state.rules.banishers_needed();
            let mut reachable: Vec<u32> =
                costs.iter().filter_map(|cost| cost[city.index()]).collect();
            reachable.sort_unstable();
            let short = needed.saturating_sub(reachable.len());
            let gathered = reachable.get(needed - 1).copied();
            (short, gathered.unwrap_or(u32::MAX))
        })
}

/// Whether a banisher stays at the psychoanalyst rather than heads for the rally point.
///
/// Waiting there costs nothing once three banishers exist. On the way, Nyog'Sothep close by
/// strikes at every step, so the banisher first rests up to the SAN the trip will cost.
pub(super) fn banisher_rests(
    state: &GameState,
    weights: &InvestigatorWeights,
    me: &Investigator,
) -> bool {
    let rally = rally_point(state, weights);
    if banishers(state).len() >= state.rules.banishers_needed() && rally == Some(me.location) {
        return true;
    }
    let harassed = state
        .nyog
        .location
        .is_some_and(|nyog| steps(nyog, me.location) <= HARASSING_RANGE);
    let trip = rally.map_or(0, |rally| {
        walking_steps_from(state, me.location)[rally.index()]
    });
    let trip_cost = match harassed && trip < UNREACHABLE {
        true => MEETING_COST * f64::from(trip) + weights.banisher_low_san,
        false => weights.banisher_low_san,
    };
    f64::from(me.san) <= trip_cost
}

/// Whether enough banishers are fit to cast the spell again soon.
pub(super) fn ready_to_cast(state: &GameState, weights: &InvestigatorWeights) -> bool {
    state
        .active_investigators()
        .into_iter()
        .map(|index| &state.investigators[index])
        .filter(|investigator| {
            is_banisher(investigator) && f64::from(investigator.san) >= weights.casting_san
        })
        .count()
        >= state.rules.banishers_needed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::nyog_sothep::BANISHERS_NEEDED;
    use crate::domain::professor_ai::test_state;
    use LocationId::*;

    fn banishers_at(cities: [LocationId; 4]) -> GameState {
        let mut state = test_state();
        state.nyog.location = Some(Concord);
        for (index, at) in cities.into_iter().enumerate() {
            let investigator = &mut state.investigators[index];
            investigator.spell = true;
            investigator.sign = true;
            investigator.location = at;
            investigator.previous = at;
        }
        state
    }

    #[test]
    fn three_banishers_together_need_not_wait_for_the_fourth() {
        let state = banishers_at([Boston, Boston, Boston, Pascoag]);
        assert_eq!(
            rally_point(&state, &InvestigatorWeights::default()),
            Some(Boston)
        );
    }

    #[test]
    fn the_rally_point_is_reachable_around_destroyed_cities() {
        let mut state = banishers_at([Boston, Providence, Worcester, Plymouth]);
        state.destroyed.extend([Dedham, Worcester]);
        let rally = rally_point(&state, &InvestigatorWeights::default()).unwrap();
        assert!(!state.is_destroyed(rally));
        let reaching = banishers(&state)
            .into_iter()
            .filter(|at| walking_steps_from(&state, *at)[rally.index()] < UNREACHABLE)
            .count();
        assert!(reaching >= BANISHERS_NEEDED);
    }

    #[test]
    fn a_harassed_banisher_rests_before_a_long_trip() {
        let mut state = banishers_at([Plymouth, Plymouth, Plymouth, Pascoag]);
        state.investigators[3].san = 8;
        assert_eq!(
            rally_point(&state, &InvestigatorWeights::default()),
            Some(Plymouth)
        );
        state.nyog.location = Some(Woonsocket);
        assert!(banisher_rests(
            &state,
            &InvestigatorWeights::default(),
            &state.investigators[3]
        ));
        state.nyog.location = Some(Provincetown);
        assert!(!banisher_rests(
            &state,
            &InvestigatorWeights::default(),
            &state.investigators[3]
        ));
    }
}
