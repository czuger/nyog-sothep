use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::table1::{self, HAND_LIMIT};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor::RouteStage;
use crate::domain::turn::professor_answers::pick;
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

/// One Table 1 draw per round.
const DRAWS_PER_TURN: u8 = 1;

impl Engine<'_> {
    /// Offers to place a counter where the professor stands; the stage note is written once he
    /// has decided.
    pub(super) fn professor_place(&mut self) {
        self.state.phase = Phase::ProfessorDraw {
            remaining: DRAWS_PER_TURN,
        };
        let options = self.placement_options(self.state.professor.location);
        if options.is_empty() {
            self.write_stage(None);
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::Placement { options }),
            self.state.phase,
        );
    }

    pub(super) fn answer_placement(
        &mut self,
        options: &[CounterKind],
        picked: Option<CounterKind>,
    ) -> Result<(), AnswerError> {
        let picked = picked.map(|kind| pick(options, kind)).transpose()?;
        self.state.prompt = None;
        if let Some(kind) = picked {
            self.state
                .counters
                .place(kind, self.state.professor.location);
        }
        self.write_stage(picked);
        Ok(())
    }

    fn write_stage(&mut self, counter: Option<CounterKind>) {
        self.state.route_sheet.push(RouteStage {
            turn: self.state.turn,
            location: self.state.professor.location,
            counter,
        });
    }

    /// The counters of the hand that may be placed in a location.
    pub(super) fn placement_options(&self, location: LocationId) -> Vec<CounterKind> {
        if !location.is_city() || self.state.counters.placed_at(location).is_some() {
            return Vec::new();
        }
        let mut options = self.state.counters.hand();
        options.retain(|kind| kind.is_placeable() && (!kind.port_only() || location.is_port()));
        options.dedup();
        options
    }

    /// One Table 1 draw, then the next; the round goes on once none remains.
    pub(super) fn professor_draw(&mut self, remaining: u8) {
        let Some(left) = remaining.checked_sub(1) else {
            self.state.phase = match self.state.turn {
                0 => Phase::ProfessorMove,
                _ => Phase::Moves,
            };
            return;
        };
        self.state.phase = Phase::ProfessorDraw { remaining: left };
        let Some(options) = table1::roll(&self.state.counters, self.dice) else {
            return;
        };
        match options.as_slice() {
            [only] => self.take_counter(*only),
            _ => self.ask(
                Prompt::Professor(ProfessorPrompt::Table1 { options }),
                self.state.phase,
            ),
        }
    }

    pub(super) fn answer_table1(
        &mut self,
        options: &[CounterKind],
        picked: CounterKind,
    ) -> Result<(), AnswerError> {
        let kind = pick(options, picked)?;
        self.state.prompt = None;
        self.take_counter(kind);
        Ok(())
    }

    /// Draws a counter; with six in hand, the professor must discard one.
    fn take_counter(&mut self, kind: CounterKind) {
        self.state.counters.draw(kind);
        let hand = self.state.counters.hand();
        if hand.len() > HAND_LIMIT {
            self.ask(
                Prompt::Professor(ProfessorPrompt::Discard { hand }),
                self.state.phase,
            );
        }
    }

    pub(super) fn answer_discard(
        &mut self,
        hand: &[CounterKind],
        picked: CounterKind,
    ) -> Result<(), AnswerError> {
        let kind = pick(hand, picked)?;
        self.state.prompt = None;
        self.state.counters.discard(kind);
        Ok(())
    }
}
