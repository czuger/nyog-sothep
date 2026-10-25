//! Where each investigator goes.

use rand::Rng;

use crate::domain::investigator::Investigator;
use crate::domain::investigator_ai::paths::{UNREACHABLE, steps, steps_from, steps_to_nearest};
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::nyog_sothep::BANISHERS_NEEDED;
use crate::domain::professor_ai::pick_near_best;
use crate::domain::professor_belief::ProfessorBelief;
use crate::domain::turn::{GameState, MoveOptions, MoveOrder};

/// SAN at which an investigator goes to the psychoanalyst, keeping enough to face Nyog'Sothep
/// (4 SAN at the first meeting, and up to 4 more if the spell fails).
const LOW_SAN: i32 = 8;
/// Once Nyog'Sothep is out, the banishers only rest when this low: time counts.
const BANISHER_LOW_SAN: i32 = 5;
/// A belief this narrow is hunted location by location; a wider one, from its centre.
const NARROW_BELIEF: usize = 8;
/// Moves scoring this close to the best are picked at random.
const TOLERANCE: f64 = 0.5;

/// What the investigators know when they choose their moves.
struct Knowledge {
    /// Cities where the professor may be.
    belief: Vec<LocationId>,
    /// Whether that set is narrow.
    narrow: bool,
}

impl Knowledge {
    fn of(state: &GameState) -> Knowledge {
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
            narrow: belief.size() <= NARROW_BELIEF,
            belief: cities,
        }
    }
}

/// The orders of the movement phase, investigator by investigator; those who move later keep
/// away from where the earlier ones are going.
pub(super) fn choose_moves<R: Rng>(state: &GameState, rng: &mut R) -> Vec<(usize, MoveOrder)> {
    let knowledge = Knowledge::of(state);
    let mut taken: Vec<LocationId> = Vec::new();
    let mut orders = Vec::new();
    for index in 0..state.investigators.len() {
        let Some(options) = MoveOptions::of(state, index, false) else {
            continue;
        };
        let order = choose_order(state, index, &options, &knowledge, &taken, rng);
        taken.push(match order {
            MoveOrder::Go(to) => to,
            _ => state.investigators[index].location,
        });
        orders.push((index, order));
    }
    orders
}

/// The « play again » move of an investigator.
pub(super) fn choose_replay<R: Rng>(
    state: &GameState,
    investigator: usize,
    rng: &mut R,
) -> MoveOrder {
    let Some(options) = MoveOptions::of(state, investigator, true) else {
        return MoveOrder::Stay;
    };
    let others: Vec<LocationId> = state
        .active_investigators()
        .into_iter()
        .filter(|index| *index != investigator)
        .map(|index| state.investigators[index].location)
        .collect();
    choose_order(
        state,
        investigator,
        &options,
        &Knowledge::of(state),
        &others,
        rng,
    )
}

fn choose_order<R: Rng>(
    state: &GameState,
    index: usize,
    options: &MoveOptions,
    knowledge: &Knowledge,
    others: &[LocationId],
    rng: &mut R,
) -> MoveOrder {
    let me = &state.investigators[index];
    if options.psychoanalyst && rests(state, me) {
        return MoveOrder::Psychoanalyst;
    }
    let here = me.location;
    let mut scored = vec![(
        MoveOrder::Stay,
        score(state, index, here, knowledge, others),
    )];
    scored.extend(options.destinations.iter().map(|to| {
        let order = MoveOrder::Go(*to);
        (order, score(state, index, *to, knowledge, others))
    }));
    // Staying with a broken car loses the turn.
    if me.car_broken {
        scored.retain(|(order, _)| *order != MoveOrder::Stay);
    }
    pick_near_best(scored, TOLERANCE, rng).unwrap_or(MoveOrder::Stay)
}

/// Whether an investigator stays to consult a psychoanalyst. A banisher waiting at the rally
/// point does when enough banishers remain, since staying there costs nothing and spares an
/// event; with too few, events are their only way to complete the group.
fn rests(state: &GameState, me: &Investigator) -> bool {
    if !(state.nyog.is_summoned() && is_banisher(me)) {
        return me.san <= LOW_SAN;
    }
    let waiting =
        banishers(state).len() >= BANISHERS_NEEDED && rally_point(state) == Some(me.location);
    me.san <= BANISHER_LOW_SAN || waiting
}

/// How good it is for an investigator to end the move in `to`.
///
/// Everyone drifts towards where the professor may be; a city is worth a little more than the
/// sea (an event is drawn there), but never enough to keep an investigator on an island. The
/// unarmed spread out, keep off known counters, and off a narrow belief unless a medallion
/// hides them.
fn score(
    state: &GameState,
    index: usize,
    to: LocationId,
    knowledge: &Knowledge,
    others: &[LocationId],
) -> f64 {
    let me = &state.investigators[index];
    if let Some(nyog) = state.nyog.location {
        return after_summoning(state, index, to, nyog);
    }
    let land = if to.is_city() { 0.5 } else { 0.0 };
    let spread = f64::from(steps_to_nearest(to, others).min(3)) * 0.5;
    if me.weapon {
        // Hunters cover different cities of the belief.
        return land + 0.6 * spread - hunting_distance(to, knowledge);
    }
    let known_counter = state.counters.placed_at(to).is_some();
    let near_professor = knowledge.narrow && knowledge.belief.contains(&to) && !me.medallion;
    land + spread
        - 0.7 * hunting_distance(to, knowledge)
        - 4.0 * f64::from(u8::from(known_counter))
        - 3.0 * f64::from(u8::from(near_professor))
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

/// Once Nyog'Sothep is out, the spell holders protected by a sign gather to banish it; the
/// others keep away from it, the unprotected most of all.
fn after_summoning(state: &GameState, index: usize, to: LocationId, nyog: LocationId) -> f64 {
    let me = &state.investigators[index];
    if to.is_city() && state.is_destroyed(to) {
        return -100.0;
    }
    if is_banisher(me) {
        return rally_point(state).map_or(0.0, |rally| -f64::from(steps(to, rally)));
    }
    let away = f64::from(steps(to, nyog).min(UNREACHABLE));
    match me.sign {
        true => away.min(3.0),
        false => away,
    }
}

/// An investigator who can cast the spell and survive meeting Nyog'Sothep.
fn is_banisher(investigator: &Investigator) -> bool {
    investigator.spell && investigator.sign
}

/// Where the active banishers stand.
fn banishers(state: &GameState) -> Vec<LocationId> {
    state
        .active_investigators()
        .into_iter()
        .filter(|index| is_banisher(&state.investigators[*index]))
        .map(|index| state.investigators[index].location)
        .collect()
}

/// The open city closest, in total, to the active banishers.
fn rally_point(state: &GameState) -> Option<LocationId> {
    let from_holders: Vec<Vec<u32>> = banishers(state).into_iter().map(steps_from).collect();
    map::cities()
        .filter(|city| !state.is_destroyed(*city))
        .min_by_key(|city| from_holders.iter().map(|steps| steps[city.index()]).max())
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
            let orders = choose_moves(&state, &mut rand::rng());
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
        let orders = choose_moves(&state, &mut rand::rng());
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
        let rally = rally_point(&state).unwrap();
        let orders = choose_moves(&state, &mut rand::rng());
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
