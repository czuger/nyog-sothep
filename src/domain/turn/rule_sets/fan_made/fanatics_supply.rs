use crate::domain::counter_kind::CounterKind;
use crate::domain::turn::engine::Engine;

/// Turns between two groups of Fanatics joining the professor.
const FANATICS_EVERY: u32 = 3;

impl Engine<'_> {
    /// Every few turns a group of Fanatics joins the professor, who may keep as many as he likes.
    pub(super) fn fanatics_arrive(&mut self) {
        let turn = self.state.turn;
        if turn == 0 || !turn.is_multiple_of(FANATICS_EVERY) {
            return;
        }
        self.state.counters.supply(CounterKind::Fanatics);
        self.reveal("Un groupe de Fanatiques rejoint le professeur.".to_string());
    }
}
