//! Where each investigator goes.

use rand::Rng;

use crate::domain::counter_kind::CounterKind;
use crate::domain::investigator::Investigator;
use crate::domain::investigator_ai::banishers::{banisher_rests, is_banisher, rally_point};
use crate::domain::investigator_ai::hunt::Hunt;
use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::investigator_ai::paths::{
    UNREACHABLE, steps, steps_from, steps_to_nearest, walking_steps_from,
};
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::SUMMONING_FANATICS_CITIES;
use crate::domain::professor_ai::pick_near_best;
use crate::domain::professor_belief::ProfessorBelief;
use crate::domain::travel;
use crate::domain::turn::{GameState, MoveOptions, MoveOrder};

/// Steps away from the others beyond which spreading out is worth nothing more.
const SPREAD_CAP: u32 = 3;
/// Steps from Nyog'Sothep beyond which an investigator with a sign feels safe enough.
const SIGNED_SAFE_DISTANCE: f64 = 3.0;

/// What the investigators know when they choose their moves.
struct Knowledge<'a> {
    /// The weights of the choices.
    weights: &'a InvestigatorWeights,
    /// Cities where the professor may be.
    belief: Vec<LocationId>,
    /// Whether that set is narrow.
    narrow: bool,
}

impl Knowledge<'_> {
    fn of<'a>(state: &GameState, weights: &'a InvestigatorWeights) -> Knowledge<'a> {
        let belief = ProfessorBelief::of(state);
        let mut cities: Vec<LocationId> = belief
            .locations()
            .into_iter()
            .filter(|location| location.is_city())
            .collect();
        if cities.is_empty() {
            cities = belief.locations();
        }
        Knowledge {
            weights,
            narrow: belief.size() as f64 <= weights.narrow_belief,
            belief: cities,
        }
    }
}

/// The orders of the movement phase, investigator by investigator; those who move later keep
/// away from where the earlier ones are going.
pub(super) fn choose_moves<R: Rng>(
    state: &GameState,
    weights: &InvestigatorWeights,
    rng: &mut R,
) -> Vec<(usize, MoveOrder)> {
    let knowledge = Knowledge::of(state, weights);
    let mut hunt = Hunt::of(state, weights);
    let mut taken: Vec<LocationId> = Vec::new();
    let mut orders = Vec::new();
    for index in 0..state.investigators.len() {
        let Some(options) = MoveOptions::of(state, index, false) else {
            continue;
        };
        let order = choose_order(state, index, &options, &knowledge, &hunt, &taken, rng);
        let end = match order {
            MoveOrder::Go(to) => to,
            _ => state.investigators[index].location,
        };
        if state.investigators[index].weapon {
            hunt.cover(end);
        }
        taken.push(end);
        orders.push((index, order));
    }
    orders
}

/// The « play again » move of an investigator.
pub(super) fn choose_replay<R: Rng>(
    state: &GameState,
    weights: &InvestigatorWeights,
    investigator: usize,
    rng: &mut R,
) -> MoveOrder {
    let Some(options) = MoveOptions::of(state, investigator, true) else {
        return MoveOrder::Stay;
    };
    let others: Vec<usize> = state
        .active_investigators()
        .into_iter()
        .filter(|index| *index != investigator)
        .collect();
    let mut hunt = Hunt::of(state, weights);
    for index in &others {
        if state.investigators[*index].weapon {
            hunt.cover(state.investigators[*index].location);
        }
    }
    let others: Vec<LocationId> = others
        .into_iter()
        .map(|index| state.investigators[index].location)
        .collect();
    choose_order(
        state,
        investigator,
        &options,
        &Knowledge::of(state, weights),
        &hunt,
        &others,
        rng,
    )
}

