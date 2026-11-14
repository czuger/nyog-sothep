//! Where Nyog'Sothep goes once the professor has joined it.

use crate::domain::location_id::LocationId;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::travel::{self, Network};
use crate::domain::turn::{ClueKind, GameState};

/// SAN at which meeting the Foul One may drive an investigator with a sign mad.
const BREAKING_SAN: i32 = 4;
/// A city is destroyed on entry, the sea is not.
const CITY_BONUS: f64 = 0.5;
/// What a banishment costs the professor, weighed by its chance.
const SPELL_RISK: f64 = 50.0;
/// Turns after the summoning over which the fear of the spell fades: the professor cannot win
/// without facing the spell holders in the end.
const PATIENCE: u32 = 12;

/// Hunts the investigators: those without a protective sign die on meeting the Foul One, those
/// with little SAN left go mad, and spell holders matter most.
///
/// Each investigator draws Nyog'Sothep in proportion to their worth and inversely to their
/// distance; those at sea, who cannot be met there, only count when nobody stands on land.
/// Cities where the spell may banish it are avoided, unless every prey stands in one, and less
/// and less as the hunt drags on.
pub fn choose_nyog_step(
    state: &GameState,
    weights: &ProfessorWeights,
    options: &[LocationId],
) -> LocationId {
    let preys = preys(state, weights);
    let rules = state.rules.rules();
    let cornered = preys
        .iter()
        .all(|(at, _)| rules.banishment_chance(state, *at) > 0.0);
    let fear = match cornered {
        true => 0.0,
        false => SPELL_RISK * fading_fear(state),
    };
    let score = |to: LocationId| {
        let reach = travel::distances(to, Network::All, |_| true);
        let pull: f64 = preys
            .iter()
            .filter_map(|(at, worth)| {
                reach[at.index()].map(|steps| worth / (1.0 + f64::from(steps)))
            })
            .sum();
        let city = if to.is_city() { CITY_BONUS } else { 0.0 };
        pull + city - fear * rules.banishment_chance(state, to)
    };
    options
        .iter()
        .copied()
        .max_by(|a, b| score(*a).total_cmp(&score(*b)))
        .unwrap_or(state.professor.location)
}

/// 1 at the summoning, down to 0 after [`PATIENCE`] turns.
fn fading_fear(state: &GameState) -> f64 {
    let summoned = state.clues.iter().find_map(|clue| match clue.kind {
        ClueKind::Summoned { .. } => Some(clue.turn),
        _ => None,
    });
    let hunting = state.turn.saturating_sub(summoned.unwrap_or(state.turn));
    (1.0 - f64::from(hunting) / f64::from(PATIENCE)).max(0.0)
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
