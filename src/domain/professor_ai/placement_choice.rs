//! Which counter the professor leaves where he stands.

use crate::domain::counter_kind::CounterKind;
use crate::domain::professor_ai::counter_values::counter_value;
use crate::domain::professor_ai::fanatics_safety::fanatics_safety;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::professor_ai::threats::{investigator_reach, nearest_investigator};
use crate::domain::table1::HAND_LIMIT;
use crate::domain::turn::GameState;

/// Picks the counter to place among the legal ones.
///
/// Fanatics go where they are safe enough; near the investigators a
/// harmful counter suited to them is laid on their path; with a full hand, the least useful
/// counter is dropped rather than wasted at the draw.
pub fn choose_placement(
    state: &GameState,
    weights: &ProfessorWeights,
    options: &[CounterKind],
) -> Option<CounterKind> {
    let city = state.professor.location;
    let safety = fanatics_safety(state, weights, &investigator_reach(state), city);
    let fanatics_wanted = (state.counters.fanatics_cities().len() as f64) < weights.fanatics_target;
    let safe = safety >= weights.safe_enough;
    if options.contains(&CounterKind::Fanatics) && fanatics_wanted && safe {
        return Some(CounterKind::Fanatics);
    }
    if nearest_investigator(state, city).is_some_and(|steps| steps <= 3)
        && let Some(trap) = best_trap(state, options)
    {
        return Some(trap);
    }
    if state.counters.hand().len() >= HAND_LIMIT {
        return options
            .iter()
            .copied()
            .filter(|kind| *kind != CounterKind::Fanatics || !fanatics_wanted)
            .min_by(|a, b| {
                counter_value(state, weights, *a).total_cmp(&counter_value(state, weights, *b))
            });
    }
    None
}

/// The harmful counter that suits the investigators best.
fn best_trap(state: &GameState, options: &[CounterKind]) -> Option<CounterKind> {
    let active: Vec<_> = state
        .investigators
        .iter()
        .filter(|investigator| investigator.is_active())
        .collect();
    let medallions = active.iter().any(|investigator| investigator.medallion);
    let unarmed = active.iter().any(|investigator| !investigator.weapon);
    let preferences = [
        (CounterKind::Inhabitants, medallions),
        (CounterKind::Ghouls, unarmed),
        (CounterKind::Storm, true),
        (CounterKind::ThingsOfTheMist, true),
        (CounterKind::DeepOnes, true),
        (CounterKind::Dreams, true),
        (CounterKind::Ghouls, true),
    ];
    preferences
        .into_iter()
        .find(|(kind, suits)| *suits && options.contains(kind))
        .map(|(kind, _)| kind)
}
