//! Where the professor goes each turn.

use rand::Rng;

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{SUMMONING_FANATICS_CITIES, SUMMONING_RANGE};
use crate::domain::professor_ai::near_best::pick_near_best;
use crate::domain::professor_ai::threats::{
    Reach, danger_with, has_investigator, investigator_reach, nearest_with,
};
use crate::domain::professor_belief::ProfessorBelief;
use crate::domain::travel::{self, Network};
use crate::domain::turn::{GameState, ProfessorMove, ProfessorMoveOptions};

/// A sighting this recent, or this narrow a belief, makes the professor want to vanish.
const EXPOSED_BELIEF_SIZE: usize = 3;

/// Scores every legal move and picks one of the near-best.
///
/// Before the summoning he drops Fanatics in new cities while drifting around Nyog'Sothep's
/// city, avoiding armed investigators; with five Fanatics cities he closes in to summon; once
/// Nyog'Sothep is out, he joins it.
pub fn choose_movement<R: Rng>(
    state: &GameState,
    options: &ProfessorMoveOptions,
    rng: &mut R,
) -> ProfessorMove {
    let exposed = ProfessorBelief::of(state).size() <= EXPOSED_BELIEF_SIZE;
    let reach = investigator_reach(state);
    let threatened = danger_with(state, &reach, state.professor.location) > 0.0;
    let mut scored: Vec<(ProfessorMove, f64)> = options
        .steps
        .iter()
        .map(|destination| {
            let step = ProfessorMove {
                destination: *destination,
                flying_horror: false,
            };
            (step, score(state, &reach, *destination, exposed))
        })
        .collect();
    let flying_bonus = if exposed || threatened { 4.0 } else { -8.0 };
    scored.extend(options.flying.iter().map(|destination| {
        let flight = ProfessorMove {
            destination: *destination,
            flying_horror: true,
        };
        (
            flight,
            score(state, &reach, *destination, false) + flying_bonus,
        )
    }));
    pick_near_best(scored, 1.5, rng).unwrap_or(ProfessorMove {
        destination: state.professor.location,
        flying_horror: false,
    })
}

fn score(state: &GameState, reach: &Reach, destination: LocationId, exposed: bool) -> f64 {
    if let Some(nyog) = state.nyog.location {
        return -6.0 * steps(destination, nyog) - 0.2 * danger_with(state, reach, destination);
    }
    let here = state.professor.location;
    let mut score = -danger_with(state, reach, destination);
    if destination != here && has_investigator(state, destination) {
        score -= 6.0;
    }
    if destination.is_sea() {
        score -= if state.sea_announcements { 6.0 } else { 3.0 };
    }
    if destination == here {
        score -= if exposed { 4.0 } else { 1.0 };
    }
    let to_goal = steps(destination, state.nyog.city);
    let fanatics = state.counters.fanatics_cities();
    if fanatics.len() >= SUMMONING_FANATICS_CITIES {
        return score - 4.0 * (to_goal - f64::from(SUMMONING_RANGE)).max(0.0);
    }
    score -= 0.7 * (to_goal - 3.0).max(0.0);
    score + placement_interest(state, reach, destination)
}

/// The value of being able to place a counter there.
fn placement_interest(state: &GameState, reach: &Reach, destination: LocationId) -> f64 {
    if !destination.is_city() || state.counters.placed_at(destination).is_some() {
        return 0.0;
    }
    let hand = state.counters.hand();
    if hand.contains(&CounterKind::Fanatics) {
        return 6.0;
    }
    let harmful = hand.iter().any(|kind| kind.is_placeable());
    match nearest_with(reach, destination) {
        Some(1..=3) if harmful => 2.0,
        _ => 0.0,
    }
}

fn steps(from: LocationId, to: LocationId) -> f64 {
    f64::from(travel::distance(from, to, Network::All, |_| true).unwrap_or(12))
}
