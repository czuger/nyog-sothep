//! Table 1: the counters the professor draws with 2d6.

use crate::domain::counter_kind::CounterKind;
use crate::domain::counter_pool::CounterPool;
use crate::domain::dice::Dice;

use CounterKind::*;

/// The most counters the professor may keep after drawing.
pub const HAND_LIMIT: usize = 5;

/// Counters drawn before the first turn.
pub const STARTING_DRAWS: usize = 4;

/// The counters offered by a 2d6 roll; two of them when the professor chooses.
pub fn options(roll: u8) -> &'static [CounterKind] {
    match roll {
        2 => &[FlyingHorror, Ghouls],
        3 => &[Ghouls],
        4 => &[DeepOnes],
        5 => &[ThingsOfTheMist, DeepOnes],
        6 => &[Inhabitants],
        7 | 8 => &[Dreams],
        9 => &[Teleportation, Dreams],
        10 => &[Storm, Fanatics],
        _ => &[Fanatics],
    }
}

/// Rolls 2d6 until the roll offers a counter left in the reserve.
///
/// # Returns
/// The available counters of the roll (two when the professor chooses), or `None` when the
/// reserve is empty.
pub fn roll(pool: &CounterPool, dice: &mut dyn Dice) -> Option<Vec<CounterKind>> {
    if !CounterKind::ALL
        .into_iter()
        .any(|kind| pool.in_reserve(kind))
    {
        return None;
    }
    loop {
        let available: Vec<CounterKind> = options(dice.roll(2))
            .iter()
            .copied()
            .filter(|kind| pool.in_reserve(*kind))
            .collect();
        if !available.is_empty() {
            return Some(available);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dice::ScriptedDice;

    #[test]
    fn choice_rolls_offer_both_counters() {
        let pool = CounterPool::full();
        let mut dice = ScriptedDice::new(&[4, 6, 1, 2], 1);
        assert_eq!(roll(&pool, &mut dice), Some(vec![Storm, Fanatics]));
        assert_eq!(roll(&pool, &mut dice), Some(vec![Ghouls]));
    }

    #[test]
    fn exhausted_kinds_are_rolled_again() {
        let mut pool = CounterPool::full();
        assert!(pool.draw(FlyingHorror));
        let mut dice = ScriptedDice::new(&[1, 1], 1);
        assert_eq!(roll(&pool, &mut dice), Some(vec![Ghouls]));
        for _ in 0..2 {
            pool.draw(Teleportation);
        }
        for _ in 0..6 {
            pool.draw(Dreams);
        }
        // 9 offers Teleportation or Dreams, both gone, so the roll is made again.
        let mut dice = ScriptedDice::new(&[4, 5, 2, 3], 1);
        assert_eq!(
            roll(&pool, &mut dice),
            Some(vec![ThingsOfTheMist, DeepOnes])
        );
    }

    #[test]
    fn an_empty_reserve_gives_nothing() {
        let mut pool = CounterPool::full();
        for kind in CounterKind::ALL {
            while pool.draw(kind) {}
        }
        assert_eq!(roll(&pool, &mut ScriptedDice::new(&[], 1)), None);
    }
}
