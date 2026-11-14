use crate::domain::dice::Dice;
use crate::domain::investigator::Investigator;
use crate::domain::item::{Gear, Item};
use crate::domain::turn::engine::Engine;
use crate::domain::turn::prompt::MoveOrder;

/// The most various objects an investigator carries.
const GEAR_LIMIT: usize = 2;
/// SAN regained instead of an item already held that nobody present can take.
const DUPLICATE_SAN: i32 = 1;
/// Laudanum is drunk when this much SAN is missing (no point is wasted under the cap)…
const LAUDANUM_MISSING_SAN: i32 = 6;
/// …or when the SAN falls this low.
const LAUDANUM_LOW_SAN: i32 = 3;

impl Engine<'_> {
    /// An investigator finds an item: an item already held goes to an investigator present (or
    /// gives 1 SAN back), the better of two weapons or protections is kept and the other given,
    /// and a third various object is given or lost.
    pub(super) fn fan_receive(&mut self, investigator: usize, item: Item) {
        let actor = &self.state.investigators[investigator];
        if actor.holds(item) {
            if !self.hand_over(investigator, item) {
                let gain = self.gain_san(investigator, DUPLICATE_SAN);
                let name = self.state.investigators[investigator].id.name();
                self.log(format!(
                    "{name} a déjà {} : {gain} SAN regagné(s) à la place.",
                    item.with_article()
                ));
            }
            return;
        }
        let (kept, spare) = sort_out(actor, item);
        if let Some(kept) = kept {
            self.state.investigators[investigator].take(kept);
        }
        if let Some(spare) = spare
            && !self.hand_over(investigator, spare)
        {
            let name = self.state.investigators[investigator].id.name();
            self.log(format!(
                "{name} laisse {}, que personne ne peut prendre.",
                spare.with_article()
            ));
        }
    }

    /// Gives an item to another investigator in the same city who can take it.
    ///
    /// # Returns
    /// `false` when nobody present can.
    fn hand_over(&mut self, giver: usize, item: Item) -> bool {
        let here = self.state.investigators[giver].location;
        let Some(taker) = self
            .state
            .investigators_at(here)
            .into_iter()
            .find(|index| *index != giver && can_take(&self.state.investigators[*index], item))
        else {
            return false;
        };
        self.state.investigators[taker].take(item);
        let (from, to) = (
            self.state.investigators[giver].id,
            self.state.investigators[taker].id,
        );
        self.log(format!(
            "{} transmet {} {}.",
            from.name(),
            item.with_article(),
            to.to_name()
        ));
        true
    }

    /// Drinks the laudanum before moving when the SAN is low enough.
    pub(super) fn drink_laudanum(&mut self, investigator: usize, order: MoveOrder) {
        let actor = &self.state.investigators[investigator];
        let thirsty =
            actor.starting_san - actor.san >= LAUDANUM_MISSING_SAN || actor.san <= LAUDANUM_LOW_SAN;
        if order == MoveOrder::Psychoanalyst || !thirsty || !actor.holds(Item::Gear(Gear::Laudanum))
        {
            return;
        }
        self.state.investigators[investigator].spend(Gear::Laudanum);
        let roll = i32::from(self.dice.d6());
        let gain = self.gain_san(investigator, roll);
        let name = self.state.investigators[investigator].id.name();
        self.log(format!(
            "{name} boit sa fiole de laudanum et regagne {gain} SAN."
        ));
    }
}

/// What an investigator takes when finding an item, and what is left to give.
fn sort_out(actor: &Investigator, found: Item) -> (Option<Item>, Option<Item>) {
    match found {
        Item::Weapon(weapon) if actor.weapon => {
            let held = actor.weapon_kind;
            let (kept, spare) = match weapon.rank() > held.rank() {
                true => (weapon, held),
                false => (held, weapon),
            };
            (Some(Item::Weapon(kept)), Some(Item::Weapon(spare)))
        }
        Item::Protection(protection) if actor.sign => {
            let held = actor.protection;
            let (kept, spare) = match protection.rank() > held.rank() {
                true => (protection, held),
                false => (held, protection),
            };
            (Some(Item::Protection(kept)), Some(Item::Protection(spare)))
        }
        Item::Gear(_) if actor.gear.len() >= GEAR_LIMIT => (None, Some(found)),
        _ => (Some(found), None),
    }
}

/// Whether an investigator has room for an item they do not hold yet.
fn can_take(investigator: &Investigator, item: Item) -> bool {
    !investigator.holds(item)
        && match item {
            Item::Weapon(_) => !investigator.weapon,
            Item::Protection(_) => !investigator.sign,
            Item::Gear(_) => investigator.gear.len() < GEAR_LIMIT,
            Item::Medallion | Item::Spell => true,
        }
}
