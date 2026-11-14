use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::encounter::EncounterOutcome;
use crate::domain::investigator::Investigator;
use crate::domain::item::Item;
use crate::domain::location_id::LocationId;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::prompt::MoveOrder;

/// The points where a rule set departs from the shared turn engine.
///
/// The engine calls these hooks without knowing which rules it plays: `RuleSet::rules` is the
/// only place that tells them apart.
pub(crate) trait Rules {
    /// The investigator may draw an event after the encounter: ask for it, or draw it.
    fn ask_event(&self, engine: &mut Engine<'_>, investigator: usize);

    /// An event gives the investigator an item.
    fn receive(&self, engine: &mut Engine<'_>, investigator: usize, item: Item);

    /// The most SAN an investigator may have.
    fn san_cap(&self, investigator: &Investigator) -> i32;

    /// Whether the psychoanalyst may still be consulted.
    fn psychoanalyst_open(&self, state: &GameState) -> bool;

    /// Whether the investigator crosses borders without a roll.
    fn free_crossing(&self, investigator: &Investigator) -> bool;

    /// Just before an investigator's move order is carried out.
    fn before_move(&self, engine: &mut Engine<'_>, investigator: usize, order: MoveOrder);

    /// Resolves the meeting of an investigator with a placed counter.
    fn encounter(
        &self,
        kind: CounterKind,
        investigator: &Investigator,
        dice: &mut dyn Dice,
    ) -> EncounterOutcome;

    /// One of the professor's draws at the end of his turn (`remaining` still to make).
    fn professor_draw(&self, engine: &mut Engine<'_>, remaining: u8);

    /// An investigator wounded the professor, who survived.
    fn professor_wounded(&self, engine: &mut Engine<'_>);

    /// Whether Nyog'Sothep and the professor stay still this turn.
    fn nyog_rests(&self, engine: &mut Engine<'_>) -> bool;

    /// After an investigator's action phase, once Nyog'Sothep is out.
    fn update_banishment_target(&self, engine: &mut Engine<'_>);

    /// Nyog'Sothep entered a city: the spell holders there may cast the banishment spell.
    fn cast_banishment(&self, engine: &mut Engine<'_>, city: LocationId);

    /// The result of the game, once it is decided.
    fn outcome(&self, state: &GameState) -> Option<Outcome>;

    /// Investigators knowing the spell needed to cast it together.
    fn banishers_needed(&self) -> usize;

    /// How likely Nyog'Sothep entering `city` now would be banished there (0 to 1), for the
    /// computer's professor.
    fn banishment_chance(&self, state: &GameState, city: LocationId) -> f64;
}

/// The spell holders still in the game standing in a city, in playing order.
pub(crate) fn spell_holders_at(state: &GameState, city: LocationId) -> Vec<usize> {
    state
        .investigators_at(city)
        .into_iter()
        .filter(|index| state.investigators[*index].spell)
        .collect()
}
