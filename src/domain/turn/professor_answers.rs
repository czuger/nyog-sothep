use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::professor_prompt::{ProfessorAnswer, ProfessorPrompt};

impl Engine<'_> {
    /// Applies the professor's answer to his prompt.
    ///
    /// # Errors
    /// Refuses an answer that does not match the prompt or is not among its options; the state
    /// is then unchanged.
    pub(super) fn answer_professor(
        &mut self,
        prompt: ProfessorPrompt,
        answer: ProfessorAnswer,
    ) -> Result<(), AnswerError> {
        use ProfessorAnswer as A;
        use ProfessorPrompt as P;
        match (&prompt, answer) {
            (P::Setup, A::Setup { start, nyog_city }) => self.answer_setup(start, nyog_city),
            (P::Teleport { options }, A::Teleport(picked)) => self.answer_teleport(options, picked),
            (P::Movement { options }, A::Location(to)) => self.answer_movement(options, to),
            (P::Attack { candidates, forced }, A::Attack(picked)) => {
                self.answer_attack(candidates, *forced, picked)
            }
            (
                P::Carry {
                    investigator,
                    options,
                },
                A::Location(to),
            ) => self.answer_carry(*investigator, options, to),
            (
                P::Relocate {
                    options,
                    pushed_back,
                },
                A::Location(to),
            ) => self.answer_relocation(options, *pushed_back, to),
            (P::Summon, A::Summon(summons)) => {
                self.answer_summon(summons);
                Ok(())
            }
            (P::Placement { options }, A::Placement(picked)) => {
                self.answer_placement(options, picked)
            }
            (P::Table1 { options }, A::Counter(kind)) => self.answer_table1(options, kind),
            (P::Discard { hand }, A::Counter(kind)) => self.answer_discard(hand, kind),
            (P::NyogStep { options }, A::Location(to)) => self.answer_nyog_step(options, to),
            (P::NameCities { .. } | P::NameFanaticsCities, A::Cities(named)) => {
                self.answer_professor_names(&prompt, &named)
            }
            _ => Err(AnswerError::Unexpected),
        }
    }
}

/// The picked value when it is among the options.
///
/// # Errors
/// `NotAnOption` otherwise.
pub(super) fn pick<T: PartialEq + Copy>(options: &[T], picked: T) -> Result<T, AnswerError> {
    match options.contains(&picked) {
        true => Ok(picked),
        false => Err(AnswerError::NotAnOption),
    }
}
