//! The professor's answers when he must name cities: always truthful, as misleading as possible.

use rand::Rng;

use crate::domain::counter::CounterPlace;
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_ai::near_best::pick_near_best;
use crate::domain::professor_ai::professor_belief::ProfessorBelief;
use crate::domain::professor_ai::threats::{has_investigator, nearest_investigator};
use crate::domain::travel::{self, Network};
use crate::domain::turn::GameState;

/// Names `count` cities including his own, the decoys being far from it and still believable.
pub fn choose_names<R: Rng>(state: &GameState, count: usize, rng: &mut R) -> Vec<LocationId> {
    let real = state.professor.location;
    let belief = ProfessorBelief::of(state);
    let from_real = travel::distances(real, Network::All, |_| true);
    let mut candidates: Vec<(LocationId, f64)> = map::cities()
        .filter(|city| *city != real)
        .map(|city| {
            let believable = if belief.contains(city) { 6.0 } else { 0.0 };
            let far = f64::from(from_real[city.index()].unwrap_or(6).min(6));
            let seen_empty = if has_investigator(state, city) {
                4.0
            } else {
                0.0
            };
            (city, believable + far - seen_empty)
        })
        .collect();
    let mut named = vec![real];
    while named.len() < count && !candidates.is_empty() {
        let Some(decoy) = pick_near_best(candidates.clone(), 2.0, rng) else {
            break;
        };
        candidates.retain(|(city, _)| *city != decoy);
        named.push(decoy);
    }
    named.sort();
    named
}

/// Names a Fanatics city out of the armed investigators' way, and a decoy that lures them onto
/// a hidden trap.
pub fn choose_fanatics_names<R: Rng>(state: &GameState, rng: &mut R) -> Vec<LocationId> {
    let fanatics = state.counters.fanatics_cities();
    let Some(real) = fanatics
        .iter()
        .copied()
        .max_by_key(|city| nearest_investigator(state, *city).unwrap_or(u8::MAX))
    else {
        return Vec::new();
    };
    let decoys: Vec<(LocationId, f64)> = map::cities()
        .filter(|city| !fanatics.contains(city))
        .filter_map(|city| {
            let counter = state.counters.placed_at(city);
            let shown = counter.is_some_and(|counter| {
                matches!(
                    counter.place,
                    CounterPlace::Placed {
                        discovered: true,
                        ..
                    }
                )
            });
            if shown {
                return None;
            }
            let trap = if counter.is_some() { 5.0 } else { 0.0 };
            let lure = 6.0 - f64::from(nearest_investigator(state, city).unwrap_or(6).min(6));
            Some((city, trap + lure))
        })
        .collect();
    let mut named = vec![real];
    named.extend(pick_near_best(decoys, 2.0, rng));
    named.sort();
    named
}
