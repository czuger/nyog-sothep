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
use crate::domain::turn::engine::Engine;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::log_line::LogLine;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor::{Professor, RouteStage};
use crate::domain::turn::professor_brain::ProfessorBrain;

/// The professor's start when his brain picks something that is not a city.
const FALLBACK_START: LocationId = LocationId::Worcester;
/// Nyog'Sothep's city when his brain picks something that is not a city.
const FALLBACK_NYOG_CITY: LocationId = LocationId::Concord;

impl Engine<'_> {
    /// Sets up a new game and plays it until the human's first decision.
    ///
    /// The investigators roll their SAN and land in Nantucket; the professor secretly picks his
    /// start and Nyog'Sothep's city, and draws his starting counters.
    pub fn new_game(
        investigators: &[InvestigatorId],
        dice: &mut dyn Dice,
        brain: &mut dyn ProfessorBrain,
    ) -> (GameState, Vec<LogLine>) {
        let investigators: Vec<Investigator> = investigators
            .iter()
            .map(|id| {
                let san = i32::from(dice.roll(STARTING_SAN_DICE)) + STARTING_SAN_BONUS;
                Investigator::new(*id, san)
            })
            .collect();
        let mut state = blank_state(investigators);
        let (start, city) = brain.setup(&state);
        state.professor.location = if start.is_city() {
            start
        } else {
            FALLBACK_START
        };
        state.nyog = NyogSothep::new(if city.is_city() {
            city
        } else {
            FALLBACK_NYOG_CITY
        });
        state.route_sheet.push(RouteStage {
            turn: 0,
            location: state.professor.location,
            counter: None,
        });
        let mut engine = Engine::new(&mut state, dice, brain);
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
        for _ in 0..STARTING_DRAWS {
            engine.draw_counter();
        }
        engine.run();
        let log = engine.into_log();
        (state, log)
    }
}

fn blank_state(investigators: Vec<Investigator>) -> GameState {
    GameState {
        turn: 0,
        phase: Phase::ProfessorMove,
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
