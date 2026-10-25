use crate::domain::event_table::EventTable;
use crate::domain::map;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::move_options::MoveOptions;
use crate::domain::turn::professor_prompt::{ProfessorAnswer, ProfessorPrompt};
use crate::domain::turn::prompt::{Answer, MoveOrder, Prompt};
use crate::domain::turn::questions::with_nearest_cities;
use crate::domain::turn::setup::{FALLBACK_NYOG_CITY, FALLBACK_START};
use crate::domain::turn::side::Side;

impl Engine<'_> {
    /// The computer answers a prompt of its side, through the same checks as the human; a
    /// refused answer is replaced by a legal one.
    ///
    /// # Returns
    /// `false` when no answer could be applied, so that the engine stops instead of looping.
    pub(super) fn play_computer(&mut self, prompt: Prompt) -> bool {
        let answer = match prompt.side() {
            Side::Professor => self.professor_decides(&prompt),
            Side::Investigators => {
                let public = self.state.public_copy();
                Some(self.investigators.answer(&public, &prompt))
            }
        };
        if let Some(answer) = answer
            && self.apply(prompt.clone(), answer).is_ok()
        {
            return true;
        }
        fallback(self.state, &prompt).is_some_and(|answer| self.apply(prompt, answer).is_ok())
    }

    fn professor_decides(&mut self, prompt: &Prompt) -> Option<Answer> {
        let Prompt::Professor(prompt) = prompt else {
            return None;
        };
        Some(Answer::Professor(self.professor.answer(self.state, prompt)))
    }
}

/// A legal answer to any prompt, used when the computer's own answer is refused.
fn fallback(state: &GameState, prompt: &Prompt) -> Option<Answer> {
    let answer = match prompt {
        Prompt::Moves => Answer::Moves(
            (0..state.investigators.len())
                .filter(|index| MoveOptions::of(state, *index, false).is_some())
                .map(|index| (index, MoveOrder::Stay))
                .collect(),
        ),
        Prompt::Table { .. } => Answer::Table(EventTable::Two),
        Prompt::Cities { question, .. } => {
            let count = question.cities_named_by_investigator().unwrap_or(0);
            Answer::Cities(map::cities().take(count).collect())
        }
        Prompt::PlayAgain { .. } => Answer::PlayAgain(MoveOrder::Stay),
        Prompt::Fight { options, .. } => Answer::Fight(*options.first()?),
        Prompt::BanishmentCity => Answer::BanishmentCity(map::cities().next()?),
        Prompt::Professor(prompt) => Answer::Professor(professor_fallback(state, prompt)?),
    };
    Some(answer)
}

fn professor_fallback(state: &GameState, prompt: &ProfessorPrompt) -> Option<ProfessorAnswer> {
    use ProfessorAnswer as A;
    use ProfessorPrompt as P;
    let answer = match prompt {
        P::Setup => A::Setup {
            start: FALLBACK_START,
            nyog_city: FALLBACK_NYOG_CITY,
        },
        P::Teleport { .. } => A::Teleport(None),
        P::Movement { .. } => A::Location(state.professor.location),
        P::Attack { candidates, forced } => A::Attack(match forced {
            true => Some(*candidates.first()?),
            false => None,
        }),
        P::Carry { options, .. } | P::Relocate { options, .. } | P::NyogStep { options } => {
            A::Location(*options.first()?)
        }
        P::Summon => A::Summon(false),
        P::Placement { .. } => A::Placement(None),
        P::Table1 { options } => A::Counter(*options.first()?),
        P::Discard { hand } => A::Counter(*hand.first()?),
        P::NameCities { count } => A::Cities(with_nearest_cities(state.professor.location, *count)),
        P::NameFanaticsCities => A::Cities(with_nearest_cities(
            *state.counters.fanatics_cities().first()?,
            2,
        )),
    };
    Some(answer)
}
