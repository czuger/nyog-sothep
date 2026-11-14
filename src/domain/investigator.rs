use serde::{Deserialize, Serialize};

use crate::domain::investigator_id::InvestigatorId;
use crate::domain::investigator_status::InvestigatorStatus;
use crate::domain::item::{Gear, Item, Protection, Weapon};
use crate::domain::location_id::LocationId;

/// SAN dice rolled for each investigator at the start, plus a fixed bonus (`3d6 + 3`).
pub const STARTING_SAN_DICE: u8 = 3;
/// Fixed bonus added to the starting SAN roll.
pub const STARTING_SAN_BONUS: i32 = 3;
/// Where the investigators start.
pub const STARTING_CITY: LocationId = LocationId::Nantucket;

/// One of the four investigators played by the human.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Investigator {
    /// Which counter.
    pub id: InvestigatorId,
    /// Sanity points; at 0 the investigator goes mad.
    pub san: i32,
    /// SAN rolled at the start.
    #[serde(default)]
    pub starting_san: i32,
    /// Holds a weapon.
    pub weapon: bool,
    /// Which weapon, when holding one.
    #[serde(default)]
    pub weapon_kind: Weapon,
    /// Holds a protection (a protective sign in the original rules).
    pub sign: bool,
    /// Which protection, when holding one.
    #[serde(default)]
    pub protection: Protection,
    /// Holds a Fanatic medallion.
    pub medallion: bool,
    /// Various objects (fan-made version).
    #[serde(default)]
    pub gear: Vec<Gear>,
    /// Knows the banishment spell.
    pub spell: bool,
    /// Current location.
    pub location: LocationId,
    /// The location left by the last move, where push-backs send the investigator.
    pub previous: LocationId,
    /// In the game, gone mad or dead.
    pub status: InvestigatorStatus,
    /// Coming turns the investigator loses.
    pub skip_turns: u8,
    /// This turn's action phase is skipped (lost turn or psychoanalyst).
    pub skips_action: bool,
    /// Next turn: stay in place (losing the turn) unless taking to the sea.
    pub car_broken: bool,
    /// Next turn: this city may not be entered (chased by the Inhabitants).
    pub forbidden_city: Option<LocationId>,
    /// Next turn: borders are crossed without a roll (border result 5).
    pub free_border: bool,
    /// Next turn: the border towards this city may not be crossed (border result 6).
    pub blocked_crossing: Option<LocationId>,
    /// Has already met Nyog'Sothep.
    pub seen_nyog: bool,
    /// Groups of Fanatics killed, for the scoring.
    pub fanatics_killed: u32,
    /// Hit points taken from the professor, for the scoring.
    pub damage_dealt: u32,
    /// Took part in a successful banishment, for the scoring.
    pub banished: bool,
}

impl Investigator {
    /// A new investigator in Nantucket with the given SAN.
    pub fn new(id: InvestigatorId, san: i32) -> Investigator {
        Investigator {
            id,
            san,
            starting_san: san,
            weapon: false,
            weapon_kind: Weapon::Revolver,
            sign: false,
            protection: Protection::Sign,
            medallion: false,
            gear: Vec::new(),
            spell: false,
            location: STARTING_CITY,
            previous: STARTING_CITY,
            status: InvestigatorStatus::Active,
            skip_turns: 0,
            skips_action: false,
            car_broken: false,
            forbidden_city: None,
            free_border: false,
            blocked_crossing: None,
            seen_nyog: false,
            fanatics_killed: 0,
            damage_dealt: 0,
            banished: false,
        }
    }

    /// Whether the investigator is still in the game.
    pub fn is_active(&self) -> bool {
        self.status == InvestigatorStatus::Active
    }

    /// How many dice the investigator rolls on Table 2 or 3.
    pub fn event_dice(&self) -> u8 {
        match (self.sign, self.medallion) {
            (true, true) => 3,
            (true, false) => 2,
            _ => 1,
        }
    }

    /// Takes an item (an object already held of the same sort is replaced).
    pub fn take(&mut self, item: Item) {
        match item {
            Item::Weapon(weapon) => {
                self.weapon = true;
                self.weapon_kind = weapon;
            }
            Item::Protection(protection) => {
                self.sign = true;
                self.protection = protection;
            }
            Item::Medallion => self.medallion = true,
            Item::Spell => self.spell = true,
            Item::Gear(gear) => self.gear.push(gear),
        }
    }

    /// Whether the investigator holds this very item.
    pub fn holds(&self, item: Item) -> bool {
        match item {
            Item::Weapon(weapon) => self.weapon && self.weapon_kind == weapon,
            Item::Protection(protection) => self.sign && self.protection == protection,
            Item::Medallion => self.medallion,
            Item::Spell => self.spell,
            Item::Gear(gear) => self.gear.contains(&gear),
        }
    }

    /// The objects carried, in display order (the spell is not an object).
    pub fn items(&self) -> Vec<Item> {
        let mut items = Vec::new();
        if self.weapon {
            items.push(Item::Weapon(self.weapon_kind));
        }
        if self.sign {
            items.push(Item::Protection(self.protection));
        }
        if self.medallion {
            items.push(Item::Medallion);
        }
        items.extend(self.gear.iter().map(|gear| Item::Gear(*gear)));
        items
    }

    /// The protection is used up, unless it is never lost.
    pub fn lose_protection(&mut self) {
        self.sign &= !self.protection.is_lost_on_use();
    }

    /// Uses up one object.
    ///
    /// # Returns
    /// `false` when the investigator did not hold it.
    pub fn spend(&mut self, gear: Gear) -> bool {
        match self.gear.iter().position(|held| *held == gear) {
            Some(index) => {
                self.gear.remove(index);
                true
            }
            None => false,
        }
    }

    /// Moves the investigator, remembering where they came from.
    pub fn go_to(&mut self, destination: LocationId) {
        if destination != self.location {
            self.previous = self.location;
            self.location = destination;
        }
    }

    /// Sends the investigator back to the location they left.
    pub fn go_back(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.location);
    }
}
