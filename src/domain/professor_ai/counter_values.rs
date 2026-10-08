//! How much each counter is worth to the professor, for Table 1 choices and discards.

use crate::domain::counter_kind::CounterKind;
use crate::domain::turn::GameState;

/// Fanatics cities the professor aims for, a little above the five needed in case some die.
pub const FANATICS_TARGET: usize = 7;

/// The worth of holding one more counter of this kind.
pub fn counter_value(state: &GameState, kind: CounterKind) -> f64 {
    match kind {
        CounterKind::Fanatics => {
            let placed = state.counters.fanatics_cities().len();
            let held = state
                .counters
                .hand()
                .into_iter()
                .filter(|held| *held == CounterKind::Fanatics)
                .count();
            if placed + held < FANATICS_TARGET {
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
pub fn choose_table1(state: &GameState, options: &[CounterKind]) -> CounterKind {
    best_by(state, options, f64::total_cmp).unwrap_or(CounterKind::Fanatics)
}

/// The least useful counter of the hand.
pub fn choose_discard(state: &GameState, hand: &[CounterKind]) -> CounterKind {
    best_by(state, hand, |a, b| b.total_cmp(a)).unwrap_or(CounterKind::Dreams)
}

fn best_by(
    state: &GameState,
    kinds: &[CounterKind],
    order: impl Fn(&f64, &f64) -> std::cmp::Ordering,
) -> Option<CounterKind> {
    kinds
        .iter()
        .copied()
        .max_by(|a, b| order(&counter_value(state, *a), &counter_value(state, *b)))
}
