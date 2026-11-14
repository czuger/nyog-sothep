use crate::domain::counter_kind::CounterKind;
use crate::domain::table1;
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor_answers::pick;
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

impl Engine<'_> {
    /// One Table 1 draw, then the next; the round goes on once none remains.
    pub(in crate::domain::turn) fn table1_draw(&mut self, remaining: u8) {
        let Some(left) = remaining.checked_sub(1) else {
            self.finish_draws();
            return;
        };
        self.state.phase = Phase::ProfessorDraw { remaining: left };
        let Some(options) = table1::roll(&self.state.counters, &mut self.dice) else {
            return;
        };
        match options.as_slice() {
            [only] => self.take_table1_counter(*only),
            _ => self.ask(
                Prompt::Professor(ProfessorPrompt::Table1 { options }),
                self.state.phase,
            ),
        }
    }

    pub(in crate::domain::turn) fn answer_table1(
        &mut self,
        options: &[CounterKind],
        picked: CounterKind,
    ) -> Result<(), AnswerError> {
        let kind = pick(options, picked)?;
        self.state.prompt = None;
        self.take_table1_counter(kind);
        Ok(())
    }

    /// Draws a counter; with six in hand, the professor must discard one.
    fn take_table1_counter(&mut self, kind: CounterKind) {
        self.state.counters.draw(kind);
        self.reveal(format!(
            "Le professeur tire un pion de la table 1 : {}.",
            kind.name()
        ));
        let hand = self.state.counters.hand();
        self.discard_over_limit(hand);
    }
}
