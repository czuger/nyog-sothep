use serde::{Deserialize, Serialize};

use crate::domain::investigator::Investigator;
use crate::domain::item::{Gear, Item};

/// How an investigator faces Professor Petersen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FightChoice {
    /// Shoot him with the weapon.
    Weapon,
    /// Throw dynamite at him (fan-made version).
    Dynamite,
    /// Hold up the protective sign.
    Sign,
    /// Wear the medallion and pass for a Fanatic.
    Hide,
    /// Nothing to fight with.
    Unarmed,
}

impl FightChoice {
    /// The choices open to an investigator; `[Unarmed]` when they hold nothing useful.
    pub fn options(investigator: &Investigator) -> Vec<FightChoice> {
        let mut options = Vec::new();
        if investigator.weapon {
            options.push(FightChoice::Weapon);
        }
        if investigator.holds(Item::Gear(Gear::Dynamite)) {
            options.push(FightChoice::Dynamite);
        }
        if investigator.sign {
            options.push(FightChoice::Sign);
        }
        if investigator.medallion {
            options.push(FightChoice::Hide);
        }
        if options.is_empty() {
            options.push(FightChoice::Unarmed);
        }
        options
    }

    /// The stable code used in forms.
    pub fn code(self) -> &'static str {
        match self {
            FightChoice::Weapon => "weapon",
            FightChoice::Dynamite => "dynamite",
            FightChoice::Sign => "sign",
            FightChoice::Hide => "hide",
            FightChoice::Unarmed => "unarmed",
        }
    }

    /// Reads a code written by [`FightChoice::code`].
    pub fn from_code(code: &str) -> Option<FightChoice> {
        [
            FightChoice::Weapon,
            FightChoice::Dynamite,
            FightChoice::Sign,
            FightChoice::Hide,
            FightChoice::Unarmed,
        ]
        .into_iter()
        .find(|choice| choice.code() == code)
    }

    /// French label of the choice.
    pub fn label(self) -> &'static str {
        match self {
            FightChoice::Weapon => "Tirer avec son arme",
            FightChoice::Dynamite => "Lancer sa dynamite (3 points de vie)",
            FightChoice::Sign => "Brandir sa protection",
            FightChoice::Hide => "Se faire passer pour un Fanatique grâce au médaillon",
            FightChoice::Unarmed => "Subir l'attaque, sans rien pour se défendre",
        }
    }
}
