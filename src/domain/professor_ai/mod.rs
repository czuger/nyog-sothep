//! Professor Petersen's AI: his decisions when the computer plays him.

mod bluff;
mod counter_values;
mod fanatics_safety;
mod fight_choices;
mod movement_choice;
mod near_best;
mod nyog_choice;
mod petersen_ai;
mod placement_choice;
mod professor_weights;
mod setup_choice;
mod threats;

pub use near_best::pick_near_best;
pub use petersen_ai::PetersenAi;
pub use professor_weights::ProfessorWeights;

#[cfg(test)]
mod ai_tests;

/// A game in its first movement phase, built by hand for the AI tests.
#[cfg(test)]
pub(crate) fn test_state() -> crate::domain::turn::GameState {
    use crate::domain::turn::{GameState, Phase, Professor, Prompt, RuleSet, Side};
    use crate::domain::{CounterPool, Investigator, InvestigatorId, LocationId, NyogSothep};
    GameState {
        human: Side::Investigators,
        rules: RuleSet::Original,
        turn: 1,
        phase: Phase::Moves,
        prompt: Some(Prompt::Moves),
        outcome: None,
        professor: Professor {
            hp: 14,
            location: LocationId::Worcester,
        },
        investigators: [
            InvestigatorId::Armitage,
            InvestigatorId::Carter,
            InvestigatorId::Barnes,
            InvestigatorId::Walker,
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
