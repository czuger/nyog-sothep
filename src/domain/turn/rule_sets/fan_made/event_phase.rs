use crate::domain::counter::CounterPlace;
use crate::domain::dice::Dice;
use crate::domain::item::{Gear, Item};
use crate::domain::location_id::LocationId;
use crate::domain::travel;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::phase::ActionStep;
use crate::domain::turn::rule_sets::fan_made::event_tables::FanTable;

impl Engine<'_> {
    /// No event once Nyog'Sothep is out; otherwise the Ibn-Ghazi powder when it may teach
    /// something, or one d20 on the table of the moment.
    pub(super) fn fan_event(&mut self, investigator: usize) {
        if self.state.nyog.is_summoned() {
            self.action_step(investigator, ActionStep::Fight);
            return;
        }
        let actor = &self.state.investigators[investigator];
        let powder = actor.holds(Item::Gear(Gear::IbnGhaziPowder));
        if powder && !unknown_neighbours(self.state, actor.location).is_empty() {
            self.scatter_powder(investigator);
            return;
        }
        let table = FanTable::for_fanatics_cities(self.state.counters.fanatics_cities().len());
        let roll = self.dice.d20();
        let name = self.state.investigators[investigator].id.name();
        self.log(format!(
            "{name} tire un événement sur la table {} (1d20 : {roll}) : « {} »",
            table.title(),
            table.text(roll)
        ));
        self.resolve_event(investigator, table.effect(roll));
    }

    /// The powder reveals the counters hidden in the neighbouring cities, instead of the event.
    fn scatter_powder(&mut self, investigator: usize) {
        let actor = &mut self.state.investigators[investigator];
        actor.spend(Gear::IbnGhaziPowder);
        let (name, here) = (actor.id.name(), actor.location);
        self.log(format!(
            "{name} répand la poudre d'Ibn-Ghazi : les villes voisines n'ont plus de secret."
        ));
        for city in unknown_neighbours(self.state, here) {
            let kind = self
                .state
                .counters
                .placed_at(city)
                .map(|counter| counter.kind);
            if kind.is_some() {
                let discovered = CounterPlace::Placed {
                    city,
                    discovered: true,
                };
                self.state.counters.set_place_at(city, discovered);
            }
            let clue = ClueKind::EncounterKind { city, kind };
            self.log(clue.describe());
            self.clue(clue);
        }
        self.action_step(investigator, ActionStep::AfterEvent { plays_again: false });
    }
}

/// The neighbouring cities, still standing, whose counter (if any) is not known.
fn unknown_neighbours(state: &GameState, from: LocationId) -> Vec<LocationId> {
    travel::neighbours(from)
        .iter()
        .copied()
        .filter(|city| city.is_city() && !state.is_destroyed(*city))
        .filter(|city| {
            !matches!(
                state.counters.placed_at(*city).map(|counter| counter.place),
                Some(CounterPlace::Placed {
                    discovered: true,
                    ..
                })
            )
        })
        .collect()
}
