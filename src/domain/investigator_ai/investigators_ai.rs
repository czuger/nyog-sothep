use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::domain::investigator_ai::banishment_choice::choose_banishment_city;
use crate::domain::investigator_ai::fight_preference::choose_fight;
use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::investigator_ai::move_choice::{choose_moves, choose_replay};
use crate::domain::investigator_ai::question_choice::choose_cities;
use crate::domain::investigator_ai::table_choice::choose_table;
use crate::domain::turn::{Answer, GameState, InvestigatorBrain, Prompt};

/// The four investigators played by the computer.
///
/// Like `PetersenAi` it keeps no memory: every decision is computed from what the investigators
/// know, and its random generator only varies the choice among near-best options.
pub struct InvestigatorsAi {
    rng: StdRng,
    weights: InvestigatorWeights,
}

impl InvestigatorsAi {
    /// An AI seeded from the operating system.
    pub fn new() -> InvestigatorsAi {
        InvestigatorsAi {
            rng: rand::make_rng(),
            weights: InvestigatorWeights::default(),
        }
    }

    /// A reproducible AI, for simulations.
    pub fn seeded(seed: u64) -> InvestigatorsAi {
        InvestigatorsAi {
            rng: StdRng::seed_from_u64(seed),
            weights: InvestigatorWeights::default(),
        }
    }

    /// The same AI scoring its choices with other weights.
    pub fn with_weights(self, weights: InvestigatorWeights) -> InvestigatorsAi {
        InvestigatorsAi { weights, ..self }
    }
}

impl Default for InvestigatorsAi {
    fn default() -> InvestigatorsAi {
        InvestigatorsAi::new()
    }
}

impl InvestigatorBrain for InvestigatorsAi {
    fn answer(&mut self, state: &GameState, prompt: &Prompt) -> Answer {
        match prompt {
            Prompt::Moves => Answer::Moves(choose_moves(state, &self.weights, &mut self.rng)),
            Prompt::PlayAgain { investigator } => Answer::PlayAgain(choose_replay(
                state,
                &self.weights,
                *investigator,
                &mut self.rng,
            )),
            Prompt::Table { investigator } => {
                Answer::Table(choose_table(state, &self.weights, *investigator))
            }
            Prompt::Cities {
                investigator,
                question,
            } => Answer::Cities(choose_cities(
                state,
                *investigator,
                *question,
                &mut self.rng,
            )),
            Prompt::Fight { options, .. } => Answer::Fight(choose_fight(options)),
            // The engine never asks the investigators the professor's prompts; an empty
            // answer is refused and replaced by a legal one.
            Prompt::BanishmentCity | Prompt::Professor(_) => {
                Answer::BanishmentCity(choose_banishment_city(state, &self.weights))
            }
        }
    }
}
