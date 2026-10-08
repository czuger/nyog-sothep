//! Which counter the professor leaves where he stands.

use crate::domain::counter_kind::CounterKind;
use crate::domain::professor_ai::counter_values::{FANATICS_TARGET, counter_value};
use crate::domain::professor_ai::threats::{investigator_reach, nearest_investigator};
use crate::domain::table1::HAND_LIMIT;
use crate::domain::turn::GameState;

/// Picks the counter to place among the legal ones.
///
/// Fanatics go where no armed investigator can strike at once; near the investigators a
/// harmful counter suited to them is laid on their path; with a full hand, the least useful
/// counter is dropped rather than wasted at the draw.
pub fn choose_placement(state: &GameState, options: &[CounterKind]) -> Option<CounterKind> {
    let city = state.professor.location;
    let armed_close = investigator_reach(state).into_iter().any(|(index, reach)| {
        state.investigators[index].weapon && reach[city.index()].is_some_and(|steps| steps <= 1)
    });
    let fanatics_wanted = state.counters.fanatics_cities().len() < FANATICS_TARGET;
    if options.contains(&CounterKind::Fanatics) && fanatics_wanted && !armed_close {
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
            .min_by(|a, b| counter_value(state, *a).total_cmp(&counter_value(state, *b)));
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
