//! An investigator fighting Professor Petersen.

use crate::domain::dice::Dice;
use crate::domain::fight_choice::FightChoice;

/// What the professor does after the fight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfessorAfterFight {
    /// He stays where he is.
    Stays,
    /// He retreats one location (shot at).
    Retreats,
    /// He is pushed back one location (protective sign).
    PushedBack,
}

/// The result of a fight against the professor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FightOutcome {
    /// SAN lost by the investigator.
    pub san_loss: i32,
    /// Hit points lost by the professor.
    pub damage: i32,
    /// The investigator loses their protective sign.
    pub loses_sign: bool,
    /// The professor moves the investigator two cities.
    pub carries_investigator: bool,
    /// What the professor does next.
    pub professor: ProfessorAfterFight,
}

/// The professor's hit points at the start.
pub const PROFESSOR_HP: i32 = 14;

/// Resolves a fight with the investigator's choice.
pub fn resolve(choice: FightChoice, dice: &mut dyn Dice) -> FightOutcome {
    let calm = FightOutcome {
        san_loss: 0,
        damage: 0,
        loses_sign: false,
        carries_investigator: false,
        professor: ProfessorAfterFight::Stays,
    };
    match choice {
        FightChoice::Weapon => FightOutcome {
            damage: match dice.d6() {
                1 => 0,
                6 => 3,
                _ => 2,
            },
            professor: ProfessorAfterFight::Retreats,
            ..calm
        },
        FightChoice::Sign => FightOutcome {
            san_loss: 2,
            loses_sign: true,
            professor: ProfessorAfterFight::PushedBack,
            ..calm
        },
        FightChoice::Hide => calm,
        FightChoice::Unarmed => FightOutcome {
            san_loss: 4,
            carries_investigator: true,
            ..calm
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dice::ScriptedDice;
    use crate::domain::investigator::Investigator;
    use crate::domain::investigator_id::InvestigatorId;

    #[test]
    fn weapon_damage_follows_the_die() {
        let mut dice = ScriptedDice::new(&[1, 2, 5, 6], 1);
        let damages: Vec<i32> = (0..4)
            .map(|_| resolve(FightChoice::Weapon, &mut dice).damage)
            .collect();
        assert_eq!(damages, vec![0, 2, 2, 3]);
    }

    #[test]
    fn options_depend_on_the_items() {
        let mut investigator = Investigator::new(InvestigatorId::Hastings, 10);
        assert_eq!(
            FightChoice::options(&investigator),
            vec![FightChoice::Unarmed]
        );
        investigator.sign = true;
        investigator.medallion = true;
        assert_eq!(
            FightChoice::options(&investigator),
            vec![FightChoice::Sign, FightChoice::Hide]
        );
        let mut dice = ScriptedDice::new(&[], 1);
        let outcome = resolve(FightChoice::Sign, &mut dice);
        assert!(outcome.loses_sign && outcome.san_loss == 2);
        assert_eq!(outcome.professor, ProfessorAfterFight::PushedBack);
        assert!(resolve(FightChoice::Unarmed, &mut dice).carries_investigator);
    }
}
