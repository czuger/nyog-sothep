use crate::domain::turn::engine::Engine;
use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::replay_board::ReplayBoard;
use crate::domain::turn::replay_step::ReplayStep;

impl Engine<'_> {
    /// Writes a public log line for the current turn; the replay shows it too.
    pub(super) fn log(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.log.push(LogLine {
            turn: self.state.turn,
            text: text.clone(),
        });
        self.record(Some(text));
    }

    /// Writes a public log line that the replay replaces with the secret it hides.
    pub(super) fn log_revealing(&mut self, public: impl Into<String>, revealed: String) {
        self.log.push(LogLine {
            turn: self.state.turn,
            text: public.into(),
        });
        self.record(Some(revealed));
    }

    /// A step only the replay shows: a secret of the professor.
    pub(super) fn reveal(&mut self, text: String) {
        self.record(Some(text));
    }

    /// Shows on the last step of this engine the board changes made since: they are its
    /// consequences, written after its text (the effects of an event).
    pub(super) fn amend_replay(&mut self) {
        let Some(step) = self.replay.last_mut() else {
            return;
        };
        let board = ReplayBoard::of(self.state);
        if self.replayed_board.as_ref() != Some(&board) {
            step.board = Some(board.clone());
            self.replayed_board = Some(board);
        }
    }

    /// Records the dice and board changes that no step has shown yet.
    pub(super) fn settle_replay(&mut self) {
        self.amend_replay();
        let changed = self.replayed_board.as_ref() != Some(&ReplayBoard::of(self.state));
        if changed || !self.dice.rolls().is_empty() {
            self.record(None);
        }
    }

    /// Appends a replay step with the dice rolled since the previous one, and the board when it
    /// changed.
    fn record(&mut self, text: Option<String>) {
        let board = ReplayBoard::of(self.state);
        let changed = self.replayed_board.as_ref() != Some(&board);
        self.replay.push(ReplayStep {
            turn: self.state.turn,
            text,
            dice: self.dice.take_rolls(),
            board: changed.then(|| board.clone()),
        });
        self.replayed_board = Some(board);
    }
}