fn choose_order<R: Rng>(
    state: &GameState,
    index: usize,
    options: &MoveOptions,
    knowledge: &Knowledge,
    hunt: &Hunt,
    others: &[LocationId],
    rng: &mut R,
) -> MoveOrder {
    let me = &state.investigators[index];
    let weights = knowledge.weights;
    if options.psychoanalyst && rests(state, weights, me) {
        return MoveOrder::Psychoanalyst;
    }
    let here = me.location;
    let mut scored = vec![(
        MoveOrder::Stay,
        score(state, index, here, knowledge, hunt, others),
    )];
    scored.extend(options.destinations.iter().map(|to| {
        let order = MoveOrder::Go(*to);
        (order, score(state, index, *to, knowledge, hunt, others))
    }));
    // Staying with a broken car loses the turn.
    if me.car_broken {
        scored.retain(|(order, _)| *order != MoveOrder::Stay);
    }
    let order = pick_near_best(scored, weights.move_tolerance, rng).unwrap_or(MoveOrder::Stay);
    // Once Nyog'Sothep is out, events are mostly a risk: staying is better spent resting.
    match order == MoveOrder::Stay && options.psychoanalyst && state.nyog.is_summoned() {
        true => MoveOrder::Psychoanalyst,
        false => order,
    }
}

/// Whether an investigator stays to consult a psychoanalyst.
fn rests(state: &GameState, weights: &InvestigatorWeights, me: &Investigator) -> bool {
    match state.nyog.is_summoned() && is_banisher(me) {
        true => banisher_rests(state, weights, me),
        false => f64::from(me.san) <= weights.low_san,
    }
}

/// How good it is for an investigator to end the move in `to`.
///
/// Everyone drifts towards where the professor may be; a city is worth a little more than the
/// sea (an event is drawn there), but never enough to keep an investigator on an island.
/// Hunters share out his likely positions and attack the Fanatics found; the unarmed spread
/// out, keep off known counters, and off a narrow belief unless a medallion hides them.
fn score(
    state: &GameState,
    index: usize,
    to: LocationId,
    knowledge: &Knowledge,
    hunt: &Hunt,
    others: &[LocationId],
) -> f64 {
    let me = &state.investigators[index];
    let weights = knowledge.weights;
    if let Some(nyog) = state.nyog.location {
        return after_summoning(state, weights, index, to, nyog);
    }
    let land = if to.is_city() {
        weights.land_worth
    } else {
        0.0
    };
    let spread = f64::from(steps_to_nearest(to, others).min(SPREAD_CAP)) * weights.spread_worth;
    if me.weapon {
        return land + weights.hunter_spread_factor * spread - hunting_distance(to, knowledge)
            + hunt.worth(to)
            + fanatics_bonus(state, weights, me, to);
    }
    let known_counter = state.counters.placed_at(to).is_some();
    let near_professor = knowledge.narrow && knowledge.belief.contains(&to) && !me.medallion;
    land + spread
        - weights.unarmed_hunting_factor * hunting_distance(to, knowledge)
        - weights.known_counter_cost * f64::from(u8::from(known_counter))
        - weights.near_professor_cost * f64::from(u8::from(near_professor))
}

/// Armed investigators close in on the professor: the nearest possible city when few remain,
/// otherwise the centre of the possible cities.
fn hunting_distance(to: LocationId, knowledge: &Knowledge) -> f64 {
    if knowledge.narrow {
        return f64::from(steps_to_nearest(to, &knowledge.belief));
    }
    let from_here = steps_from(to);
    let total: u32 = knowledge
        .belief
        .iter()
        .map(|at| from_here[at.index()])
        .sum();
    f64::from(total) / knowledge.belief.len().max(1) as f64
}

