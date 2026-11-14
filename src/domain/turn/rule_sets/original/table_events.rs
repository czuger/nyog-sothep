use crate::domain::dice::Dice;
use crate::domain::event_table::EventTable;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::prompt::Prompt;

impl Engine<'_> {
    /// Asks on which table, 2 or 3, the investigator draws the event.
    pub(super) fn ask_table(&mut self, investigator: usize) {
        self.ask(
            Prompt::Table { investigator },
            Phase::Action {
                investigator,
                step: ActionStep::Event,
            },
        );
    }

    /// Rolls on the chosen table (one die, more with a sign and a medallion) and applies the
    /// event.
    pub(in crate::domain::turn) fn draw_event(&mut self, investigator: usize, table: EventTable) {
        let actor = &self.state.investigators[investigator];
        let dice_count = actor.event_dice();
        let name = actor.id.name();
        let roll = self.dice.roll(dice_count);
        self.log(format!(
            "{name} tire un événement sur la table {} ({dice_count}d6 : {roll}) : « {} »",
            table.number(),
            table.text(roll)
        ));
        self.resolve_event(investigator, table.effect(roll));
    }
}
