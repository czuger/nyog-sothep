use crate::domain::dice::Dice;
use crate::domain::investigator::Investigator;
use crate::domain::item::{Gear, Item};
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::meeting_san_loss;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::prompt::Prompt;
use crate::domain::turn::rule_sets::rules::spell_holders_at;

/// The total that banishes Nyog'Sothep.
const BANISHING_TOTAL: u32 = 6;
/// The total that only sends it elsewhere.
const DISPLACING_TOTAL: u32 = 5;
/// The natural roll that turns the spell against its caster.
const BACKFIRE_ROLL: u8 = 1;

/// What one cast of the spell does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cast {
    /// A natural 1: Nyog'Sothep kills the caster.
    Backfires,
    /// Nothing happens.
    Fails,
    /// The investigators place Nyog'Sothep in the city of their choice.
    Displaces,
    /// Nyog'Sothep is cast back into the abyss.
    Banishes,
}

impl Cast {
    /// Reads a cast: the natural roll, and the total with its bonuses.
    pub(crate) fn of(roll: u8, total: u32) -> Cast {
        match (roll, total) {
            (BACKFIRE_ROLL, _) => Cast::Backfires,
            (_, BANISHING_TOTAL..) => Cast::Banishes,
            (_, DISPLACING_TOTAL) => Cast::Displaces,
            _ => Cast::Fails,
        }
    }
}

/// How likely the spell holders of a city banish Nyog'Sothep entering it now: those who survive
/// the meeting cast in turn, each failure helping the next, until a partial banishment.
pub(crate) fn banishment_chance(state: &GameState, city: LocationId) -> f64 {
    if !city.is_city() {
        return 0.0;
    }
    let present = state.investigators_at(city);
    let casters = spell_holders_at(state, city)
        .into_iter()
        .filter(|index| survives_meeting(&state.investigators[*index]))
        .count();
    let book = present
        .iter()
        .any(|index| state.investigators[*index].holds(Item::Gear(Gear::Necronomicon)));
    let (mut chance, mut going_on) = (0.0, 1.0);
    for failed in (state.nyog.failed_banishments..).take(casters) {
        let casts: Vec<Cast> = (1..=6u8)
            .map(|roll| Cast::of(roll, u32::from(roll) + failed + u32::from(book)))
            .collect();
        let share = |cast: Cast| casts.iter().filter(|each| **each == cast).count() as f64 / 6.0;
        chance += going_on * share(Cast::Banishes);
        going_on *= share(Cast::Backfires) + share(Cast::Fails);
    }
    chance
}

/// Whether an investigator is still in the game, and sane, after meeting Nyog'Sothep.
fn survives_meeting(investigator: &Investigator) -> bool {
    let loss = meeting_san_loss(investigator.seen_nyog) - investigator.protection.nyog_relief();
    investigator.is_active() && investigator.sign && investigator.san > loss
}

impl Engine<'_> {
    /// Every spell holder in the city casts the spell in turn, until it works, partly works, or
    /// no caster is left.
    pub(super) fn progressive_banishment(&mut self, city: LocationId) {
        for caster in spell_holders_at(self.state, city) {
            if self.state.is_finished() || !self.state.investigators[caster].is_active() {
                return;
            }
            if !self.cast_alone(caster, city) {
                return;
            }
        }
    }

    /// One cast: 1d6, +1 per failed attempt of the game, +1 with the Necronomicon present.
    ///
    /// # Returns
    /// Whether the next caster may try.
    fn cast_alone(&mut self, caster: usize, city: LocationId) -> bool {
        let roll = self.dice.d6();
        let failed = self.state.nyog.failed_banishments;
        let necronomicon = self
            .state
            .investigators_at(city)
            .into_iter()
            .any(|index| self.state.investigators[index].holds(Item::Gear(Gear::Necronomicon)));
        let total = u32::from(roll) + failed + u32::from(necronomicon);
        let id = self.state.investigators[caster].id;
        let book = match necronomicon {
            true => ", +1 pour le Necronomicon",
            false => "",
        };
        self.log(format!(
            "{} lance le sort de renvoi : {roll} au dé, +{failed} pour les tentatives ratées{book}, soit {total}.",
            id.name()
        ));
        let cast = Cast::of(roll, total);
        if cast != Cast::Banishes {
            self.state.nyog.failed_banishments += 1;
        }
        match cast {
            Cast::Backfires => {
                self.log(format!(
                    "Le sort se retourne contre {}, que Nyog'Sothep tue sur-le-champ.",
                    id.inner_name()
                ));
                self.put_to_death(caster);
                true
            }
            Cast::Fails => {
                self.log("Le sort échoue : Nyog'Sothep reste libre.");
                true
            }
            Cast::Displaces => {
                self.log("Le sort agit en partie : les investigateurs choisissent la ville où envoyer Nyog'Sothep.");
                self.ask(Prompt::BanishmentCity, Phase::Banishment);
                false
            }
            Cast::Banishes => {
                self.state.investigators[caster].banished = true;
                self.finish(Outcome::Banished);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_spell_table() {
        assert_eq!(Cast::of(1, 9), Cast::Backfires);
        assert_eq!(Cast::of(2, 4), Cast::Fails);
        assert_eq!(Cast::of(3, 5), Cast::Displaces);
        assert_eq!(Cast::of(6, 6), Cast::Banishes);
        assert_eq!(Cast::of(4, 7), Cast::Banishes);
    }

    #[test]
    fn three_casters_gathered_for_the_first_time_win_56_percent() {
        use crate::domain::professor_ai::test_state;
        let mut state = test_state();
        state.nyog.location = Some(LocationId::Boston);
        for investigator in state.investigators.iter_mut().take(3) {
            investigator.location = LocationId::Concord;
            investigator.spell = true;
            investigator.sign = true;
            investigator.san = 10;
        }
        let chance = banishment_chance(&state, LocationId::Concord);
        assert!((chance - 5.0 / 9.0).abs() < 1e-9, "{chance}");
        state.investigators[0].san = 4;
        state.investigators[1].sign = false;
        let alone = banishment_chance(&state, LocationId::Concord);
        assert!((alone - 1.0 / 6.0).abs() < 1e-9, "{alone}");
    }
}
