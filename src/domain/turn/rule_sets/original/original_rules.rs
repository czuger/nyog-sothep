use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::encounter::{self, EncounterOutcome};
use crate::domain::investigator::Investigator;
use crate::domain::item::Item;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{BANISHERS_NEEDED, SUMMONING_FANATICS_CITIES};
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::prompt::MoveOrder;
use crate::domain::turn::rule_sets::rules::{Rules, spell_holders_at};

/// The rules published in *Jeux & Stratégie*: Tables 1, 2 and 3, the spell cast by three.
pub(crate) struct OriginalRules;

impl Rules for OriginalRules {
    fn ask_event(&self, engine: &mut Engine<'_>, investigator: usize) {
        engine.ask_table(investigator);
    }

    fn receive(&self, engine: &mut Engine<'_>, investigator: usize, item: Item) {
        engine.state.investigators[investigator].take(item);
    }

    fn san_cap(&self, _investigator: &Investigator) -> i32 {
        i32::MAX
    }

    fn psychoanalyst_open(&self, _state: &GameState) -> bool {
        true
    }

    fn free_crossing(&self, _investigator: &Investigator) -> bool {
        false
    }

    fn before_move(&self, _engine: &mut Engine<'_>, _investigator: usize, _order: MoveOrder) {}

    fn encounter(
        &self,
        kind: CounterKind,
        investigator: &Investigator,
        dice: &mut dyn Dice,
    ) -> EncounterOutcome {
        encounter::resolve(kind, investigator, dice)
    }

    fn professor_draw(&self, engine: &mut Engine<'_>, remaining: u8) {
        engine.table1_draw(remaining);
    }

    fn professor_wounded(&self, _engine: &mut Engine<'_>) {}

    /// They stay still on 1 or 2.
    fn nyog_rests(&self, engine: &mut Engine<'_>) -> bool {
        engine.dice.d6() <= 2
    }

    fn update_banishment_target(&self, engine: &mut Engine<'_>) {
        engine.update_group_target();
    }

    fn cast_banishment(&self, engine: &mut Engine<'_>, city: LocationId) {
        engine.group_banishment(city);
    }

    /// Everyone lost; too few investigators left to banish Nyog'Sothep; or too few Fanatics
    /// left to summon it (house rule).
    fn outcome(&self, state: &GameState) -> Option<Outcome> {
        let active = state.active_investigators().len();
        let summoned = state.nyog.is_summoned();
        if active == 0 {
            Some(Outcome::InvestigatorsLost)
        } else if summoned && active < BANISHERS_NEEDED {
            Some(Outcome::NyogSothepReigns)
        } else if !summoned && state.counters.fanatics_in_play() < SUMMONING_FANATICS_CITIES {
            Some(Outcome::FanaticsDestroyed)
        } else {
            None
        }
    }

    fn banishers_needed(&self) -> usize {
        BANISHERS_NEEDED
    }

    /// Counted as certain once three spell holders stand there.
    fn banishment_chance(&self, state: &GameState, city: LocationId) -> f64 {
        match spell_holders_at(state, city).len() >= BANISHERS_NEEDED {
            true => 1.0,
            false => 0.0,
        }
    }
}
