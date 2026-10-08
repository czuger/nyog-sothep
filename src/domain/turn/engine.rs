use crate::domain::dice::Dice;
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::{Clue, ClueKind};
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::professor_brain::ProfessorBrain;
use crate::domain::turn::prompt::{Answer, Prompt};

/// The turn engine: plays the game forward until the human must decide or the game ends.
///
/// The professor's decisions are taken synchronously by its brain, so the loop only pauses on a
/// [`Prompt`] for the human.
pub struct Engine<'a> {
    pub(super) state: &'a mut GameState,
    pub(super) dice: &'a mut dyn Dice,
    pub(super) brain: &'a mut dyn ProfessorBrain,
    log: Vec<LogLine>,
}

impl<'a> Engine<'a> {
    /// An engine working on `state` with the given dice and professor.
    pub fn new(
        state: &'a mut GameState,
        dice: &'a mut dyn Dice,
        brain: &'a mut dyn ProfessorBrain,
    ) -> Engine<'a> {
        Engine {
            state,
            dice,
            brain,
            log: Vec::new(),
        }
    }

    /// The log lines written so far.
    pub fn into_log(self) -> Vec<LogLine> {
        self.log
    }

    /// Advances the game until the human must answer a prompt or the game ends.
    pub fn run(&mut self) {
        while self.state.prompt.is_none() && !self.state.is_finished() {
            match self.state.phase {
                Phase::ProfessorMove => self.professor_move(),
                Phase::ProfessorSummon => self.professor_summon(),
                Phase::ProfessorPlace => self.professor_place(),
                Phase::ProfessorDraw => self.professor_draw(),
                Phase::Moves => self.start_moves(),
                Phase::Action { investigator, step } => self.action(investigator, step),
                // Waiting phases without a prompt cannot happen; move on rather than loop.
                Phase::ProfessorFight { .. } => self.state.phase = Phase::ProfessorSummon,
                Phase::Banishment => self.state.phase = Phase::ProfessorPlace,
                Phase::Finished => break,
            }
        }
    }

    /// Applies the human's answer to the pending prompt, then runs the game forward.
    ///
    /// # Errors
    /// Refuses an answer that does not match the prompt or breaks a rule; the state is then
    /// unchanged.
    pub fn answer(&mut self, answer: Answer) -> Result<(), AnswerError> {
        if self.state.is_finished() {
            return Err(AnswerError::GameOver);
        }
        let prompt = self.state.prompt.clone().ok_or(AnswerError::Unexpected)?;
        match (prompt, answer) {
            (Prompt::Moves, Answer::Moves(orders)) => self.answer_moves(&orders)?,
            (Prompt::Table { investigator }, Answer::Table(table)) => {
                self.state.prompt = None;
                self.draw_event(investigator, table);
            }
            (
                Prompt::Cities {
                    investigator,
                    question,
                },
                Answer::Cities(cities),
            ) => self.answer_cities(investigator, question, &cities)?,
            (Prompt::PlayAgain { investigator }, Answer::PlayAgain(order)) => {
                self.answer_play_again(investigator, order)?
            }
            (
                Prompt::Fight {
                    investigator,
                    options,
                },
                Answer::Fight(choice),
            ) => self.answer_fight(investigator, &options, choice)?,
            (Prompt::BanishmentCity, Answer::BanishmentCity(city)) => {
                self.answer_banishment_city(city)?
            }
            _ => return Err(AnswerError::Unexpected),
        }
        self.run();
        Ok(())
    }

    /// Writes a public log line for the current turn.
    pub(super) fn log(&mut self, text: impl Into<String>) {
        self.log.push(LogLine {
            turn: self.state.turn,
            text: text.into(),
        });
    }

    /// Notes a clue in the investigators' notebook.
    pub(super) fn clue(&mut self, kind: ClueKind) {
        self.state.clues.push(Clue {
            turn: self.state.turn,
            kind,
        });
    }

    /// Asks the human, and parks the engine in `phase` until the answer.
    pub(super) fn ask(&mut self, prompt: Prompt, phase: Phase) {
        self.state.prompt = Some(prompt);
        self.state.phase = phase;
    }

    /// Ends the game.
    pub(super) fn finish(&mut self, outcome: Outcome) {
        if self.state.is_finished() {
            return;
        }
        self.state.outcome = Some(outcome);
        self.state.phase = Phase::Finished;
        self.state.prompt = None;
        self.log(outcome.text());
    }

    /// Ends the game when too few investigators remain.
    pub(super) fn check_end(&mut self) {
        let active = self.state.active_investigators().len();
        if active == 0 {
            self.finish(Outcome::InvestigatorsLost);
        } else if self.state.nyog.is_summoned() && active < crate::domain::BANISHERS_NEEDED {
            self.finish(Outcome::NyogSothepReigns);
        }
    }

    /// Moves the action phase on to the given step of the same investigator.
    pub(super) fn action_step(&mut self, investigator: usize, step: ActionStep) {
        self.state.phase = Phase::Action { investigator, step };
    }
}
