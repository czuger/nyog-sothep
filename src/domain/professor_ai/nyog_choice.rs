//! Where Nyog'Sothep goes once the professor has joined it.

use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::BANISHERS_NEEDED;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::travel::{self, Network};
use crate::domain::turn::GameState;

/// SAN at which meeting the Foul One may drive an investigator with a sign mad.
const BREAKING_SAN: i32 = 4;
/// A city is destroyed on entry, the sea is not.
const CITY_BONUS: f64 = 0.5;
/// Stepping onto three spell holders lets them cast the spell.
const SPELL_RISK: f64 = 50.0;

/// Hunts the investigators: those without a protective sign die on meeting the Foul One, those
/// with little SAN left go mad, and spell holders matter most.
///
/// Each investigator draws Nyog'Sothep in proportion to their worth and inversely to their
/// distance; those at sea, who cannot be met there, only count when nobody stands on land.
pub fn choose_nyog_step(
    state: &GameState,
    weights: &ProfessorWeights,
    options: &[LocationId],
) -> LocationId {
    let preys = preys(state, weights);
    let score = |to: LocationId| {
        let reach = travel::distances(to, Network::All, |_| true);
        let pull: f64 = preys
            .iter()
            .filter_map(|(at, worth)| {
                reach[at.index()].map(|steps| worth / (1.0 + f64::from(steps)))
            })
            .sum();
        let city = if to.is_city() { CITY_BONUS } else { 0.0 };
        pull + city - spell_risk(state, to)
    };
    options
        .iter()
        .copied()
        .max_by(|a, b| score(*a).total_cmp(&score(*b)))
        .unwrap_or(state.professor.location)
}

/// The investigators Nyog'Sothep hunts, with where they stand and what meeting them is worth.
fn preys(state: &GameState, weights: &ProfessorWeights) -> Vec<(LocationId, f64)> {
    let all: Vec<(LocationId, f64)> = state
        .active_investigators()
        .into_iter()
        .map(|index| {
            let investigator = &state.investigators[index];
            let worth = match investigator.sign && investigator.san > BREAKING_SAN {
                true => weights.nyog_harm_worth,
                false => weights.nyog_kill_worth,
            };
            let spell = match investigator.spell {
                true => weights.nyog_spell_factor,
                false => 1.0,
            };
            (investigator.location, worth * spell)
        })
        .collect();
    let on_land: Vec<(LocationId, f64)> =
        all.iter().copied().filter(|(at, _)| at.is_city()).collect();
    match on_land.is_empty() {
        true => all,
        false => on_land,
    }
}

fn spell_risk(state: &GameState, to: LocationId) -> f64 {
    let casters = state
        .investigators_at(to)
        .into_iter()
        .filter(|index| state.investigators[*index].spell)
        .count();
    match casters >= BANISHERS_NEEDED {
        true => SPELL_RISK,
        false => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;
    use LocationId::*;

    #[test]
    fn investigators_at_sea_are_left_for_those_on_land() {
        let mut state = test_state();
        state.nyog.location = Some(CapeCodBay);
        state.professor.location = CapeCodBay;
        for (index, at) in [CapeCodBay, CapeCodBay, CapeCodBay, Boston]
            .into_iter()
            .enumerate()
        {
            state.investigators[index].location = at;
        }
        let options = travel::neighbours(CapeCodBay).to_vec();
        let to = choose_nyog_step(&state, &ProfessorWeights::default(), &options);
        let from_boston = travel::distance(Boston, CapeCodBay, Network::All, |_| true).unwrap();
        let now = travel::distance(Boston, to, Network::All, |_| true).unwrap();
        assert!(now < from_boston, "{to:?}");
    }

    #[test]
    fn three_spell_holders_are_avoided() {
        let mut state = test_state();
        state.nyog.location = Some(Lynn);
        state.professor.location = Lynn;
        for investigator in state.investigators.iter_mut().take(3) {
            investigator.location = Boston;
            investigator.spell = true;
        }
        let options = travel::neighbours(Lynn).to_vec();
        assert!(options.contains(&Boston));
        let weights = ProfessorWeights::default();
        assert_ne!(choose_nyog_step(&state, &weights, &options), Boston);
    }
}
