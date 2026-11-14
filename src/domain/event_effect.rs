use crate::domain::item::Item;
use crate::domain::professor_question::ProfessorQuestion;

/// What an event does, as data applied by the turn engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EventEffect {
    /// SAN change: negative for a loss, positive for a gain.
    pub san: i32,
    /// An item (or the banishment spell) is gained.
    pub gains: Option<Item>,
    /// The weapon, if any, is lost.
    pub loses_weapon: bool,
    /// Back to the location the investigator left.
    pub goes_back: bool,
    /// The next turn is lost.
    pub skips_next_turn: bool,
    /// The next turn is lost unless the investigator takes to the sea.
    pub breaks_car: bool,
    /// The investigator moves and plays again at once.
    pub plays_again: bool,
    /// Something the professor must answer or do.
    pub question: Option<ProfessorQuestion>,
}
