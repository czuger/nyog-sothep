use crate::domain::counter_pool::CounterPool;
use crate::domain::dice::Dice;
use crate::domain::investigator::{
    Investigator, STARTING_CITY, STARTING_SAN_BONUS, STARTING_SAN_DICE,
};
use crate::domain::investigator_id::InvestigatorId;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::NyogSothep;
use crate::domain::professor_fight::PROFESSOR_HP;
use crate::domain::table1::STARTING_DRAWS;
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::investigator_brain::InvestigatorBrain;
use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor::{Professor, RouteStage};
use crate::domain::turn::professor_brain::ProfessorBrain;
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;
use crate::domain::turn::side::Side;

/// The professor's start when the computer's pick is refused.
pub(super) const FALLBACK_START: LocationId = LocationId::Worcester;
/// Nyog'Sothep's city when the computer's pick is refused.
pub(super) const FALLBACK_NYOG_CITY: LocationId = LocationId::Concord;

impl Engine<'_> {
    /// Sets up a new game and plays it until the human's first decision.
    ///
    /// The investigators roll their SAN and land in Nantucket; the professor secretly picks his
    /// start and Nyog'Sothep's city, and draws his starting counters.
    ///
    /// # Arguments
    /// * `human` - the side the human plays; the matching brain is never asked.
    pub fn new_game(
        investigators: &[InvestigatorId],
        human: Side,
        dice: &mut dyn Dice,
        professor: &mut dyn ProfessorBrain,
        investigator_brain: &mut dyn InvestigatorBrain,
    ) -> (GameState, Vec<LogLine>) {
        let investigators: Vec<Investigator> = investigators
            .iter()
            .map(|id| {
                let san = i32::from(dice.roll(STARTING_SAN_DICE)) + STARTING_SAN_BONUS;
                Investigator::new(*id, san)
            })
            .collect();
        let mut state = blank_state(investigators, human);
        let mut engine = Engine::new(&mut state, dice, professor, investigator_brain);
        let names: Vec<&str> = engine
            .state
            .investigators
            .iter()
            .map(|investigator| investigator.id.name())
            .collect();
        engine.log(format!(
            "Mai 1921. {} débarquent à Nantucket sur les traces du professeur Petersen.",
            names.join(", ")
        ));
        engine.run();
        let log = engine.into_log();
        (state, log)
    }

    /// Asks the professor for his start and Nyog'Sothep's city.
    pub(super) fn ask_setup(&mut self) {
        self.ask(Prompt::Professor(ProfessorPrompt::Setup), Phase::Setup);
    }

    /// Writes the professor's secret choices, then he draws his starting counters.
    pub(super) fn answer_setup(
        &mut self,
        start: LocationId,
        nyog_city: LocationId,
    ) -> Result<(), AnswerError> {
        if !start.is_city() || !nyog_city.is_city() {
            return Err(AnswerError::NotACity);
        }
        self.state.prompt = None;
        self.state.professor.location = start;
        self.state.nyog = NyogSothep::new(nyog_city);
        self.state.route_sheet.push(RouteStage {
            turn: 0,
            location: start,
            counter: None,
        });
        self.state.phase = Phase::ProfessorDraw {
            remaining: STARTING_DRAWS,
        };
        Ok(())
    }
}

fn blank_state(investigators: Vec<Investigator>, human: Side) -> GameState {
    GameState {
        human,
        turn: 0,
        phase: Phase::Setup,
        prompt: None,
        outcome: None,
        professor: Professor {
            hp: PROFESSOR_HP,
            location: STARTING_CITY,
        },
        investigators,
        counters: CounterPool::full(),
        nyog: NyogSothep::new(FALLBACK_NYOG_CITY),
        destroyed: Vec::new(),
        route_sheet: Vec::new(),
        clues: Vec::new(),
        sea_announcements: false,
        move_in_the_open: false,
    }
}
