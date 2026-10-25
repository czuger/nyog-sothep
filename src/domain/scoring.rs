//! The investigators' scoring table, shown at the end of the game.

use crate::domain::investigator::Investigator;
use crate::domain::investigator_status::InvestigatorStatus;

/// One investigator's score, line by line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    /// +2 per hit point taken from the professor.
    pub damage: i32,
    /// +3 per group of Fanatics killed.
    pub fanatics: i32,
    /// +5 for taking part in a successful banishment.
    pub banishment: i32,
    /// +1 per SAN point left.
    pub sanity: i32,
    /// −5 for having become a Fanatic.
    pub madness: i32,
    /// −2 per city destroyed by Nyog'Sothep.
    pub destruction: i32,
}

impl Score {
    /// Scores an investigator at the end of the game.
    pub fn of(investigator: &Investigator, destroyed_cities: usize) -> Score {
        let active = investigator.status == InvestigatorStatus::Active;
        Score {
            damage: 2 * investigator.damage_dealt as i32,
            fanatics: 3 * investigator.fanatics_killed as i32,
            banishment: if investigator.banished { 5 } else { 0 },
            sanity: if active { investigator.san.max(0) } else { 0 },
            madness: if investigator.status == InvestigatorStatus::Mad {
                -5
            } else {
                0
            },
            destruction: -2 * destroyed_cities as i32,
        }
    }

    /// The sum of every line.
    pub fn total(&self) -> i32 {
        self.damage
            + self.fanatics
            + self.banishment
            + self.sanity
            + self.madness
            + self.destruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::investigator_id::InvestigatorId;

    #[test]
    fn scores_add_up() {
        let mut investigator = Investigator::new(InvestigatorId::Walker, 9);
        investigator.damage_dealt = 3;
        investigator.fanatics_killed = 1;
        investigator.banished = true;
        let score = Score::of(&investigator, 2);
        assert_eq!(score.total(), 6 + 3 + 5 + 9 - 4);
        investigator.status = InvestigatorStatus::Mad;
        assert_eq!(Score::of(&investigator, 0).total(), 6 + 3 + 5 - 5);
    }
}
