use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::encounter::EncounterOutcome;
use crate::domain::investigator::Investigator;
use crate::domain::item::{Gear, Item};
use crate::domain::location_id::LocationId;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::prompt::MoveOrder;
use crate::domain::turn::rule_sets::fan_made::{encounters, end_of_game, progressive_banishment};
use crate::domain::turn::rule_sets::rules::Rules;

/// The spell is cast alone.
const BANISHERS_NEEDED: usize = 1;

/// The fan-made rules.
pub(crate) struct FanMadeRules;

impl Rules for FanMadeRules {
    fn ask_event(&self, engine: &mut Engine<'_>, investigator: usize) {
        engine.fan_event(investigator);
    }

    fn receive(&self, engine: &mut Engine<'_>, investigator: usize, item: Item) {
        engine.fan_receive(investigator, item);
    }

    /// The starting SAN.
    fn san_cap(&self, investigator: &Investigator) -> i32 {
        investigator.starting_san
    }

    /// Closed once Nyog'Sothep is out.
    fn psychoanalyst_open(&self, state: &GameState) -> bool {
        !state.nyog.is_summoned()
    }

    fn free_crossing(&self, investigator: &Investigator) -> bool {
        investigator.holds(Item::Gear(Gear::Pass))
    }

    fn before_move(&self, engine: &mut Engine<'_>, investigator: usize, order: MoveOrder) {
        engine.drink_laudanum(investigator, order);
    }

    fn encounter(
        &self,
        kind: CounterKind,
        investigator: &Investigator,
        dice: &mut dyn Dice,
    ) -> EncounterOutcome {
        encounters::resolve(kind, investigator, dice)
    }

    fn professor_draw(&self, engine: &mut Engine<'_>, remaining: u8) {
        engine.location_draw(remaining);
    }

    /// He gives himself away: his next move is made in the open.
    fn professor_wounded(&self, engine: &mut Engine<'_>) {
        engine.state.move_in_the_open = true;
        engine
            .log("Blessé, le professeur se trahit : il fera son prochain déplacement à découvert.");
        engine.clue(ClueKind::MoveInTheOpen);
    }

    /// The Foul One moves every turn.
    fn nyog_rests(&self, _engine: &mut Engine<'_>) -> bool {
        false
    }

    /// It hunts freely: no city draws it.
    fn update_banishment_target(&self, engine: &mut Engine<'_>) {
        engine.state.nyog.target = None;
    }

    fn cast_banishment(&self, engine: &mut Engine<'_>, city: LocationId) {
        engine.progressive_banishment(city);
    }

    fn outcome(&self, state: &GameState) -> Option<Outcome> {
        end_of_game::outcome(state)
    }

    fn banishers_needed(&self) -> usize {
        BANISHERS_NEEDED
    }

    fn banishment_chance(&self, state: &GameState, city: LocationId) -> f64 {
        progressive_banishment::banishment_chance(state, city)
    }
}
