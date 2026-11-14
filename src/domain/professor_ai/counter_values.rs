//! How much each counter is worth to the professor, for Table 1 choices and discards.

use crate::domain::counter_kind::CounterKind;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::turn::GameState;

/// The worth of holding one more counter of this kind.
pub fn counter_value(state: &GameState, weights: &ProfessorWeights, kind: CounterKind) -> f64 {
    match kind {
        CounterKind::Fanatics => {
            let placed = state.counters.fanatics_cities().len();
            let held = state
                .counters
                .hand()
                .into_iter()
                .filter(|held| *held == CounterKind::Fanatics)
                .count();
            if ((placed + held) as f64) < weights.fanatics_target {
                10.0
            } else {
                3.0
            }
        }
        CounterKind::FlyingHorror => 9.0,
        CounterKind::Teleportation => 7.0,
        CounterKind::Ghouls => 6.0,
        CounterKind::Storm | CounterKind::ThingsOfTheMist => 5.0,
        CounterKind::DeepOnes | CounterKind::Dreams => 4.0,
        CounterKind::Inhabitants => {
            let medallions = state
                .investigators
                .iter()
                .any(|investigator| investigator.is_active() && investigator.medallion);
            if medallions { 5.0 } else { 2.0 }
        }
    }
}

/// The better of the two counters a Table 1 roll offers.
pub fn choose_table1(
    state: &GameState,
    weights: &ProfessorWeights,
    options: &[CounterKind],
) -> CounterKind {
    best_by(state, weights, options, f64::total_cmp).unwrap_or(CounterKind::Fanatics)
}

/// The least useful counter of the hand.
pub fn choose_discard(
    state: &GameState,
    weights: &ProfessorWeights,
    hand: &[CounterKind],
) -> CounterKind {
    best_by(state, weights, hand, |a, b| b.total_cmp(a)).unwrap_or(CounterKind::Dreams)
}

fn best_by(
    state: &GameState,
    weights: &ProfessorWeights,
    kinds: &[CounterKind],
    order: impl Fn(&f64, &f64) -> std::cmp::Ordering,
) -> Option<CounterKind> {
    kinds.iter().copied().max_by(|a, b| {
        order(
            &counter_value(state, weights, *a),
            &counter_value(state, weights, *b),
        )
    })
}
