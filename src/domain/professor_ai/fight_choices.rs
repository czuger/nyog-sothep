//! The professor's choices around fights: attacking, retreating, carrying, teleporting.

use crate::domain::counter::CounterPlace;
use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::professor_ai::threats::{danger, nearest_investigator};
use crate::domain::travel::{self, Network};
use crate::domain::turn::{GameState, Teleport};

/// An attack is worth making from this score on.
const WORTH_ATTACKING: f64 = 8.0;

/// Picks the investigator to attack on arrival, if any.
///
/// Forced by the die, he attacks the one who can hurt him least; otherwise he only attacks
/// someone with nothing to defend themselves, and never with an armed investigator in the city.
pub fn choose_attack(state: &GameState, candidates: &[usize], forced: bool) -> Option<usize> {
    let best = candidates
        .iter()
        .copied()
        .max_by(|a, b| victim_score(state, *a).total_cmp(&victim_score(state, *b)))?;
    let armed_here = candidates
        .iter()
        .any(|index| state.investigators[*index].weapon);
    match forced || (victim_score(state, best) >= WORTH_ATTACKING && !armed_here) {
        true => Some(best),
        false => None,
    }
}

fn victim_score(state: &GameState, index: usize) -> f64 {
    let victim = &state.investigators[index];
    if victim.weapon {
        -10.0
    } else if victim.sign {
        2.0
    } else if victim.medallion {
        0.0
    } else if victim.san <= 4 {
        18.0
    } else {
        WORTH_ATTACKING
    }
}

/// Picks where to retreat or be pushed: away from armed investigators, preferably on land.
pub fn choose_relocation(state: &GameState, options: &[LocationId]) -> LocationId {
    best_of(options, |location| {
        let sea = if location.is_sea() { 2.0 } else { 0.0 };
        let distance = f64::from(nearest_investigator(state, location).unwrap_or(6).min(6));
        distance - danger(state, location) - sea
    })
    .unwrap_or(state.professor.location)
}

/// Picks where to carry an unarmed investigator: far away, or onto a hidden trap.
pub fn choose_carry(state: &GameState, options: &[LocationId]) -> LocationId {
    best_of(options, |location| {
        steps(location, state.professor.location)
            + steps(location, state.nyog.city)
            + trap_bonus(state, location)
    })
    .unwrap_or(state.professor.location)
}

/// Picks a Teleportation: an armed investigator close by is sent away (or, after the
/// summoning, a spell holder).
pub fn choose_teleport(state: &GameState, options: &[Teleport]) -> Option<Teleport> {
    let summoned = state.nyog.is_summoned();
    let worth: Vec<Teleport> = options
        .iter()
        .copied()
        .filter(|teleport| {
            let target = &state.investigators[teleport.investigator];
            target.weapon || (summoned && target.spell)
        })
        .collect();
    best_of(&worth, |teleport| {
        steps(teleport.destination, state.professor.location)
            + trap_bonus(state, teleport.destination)
    })
}

/// A hidden counter waiting in a city, which the investigator will meet.
fn trap_bonus(state: &GameState, location: LocationId) -> f64 {
    match state.counters.placed_at(location) {
        Some(counter)
            if matches!(
                counter.place,
                CounterPlace::Placed {
                    discovered: false,
                    ..
                }
            ) =>
        {
            if counter.kind == CounterKind::Fanatics {
                1.0
            } else {
                5.0
            }
        }
        _ => 0.0,
    }
}

fn steps(from: LocationId, to: LocationId) -> f64 {
    f64::from(travel::distance(from, to, Network::All, |_| true).unwrap_or(12))
}

fn best_of<T: Copy>(options: &[T], score: impl Fn(T) -> f64) -> Option<T> {
    options
        .iter()
        .copied()
        .max_by(|a, b| score(*a).total_cmp(&score(*b)))
}
