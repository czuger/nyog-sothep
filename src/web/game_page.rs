use askama::Template;

use crate::domain::turn::{Prompt, RouteStage};
use crate::domain::{FightChoice, LocationId, ProfessorQuestion};
use crate::service::{GameView, InvestigatorView, ProfessorSighting};
use crate::web::board_view::BoardView;
use crate::web::screen::Screen;

/// One choice of a select or button list: the form value and its French label.
pub struct Choice {
    /// Form value.
    pub value: String,
    /// French label.
    pub label: String,
}

/// What the professor is asked about the cities the human names.
pub struct CityQuestion {
    /// French sentence explaining what to do.
    pub text: &'static str,
    /// How many cities to name.
    pub count: usize,
    /// The human may decline.
    pub optional: bool,
}

/// A line of the revealed route sheet.
pub struct RouteLine {
    /// The turn.
    pub turn: u32,
    /// Where the professor ended his move.
    pub place: String,
    /// What he placed there.
    pub counter: &'static str,
}

/// The game page.
#[derive(Template)]
#[template(path = "game.html")]
pub struct GamePage {
    /// The game as the investigators see it.
    pub view: GameView,
    /// The screen drawn for the pending prompt.
    pub screen: Screen,
    /// The map and its tokens.
    pub board: BoardView,
    /// Error from the last answer, if it was refused.
    pub error: Option<String>,
}

impl GamePage {
    /// Builds the page, with an optional error message from a refused answer.
    pub fn new(view: GameView, error: Option<String>) -> GamePage {
        let screen = Screen::of(&view);
        let board = BoardView::of(&view, screen);
        GamePage {
            view,
            screen,
            board,
            error,
        }
    }

    /// The game's private URL.
    pub fn game_path(&self) -> String {
        format!("/games/{}/{}", self.view.game_id, self.view.token)
    }

    /// The investigators the human moves now.
    pub fn movers(&self) -> Vec<&InvestigatorView> {
        self.view
            .investigators
            .iter()
            .filter(|investigator| investigator.moves.is_some())
            .collect()
    }

    /// The choices of an investigator's move select.
    pub fn move_choices(&self, investigator: &InvestigatorView) -> Vec<Choice> {
        let Some(moves) = &investigator.moves else {
            return Vec::new();
        };
        let mut choices = vec![Choice {
            value: "stay".to_string(),
            label: format!("Rester {}", investigator.location.at_phrase()),
        }];
        if moves.psychoanalyst {
            choices.push(Choice {
                value: "psychoanalyst".to_string(),
                label: "Consulter un psychanalyste (+1d6 SAN, rien d'autre ce tour)".to_string(),
            });
        }
        choices.extend(moves.destinations.iter().map(|to| Choice {
            value: to.code().to_string(),
            label: destination_label(investigator.location, *to),
        }));
        choices
    }

    /// The investigator the prompt is about.
    pub fn prompted(&self) -> Option<&InvestigatorView> {
        let index = match self.view.prompt.as_ref()? {
            Prompt::Table { investigator }
            | Prompt::Cities { investigator, .. }
            | Prompt::PlayAgain { investigator }
            | Prompt::Fight { investigator, .. } => *investigator,
            Prompt::Moves | Prompt::BanishmentCity => return None,
        };
        self.view.investigators.get(index)
    }

    /// The French name of the investigator the prompt is about.
    pub fn prompted_name(&self) -> &'static str {
        self.prompted()
            .map_or("", |investigator| investigator.id.name())
    }

    /// How many dice the prompted investigator rolls on an event table.
    pub fn event_dice(&self) -> u8 {
        self.prompted().map_or(1, |investigator| {
            match (investigator.sign, investigator.medallion) {
                (true, true) => 3,
                (true, false) => 2,
                _ => 1,
            }
        })
    }

    /// The question asked about the named cities.
    pub fn city_question(&self) -> CityQuestion {
        let question = match &self.view.prompt {
            Some(Prompt::Cities { question, .. }) => *question,
            _ => ProfessorQuestion::EncountersInTwoCities,
        };
        match question {
            ProfessorQuestion::EncounterInOneCity => CityQuestion {
                text: "Citez une ville : s'il y a une rencontre, le professeur doit la préciser.",
                count: 1,
                optional: false,
            },
            ProfessorQuestion::NyogSothepInTwoCities { optional } => CityQuestion {
                text: "Citez deux villes : le professeur doit dire si Nyog'Sothep doit apparaître dans l'une d'elles.",
                count: 2,
                optional,
            },
            _ => CityQuestion {
                text: "Citez deux villes : le professeur doit dire s'il y a des rencontres dans chacune.",
                count: 2,
                optional: false,
            },
        }
    }

    /// The fight choices open to the prompted investigator.
    pub fn fight_choices(&self) -> Vec<Choice> {
        let options: &[FightChoice] = match &self.view.prompt {
            Some(Prompt::Fight { options, .. }) => options,
            _ => &[],
        };
        options
            .iter()
            .map(|choice| Choice {
                value: choice.code().to_string(),
                label: choice.label().to_string(),
            })
            .collect()
    }

    /// French text of the professor's known position, for the banner.
    pub fn professor_status(&self) -> String {
        match self.view.professor {
            ProfessorSighting::Unknown => "position inconnue".to_string(),
            ProfessorSighting::LastSeen { at, turn } => {
                format!("vu {} au tour {turn}", at.at_phrase())
            }
            ProfessorSighting::Visible { at } => format!("{} (à découvert)", at.at_phrase()),
        }
    }

    /// The outcome's French text, once the game is over.
    pub fn outcome_text(&self) -> &'static str {
        self.view.outcome.map_or("", |outcome| outcome.text())
    }

    /// Whether the investigators won.
    pub fn investigators_won(&self) -> bool {
        self.view
            .outcome
            .is_some_and(|outcome| outcome.investigators_win())
    }

    /// The revealed route sheet.
    pub fn route_lines(&self) -> Vec<RouteLine> {
        self.view.route_sheet.iter().map(route_line).collect()
    }

    /// Nyog'Sothep's appearance city, once revealed.
    pub fn nyog_city_name(&self) -> String {
        self.view
            .nyog_city
            .map_or_else(String::new, |city| city.to_string())
    }
}

fn destination_label(from: LocationId, to: LocationId) -> String {
    match (from.is_sea(), to.is_sea()) {
        (false, true) => format!("Embarquer : {to}"),
        (true, true) => format!("Naviguer : {to}"),
        (true, false) => format!("Débarquer à {to}"),
        (false, false) => format!("Aller à {to}"),
    }
}

fn route_line(stage: &RouteStage) -> RouteLine {
    RouteLine {
        turn: stage.turn,
        place: stage.location.to_string(),
        counter: stage.counter.map_or("—", |kind| kind.name()),
    }
}