/// What an armed investigator gains by walking into face-up Fanatics: a chance to kill them,
/// better with a medallion, and worth more as the Fanatics left near the five needed.
fn fanatics_bonus(
    state: &GameState,
    weights: &InvestigatorWeights,
    me: &Investigator,
    to: LocationId,
) -> f64 {
    let fanatics_here = state
        .counters
        .placed_at(to)
        .is_some_and(|counter| counter.kind == CounterKind::Fanatics);
    if !fanatics_here {
        return 0.0;
    }
    let kill_chance = if me.medallion { 2.0 / 6.0 } else { 1.0 / 6.0 };
    let spare = state
        .counters
        .fanatics_in_play()
        .saturating_sub(SUMMONING_FANATICS_CITIES);
    weights.fanatics_kill_worth * kill_chance / (1.0 + spare as f64)
}

/// Once Nyog'Sothep is out, the spell holders protected by a sign gather to banish it; the
/// others keep away from it, the unprotected most of all. Nobody wants to be shut in by ruins.
fn after_summoning(
    state: &GameState,
    weights: &InvestigatorWeights,
    index: usize,
    to: LocationId,
    nyog: LocationId,
) -> f64 {
    let me = &state.investigators[index];
    if to.is_city() && state.is_destroyed(to) {
        return -100.0;
    }
    let goal = if is_banisher(me) {
        rally_point(state, weights).map_or(0.0, |rally| {
            -f64::from(walking_steps_from(state, to)[rally.index()])
        })
    } else {
        let away = f64::from(steps(to, nyog).min(UNREACHABLE));
        match me.sign {
            true => away.min(SIGNED_SAFE_DISTANCE),
            false => away,
        }
    };
    goal - trap_risk(state, weights, to)
}

/// The risk of being shut in by destroyed cities; the sea is never destroyed, so it is always a
/// way out.
fn trap_risk(state: &GameState, weights: &InvestigatorWeights, to: LocationId) -> f64 {
    let neighbours = travel::neighbours(to);
    if to.is_sea() || neighbours.iter().any(|next| next.is_sea()) {
        return 0.0;
    }
    let exits = neighbours
        .iter()
        .filter(|next| !state.is_destroyed(**next))
        .count();
    match exits {
        0 => weights.shut_in_cost,
        1 => weights.dead_end_cost,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;
    use crate::domain::turn::{Clue, ClueKind};
    use LocationId::*;

    #[test]
    fn an_armed_investigator_closes_in_on_a_sighting() {
        let mut state = test_state();
        state.clues = vec![Clue {
            turn: 1,
            kind: ClueKind::Spotted { at: Boston },
        }];
        let hunter = &mut state.investigators[0];
        hunter.weapon = true;
        hunter.location = Plymouth;
        hunter.previous = Plymouth;
        let before = steps(Plymouth, Boston);
        for _ in 0..20 {
            let orders = choose_moves(&state, &InvestigatorWeights::default(), &mut rand::rng());
            let MoveOrder::Go(to) = orders[0].1 else {
                panic!("the hunter stays: {orders:?}");
            };
            assert!(steps(to, Boston) < before, "{to:?}");
        }
    }

    #[test]
    fn a_tired_investigator_rests() {
        let mut state = test_state();
        state.investigators[1].san = 3;
        let orders = choose_moves(&state, &InvestigatorWeights::default(), &mut rand::rng());
        assert_eq!(orders[1], (1, MoveOrder::Psychoanalyst));
    }

    #[test]
    fn spell_holders_gather_once_nyog_sothep_is_out() {
        let mut state = test_state();
        state.nyog.location = Some(Concord);
        for (index, at) in [Boston, Providence, Worcester, Plymouth]
            .into_iter()
            .enumerate()
        {
            let investigator = &mut state.investigators[index];
            investigator.spell = true;
            investigator.sign = true;
            investigator.location = at;
            investigator.previous = at;
        }
        let rally = rally_point(&state, &InvestigatorWeights::default()).unwrap();
        let orders = choose_moves(&state, &InvestigatorWeights::default(), &mut rand::rng());
        for (index, order) in orders {
            let from = state.investigators[index].location;
            let to = match order {
                MoveOrder::Go(to) => to,
                _ => from,
            };
            assert!(steps(to, rally) <= steps(from, rally));
        }
    }
}
