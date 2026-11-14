use crate::domain::turn::game_state::GameState;
use crate::domain::turn::prompt::{Answer, Prompt};

/// The decisions of the investigators when the computer plays them.
pub trait InvestigatorBrain {
    /// Answers one of the investigators' prompts.
    ///
    /// `state` is [`GameState::public_copy`]: the professor's secrets are not in it. The engine
    /// checks the answer like a human's and falls back to a legal one when it is refused.
    fn answer(&mut self, state: &GameState, prompt: &Prompt) -> Answer;
}
