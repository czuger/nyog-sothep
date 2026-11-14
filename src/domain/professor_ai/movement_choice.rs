//! Where the professor goes each turn.

use rand::Rng;

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{SUMMONING_FANATICS_CITIES, SUMMONING_RANGE};
use crate::domain::professor_ai::fanatics_safety::fanatics_safety;
use crate::domain::professor_ai::near_best::pick_near_best;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::professor_ai::threats::{
    Reach, danger_with, has_investigator, investigator_reach, nearest_with,
};
use crate::domain::professor_belief::ProfessorBelief;
use crate::domain::travel::{self, Network};
use crate::domain::turn::{GameState, ProfessorMove, ProfessorMoveOptions};

/// Steps from the investigators within which a harmful counter is worth laying.
const TRAP_RANGE: u8 = 3;

/// Scores every legal move and picks one of the near-best.
///
/// Before the summoning he drops Fanatics in safe new cities while drifting around
/// Nyog'Sothep's city, avoiding armed investigators; with five Fanatics cities he closes in to
/// summon; once Nyog'Sothep is out, he joins it.
pub fn choose_movement<R: Rng>(
    state: &GameState,
    weights: &ProfessorWeights,
    options: &ProfessorMoveOptions,
    rng: &mut R,
) -> ProfessorMove {
    let exposed = ProfessorBelief::of(state).size() as f64 <= weights.exposed_belief_size;
    let reach = investigator_reach(state);
    let threatened = danger_with(state, weights, &reach, state.professor.location) > 0.0;
    let score = |destination: LocationId, exposed: bool| {
        score(state, weights, &reach, destination, exposed)
    };
    let mut scored: Vec<(ProfessorMove, f64)> = options
        .steps
        .iter()
        .map(|destination| {
            let step = ProfessorMove {
                destination: *destination,
                flying_horror: false,
            };
            (step, score(*destination, exposed))
        })
        .collect();
    let flight = match exposed || threatened {
        true => weights.flight_bonus,
        false => -weights.idle_flight_cost,
    };
    scored.extend(options.flying.iter().map(|destination| {
        let flying = ProfessorMove {
            destination: *destination,
            flying_horror: true,
        };
        (flying, score(*destination, false) + flight)
    }));
    pick_near_best(scored, weights.move_tolerance, rng).unwrap_or(ProfessorMove {
        destination: state.professor.location,
        flying_horror: false,
    })
}

fn score(
    state: &GameState,
    weights: &ProfessorWeights,
    reach: &Reach,
    destination: LocationId,
    exposed: bool,
) -> f64 {
    // Nobody fights the professor once Nyog'Sothep is out: he only has to join it.
    if let Some(nyog) = state.nyog.location {
        return -steps(destination, nyog);
    }
    let here = state.professor.location;
    let mut score = -danger_with(state, weights, reach, destination);
    if destination != here && has_investigator(state, destination) {
        score -= weights.meeting_cost;
    }
    if destination.is_sea() {
        score -= match state.sea_announcements {
            true => weights.announced_sea_cost,
            false => weights.sea_cost,
        };
    }
    if destination == here {
        score -= match exposed {
            true => weights.exposed_stay_cost,
            false => weights.stay_cost,
        };
    }
    let to_goal = steps(destination, state.nyog.city);
    if state.counters.fanatics_cities().len() >= SUMMONING_FANATICS_CITIES {
        let beyond = (to_goal - f64::from(SUMMONING_RANGE)).max(0.0);
        return score - weights.summoning_approach_cost * beyond;
    }
    score -= weights.drift_cost * (to_goal - weights.drift_range).max(0.0);
    score + placement_interest(state, weights, reach, destination)
}

/// The value of being able to place a counter there.
fn placement_interest(
    state: &GameState,
    weights: &ProfessorWeights,
    reach: &Reach,
    destination: LocationId,
) -> f64 {
    if !destination.is_city() || state.counters.placed_at(destination).is_some() {
        return 0.0;
    }
    let hand = state.counters.hand();
    if hand.contains(&CounterKind::Fanatics) {
        return weights.fanatics_place_worth + fanatics_safety(state, weights, reach, destination);
    }
    let harmful = hand.iter().any(|kind| kind.is_placeable());
    match nearest_with(reach, destination) {
        Some(steps) if harmful && (1..=TRAP_RANGE).contains(&steps) => weights.trap_place_worth,
        _ => 0.0,
    }
}

fn steps(from: LocationId, to: LocationId) -> f64 {
    f64::from(travel::distance(from, to, Network::All, |_| true).unwrap_or(12))
}
