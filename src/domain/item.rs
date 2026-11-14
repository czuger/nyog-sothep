//! What an investigator may find and carry. The original rules only hand out the revolver, the
//! protective sign, the medallion and the spell; the fan-made version adds the others.

use serde::{Deserialize, Serialize};

/// A weapon: an investigator carries at most one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Weapon {
    /// The weapon of the original rules (stored as `weapon`, the name of its image).
    #[default]
    #[serde(rename = "weapon")]
    Revolver,
    /// A revolver with +1 to the die against the Fanatics and the professor.
    Shotgun,
}

impl Weapon {
    /// Every weapon.
    pub const ALL: [Weapon; 2] = [Weapon::Revolver, Weapon::Shotgun];

    /// The better weapon ranks higher.
    pub fn rank(self) -> u8 {
        self.bonus()
    }

    /// Added to the die when shooting Fanatics or the professor.
    pub fn bonus(self) -> u8 {
        match self {
            Weapon::Revolver => 0,
            Weapon::Shotgun => 1,
        }
    }

    /// The stable code, also the name of its image.
    pub fn code(self) -> &'static str {
        match self {
            Weapon::Revolver => "weapon",
            Weapon::Shotgun => "shotgun",
        }
    }

    /// French name.
    pub fn name(self) -> &'static str {
        match self {
            Weapon::Revolver => "revolver",
            Weapon::Shotgun => "fusil de chasse",
        }
    }
}

/// A protection: an investigator carries at most one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protection {
    /// The protective sign of the original rules.
    #[default]
    Sign,
    /// The star-stone of Mnar: a sign that is never lost and eases the meeting with Nyog'Sothep.
    MnarStone,
}

impl Protection {
    /// Every protection.
    pub const ALL: [Protection; 2] = [Protection::Sign, Protection::MnarStone];

    /// The better protection ranks higher.
    pub fn rank(self) -> u8 {
        u8::from(!self.is_lost_on_use())
    }

    /// Whether using it against the Deep Ones or the professor costs it.
    pub fn is_lost_on_use(self) -> bool {
        self == Protection::Sign
    }

    /// SAN its holder loses less when meeting Nyog'Sothep.
    pub fn nyog_relief(self) -> i32 {
        match self {
            Protection::Sign => 0,
            Protection::MnarStone => 1,
        }
    }

    /// The stable code, also the name of its image.
    pub fn code(self) -> &'static str {
        match self {
            Protection::Sign => "sign",
            Protection::MnarStone => "mnar_stone",
        }
    }

    /// French name.
    pub fn name(self) -> &'static str {
        match self {
            Protection::Sign => "signe protecteur",
            Protection::MnarStone => "pierre étoilée de Mnar",
        }
    }
}

/// The various objects of the fan-made version: an investigator carries at most two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gear {
    /// Single use: kills Fanatics or Ghouls met, or takes 3 hit points from the professor.
    Dynamite,
    /// Borders are crossed without a roll.
    Pass,
    /// Single use: 1d6 SAN back while moving.
    Laudanum,
    /// Single use: cancels the professor's Dreams.
    Amulet,
    /// Single use: instead of an event, reveals the hidden counters of the neighbouring cities.
    IbnGhaziPowder,
    /// +1 to the banishment spell cast in its holder's presence.
    Necronomicon,
}

impl Gear {
    /// Every various object.
    pub const ALL: [Gear; 6] = [
        Gear::Dynamite,
        Gear::Pass,
        Gear::Laudanum,
        Gear::Amulet,
        Gear::IbnGhaziPowder,
        Gear::Necronomicon,
    ];

    /// The stable code, also the name of its image.
    pub fn code(self) -> &'static str {
        match self {
            Gear::Dynamite => "dynamite",
            Gear::Pass => "pass",
            Gear::Laudanum => "laudanum",
            Gear::Amulet => "amulet",
            Gear::IbnGhaziPowder => "ibn_ghazi_powder",
            Gear::Necronomicon => "necronomicon",
        }
    }

    /// French name.
    pub fn name(self) -> &'static str {
        match self {
            Gear::Dynamite => "dynamite",
            Gear::Pass => "laissez-passer",
            Gear::Laudanum => "fiole de laudanum",
            Gear::Amulet => "amulette de rêveur",
            Gear::IbnGhaziPowder => "poudre d'Ibn-Ghazi",
            Gear::Necronomicon => "Necronomicon",
        }
    }
}

/// Anything an event may give an investigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Item {
    /// A weapon.
    Weapon(Weapon),
    /// A protection.
    Protection(Protection),
    /// A Fanatic medallion.
    Medallion,
    /// The banishment spell.
    Spell,
    /// A various object.
    Gear(Gear),
}

/// The revolver of the original rules.
pub const REVOLVER: Item = Item::Weapon(Weapon::Revolver);
/// The protective sign of the original rules.
pub const SIGN: Item = Item::Protection(Protection::Sign);

impl Item {
    /// Every object (the spell aside).
    pub fn objects() -> Vec<Item> {
        let mut objects: Vec<Item> = Weapon::ALL.into_iter().map(Item::Weapon).collect();
        objects.extend(Protection::ALL.into_iter().map(Item::Protection));
        objects.push(Item::Medallion);
        objects.extend(Gear::ALL.into_iter().map(Item::Gear));
        objects
    }

    /// The stable code, also the name of its image (the spell has none).
    pub fn code(self) -> &'static str {
        match self {
            Item::Weapon(weapon) => weapon.code(),
            Item::Protection(protection) => protection.code(),
            Item::Medallion => "medallion",
            Item::Spell => "spell",
            Item::Gear(gear) => gear.code(),
        }
    }

    /// French name with its definite article, for log sentences: « le revolver ».
    pub fn with_article(self) -> String {
        let name = self.name();
        let article = match self {
            Item::Protection(Protection::MnarStone)
            | Item::Gear(Gear::Dynamite | Gear::Laudanum | Gear::IbnGhaziPowder) => "la ",
            Item::Gear(Gear::Amulet) => "l'",
            _ => "le ",
        };
        format!("{article}{name}")
    }

    /// French name.
    pub fn name(self) -> &'static str {
        match self {
            Item::Weapon(weapon) => weapon.name(),
            Item::Protection(protection) => protection.name(),
            Item::Medallion => "médaillon de Fanatique",
            Item::Spell => "sort de renvoi",
            Item::Gear(gear) => gear.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_what_is_stored() {
        for weapon in Weapon::ALL {
            assert_eq!(
                serde_json::to_string(&weapon).unwrap(),
                format!("\"{}\"", weapon.code())
            );
        }
        for protection in Protection::ALL {
            let stored = serde_json::to_string(&protection).unwrap();
            assert_eq!(stored, format!("\"{}\"", protection.code()));
        }
        for gear in Gear::ALL {
            assert_eq!(
                serde_json::to_string(&gear).unwrap(),
                format!("\"{}\"", gear.code())
            );
        }
        assert_eq!(Item::objects().len(), 11);
        assert_eq!(
            Item::Gear(Gear::Amulet).with_article(),
            "l'amulette de rêveur"
        );
    }
}
