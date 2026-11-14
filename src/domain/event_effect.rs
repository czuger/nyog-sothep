use crate::domain::professor_question::ProfessorQuestion;

/// What an event of Table 2 or 3 does, as data applied by the turn engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EventEffect {
    /// SAN change: negative for a loss, positive for a gain.
    pub san: i32,
    /// A weapon is gained.
    pub gains_weapon: bool,
    /// The weapon, if any, is lost.
    pub loses_weapon: bool,
    /// A protective sign is gained.
    pub gains_sign: bool,
    /// A medallion is gained.
    pub gains_medallion: bool,
    /// The banishment spell is learned.
    pub learns_spell: bool,
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
