//! Professor Petersen's AI: what the investigators know of him, and his decisions.

mod bluff;
mod counter_values;
mod fight_choices;
mod movement_choice;
mod near_best;
mod nyog_choice;
mod petersen_ai;
mod placement_choice;
mod professor_belief;
mod setup_choice;
mod threats;

pub use near_best::pick_near_best;
pub use petersen_ai::PetersenAi;
pub use professor_belief::ProfessorBelief;

#[cfg(test)]
mod ai_tests;

/// A game in its first movement phase, built by hand for the AI tests.
#[cfg(test)]
pub(crate) fn test_state() -> crate::domain::turn::GameState {
    use crate::domain::turn::{GameState, Phase, Professor, Prompt};
    use crate::domain::{CounterPool, Investigator, InvestigatorId, LocationId, NyogSothep};
    GameState {
        turn: 1,
        phase: Phase::Moves,
        prompt: Some(Prompt::Moves),
        outcome: None,
        professor: Professor {
            hp: 14,
            location: LocationId::Worcester,
        },
        investigators: [
            InvestigatorId::Poirot,
            InvestigatorId::Hercule,
            InvestigatorId::Sandy,
            InvestigatorId::Lemon,
        ]
        .into_iter()
        .map(|id| Investigator::new(id, 15))
        .collect(),
        counters: CounterPool::full(),
        nyog: NyogSothep::new(LocationId::Concord),
        destroyed: Vec::new(),
        route_sheet: Vec::new(),
        clues: Vec::new(),
        sea_announcements: false,
        move_in_the_open: false,
    }
}
