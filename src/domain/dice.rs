//! Dice rolls, injected so that tests can script them.

use std::collections::VecDeque;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

/// A source of six-sided die rolls.
pub trait Dice {
    /// Rolls one six-sided die (1 to 6).
    fn d6(&mut self) -> u8;

    /// Rolls `count` six-sided dice and returns their sum.
    fn roll(&mut self, count: u8) -> u8 {
        (0..count).map(|_| self.d6()).sum()
    }
}

/// Real dice, backed by a seeded random generator.
pub struct RandomDice {
    rng: StdRng,
}

impl RandomDice {
    /// Dice seeded from the operating system.
    pub fn new() -> RandomDice {
        RandomDice {
            rng: rand::make_rng(),
        }
    }

    /// Reproducible dice, for simulations.
    pub fn seeded(seed: u64) -> RandomDice {
        RandomDice {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Default for RandomDice {
    fn default() -> RandomDice {
        RandomDice::new()
    }
}

impl Dice for RandomDice {
    fn d6(&mut self) -> u8 {
        self.rng.random_range(1..=6)
    }
}

/// Dice returning a fixed list of results, then counting 1 to 6 over and over.
///
/// The count goes on after the script so that loops rolling until some result (Table 1) always
/// end.
#[derive(Debug, Clone)]
pub struct ScriptedDice {
    rolls: VecDeque<u8>,
    next: u8,
}

impl ScriptedDice {
    /// Dice that return `rolls` in order, then `after`, `after + 1`… wrapping from 6 to 1.
    pub fn new(rolls: &[u8], after: u8) -> ScriptedDice {
        ScriptedDice {
            rolls: rolls.iter().copied().collect(),
            next: after.clamp(1, 6),
        }
    }

    /// How many scripted rolls are left.
    pub fn remaining(&self) -> usize {
        self.rolls.len()
    }
}

impl Dice for ScriptedDice {
    fn d6(&mut self) -> u8 {
        self.rolls.pop_front().unwrap_or_else(|| {
            let roll = self.next;
            self.next = roll % 6 + 1;
            roll
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripted_dice_follow_the_script() {
        let mut dice = ScriptedDice::new(&[1, 6, 3], 5);
        assert_eq!(dice.roll(2), 7);
        assert_eq!(dice.d6(), 3);
        assert_eq!(dice.remaining(), 0);
        assert_eq!(dice.roll(3), 5 + 6 + 1);
    }

    #[test]
    fn random_dice_stay_in_range() {
        let mut dice = RandomDice::seeded(7);
        for _ in 0..200 {
            assert!((1..=6).contains(&dice.d6()));
        }
    }
}
