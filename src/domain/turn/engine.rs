use crate::domain::dice::Dice;
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::{Clue, ClueKind};
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::history::History;
use crate::domain::turn::investigator_brain::InvestigatorBrain;
use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::professor_brain::ProfessorBrain;
use crate::domain::turn::prompt::{Answer, Prompt};
use crate::domain::turn::recorded_dice::RecordedDice;
use crate::domain::turn::replay_board::ReplayBoard;
use crate::domain::turn::replay_step::ReplayStep;
use crate::domain::turn::rule_sets::rules::Rules;

/// The turn engine: plays the game forward until the human must decide or the game ends.
///
/// Every decision is a [`Prompt`]. The computer's side answers its prompts synchronously through
/// its brain, so the loop only pauses on a prompt for the human.
pub struct Engine<'a> {
    pub(super) state: &'a mut GameState,
    pub(super) dice: RecordedDice<'a>,
    pub(super) professor: &'a mut dyn ProfessorBrain,
    pub(super) investigators: &'a mut dyn InvestigatorBrain,
    pub(super) log: Vec<LogLine>,
    pub(super) replay: Vec<ReplayStep>,
    /// The board of the last replay step (none before the first step of a new game).
    pub(super) replayed_board: Option<ReplayBoard>,
}

impl<'a> Engine<'a> {
    /// An engine working on `state` with the given dice; the brain of the side the human does
    /// not play takes that side's decisions (the other one is never asked).
    pub fn new(
        state: &'a mut GameState,
        dice: &'a mut dyn Dice,
        professor: &'a mut dyn ProfessorBrain,
        investigators: &'a mut dyn InvestigatorBrain,
    ) -> Engine<'a> {
        let replayed_board = Some(ReplayBoard::of(state));
        Engine {
            state,
            dice: RecordedDice::new(dice),
            professor,
            investigators,
            log: Vec::new(),
            replay: Vec::new(),
            replayed_board,
        }
    }

    /// The log lines written so far.
    pub fn into_log(self) -> Vec<LogLine> {
        self.log
    }

    /// The log lines and replay steps written so far.
    pub fn into_history(self) -> History {
        History {
            log: self.log,
            replay: self.replay,
        }
    }

    /// Advances the game until the human must answer a prompt or the game ends; the computer
    /// answers the prompts of its own side on the way.
    pub fn run(&mut self) {
        while !self.state.is_finished() {
            self.amend_replay();
            if let Some(prompt) = self.state.prompt.clone() {
                if prompt.side() == self.state.human || !self.play_computer(prompt) {
                    break;
                }
                continue;
            }
            match self.state.phase {
                Phase::Setup => self.ask_setup(),
                Phase::ProfessorMove => self.professor_move(),
                Phase::ProfessorMovement => self.professor_movement(),
                Phase::ProfessorSummon => self.professor_summon(),
                Phase::ProfessorPlace => self.professor_place(),
                Phase::ProfessorDraw { remaining } => self.rules().professor_draw(self, remaining),
                Phase::Moves => self.start_moves(),
                Phase::Action { investigator, step } => self.action(investigator, step),
                // Waiting phases without a prompt cannot happen; move on rather than loop.
                Phase::ProfessorFight { .. } => self.state.phase = Phase::ProfessorSummon,
                Phase::Banishment => self.state.phase = Phase::ProfessorPlace,
                Phase::Finished => break,
            }
        }
        self.settle_replay();
    }

    /// Applies the human's answer to the pending prompt, then runs the game forward.
    ///
    /// # Errors
    /// Refuses an answer that does not match the prompt, a prompt of the computer's side, or an
    /// answer that breaks a rule; the state is then unchanged.
    pub fn answer(&mut self, answer: Answer) -> Result<(), AnswerError> {
        if self.state.is_finished() {
            return Err(AnswerError::GameOver);
        }
        let prompt = self.state.prompt.clone().ok_or(AnswerError::Unexpected)?;
        if prompt.side() != self.state.human {
            return Err(AnswerError::Unexpected);
        }
        self.apply(prompt, answer)?;
        self.run();
        Ok(())
    }

    /// Applies an answer to the pending prompt, whichever side gives it.
    ///
    /// # Errors
    /// As [`Engine::answer`]; the state is then unchanged.
    pub(super) fn apply(&mut self, prompt: Prompt, answer: Answer) -> Result<(), AnswerError> {
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
            (Prompt::Professor(prompt), Answer::Professor(answer)) => {
                self.answer_professor(prompt, answer)?
            }
            _ => return Err(AnswerError::Unexpected),
        }
        Ok(())
    }

    /// Notes a clue in the investigators' notebook.
    pub(super) fn clue(&mut self, kind: ClueKind) {
        self.state.clues.push(Clue {
            turn: self.state.turn,
            kind,
        });
    }

    /// Asks a side, and parks the engine in `phase` until the answer.
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
        self.log(outcome.text(self.state.human));
    }

    /// Ends the game once the rules decide it.
    pub(super) fn check_end(&mut self) {
        if let Some(outcome) = self.rules().outcome(self.state) {
            self.finish(outcome);
        }
    }

    /// The rules of this game.
    pub(super) fn rules(&self) -> &'static dyn Rules {
        self.state.rules.rules()
    }

    /// Moves the action phase on to the given step of the same investigator.
    pub(super) fn action_step(&mut self, investigator: usize, step: ActionStep) {
        self.state.phase = Phase::Action { investigator, step };
    }
}
