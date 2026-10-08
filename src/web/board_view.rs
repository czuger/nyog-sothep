use crate::domain::map::{BOARD_HEIGHT, BOARD_WIDTH, location};
use crate::domain::{InvestigatorStatus, LocationId};
use crate::service::{GameView, ProfessorSighting};
use crate::web::screen::Screen;

/// Horizontal step between tokens stacked on the same location, in board pixels.
const STACK_STEP: i32 = 46;

/// A location of the map, as a click target.
pub struct Place {
    /// Location code, used as the form value.
    pub code: &'static str,
    /// French name.
    pub name: &'static str,
    /// Pixel position on the board picture.
    pub x: u32,
    /// Pixel position on the board picture.
    pub y: u32,
    /// The input drawn on the place when cities are picked on the map (`checkbox` or `radio`).
    pub pick: Option<&'static str>,
}

/// An investigator token the human can move on the map.
pub struct Mover {
    /// The form field the move fills.
    pub field: String,
    /// Location code of the investigator.
    pub from: &'static str,
    /// Location codes the investigator may enter, separated by spaces.
    pub targets: String,
}

/// A token drawn on the map.
pub struct BoardToken {
    /// Image path.
    pub image: String,
    /// French tooltip.
    pub title: String,
    /// Pixel position of the token's centre.
    pub x: i32,
    /// Pixel position of the token's centre.
    pub y: i32,
    /// Extra CSS classes.
    pub class: &'static str,
    /// A small label under the token (« T3 » for an old sighting).
    pub label: Option<String>,
    /// Draws a boat under an investigator at sea.
    pub at_sea: bool,
    /// Set when the human can move this token.
    pub mover: Option<Mover>,
}

/// The map and everything drawn on it.
pub struct BoardView {
    /// Width of the board picture.
    pub width: u32,
    /// Height of the board picture.
    pub height: u32,
    /// Every location.
    pub places: Vec<Place>,
    /// Every token, in drawing order.
    pub tokens: Vec<BoardToken>,
}

impl BoardView {
    /// Lays out the visible tokens of a game on the map.
    pub fn of(view: &GameView, screen: Screen) -> BoardView {
        let pick = match screen {
            Screen::Cities => Some("checkbox"),
            Screen::Banishment => Some("radio"),
            _ => None,
        };
        let places = LocationId::ALL
            .into_iter()
            .map(|id| {
                let info = location(id);
                Place {
                    code: id.code(),
                    name: info.name,
                    x: info.x,
                    y: info.y,
                    pick: pick.filter(|_| id.is_city()),
                }
            })
            .collect();
        let mut tokens = Vec::new();
        for id in LocationId::ALL {
            tokens.extend(stack(view, id));
        }
        BoardView {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            places,
            tokens,
        }
    }
}

/// The tokens on one location, spread side by side around its dot.
fn stack(view: &GameView, id: LocationId) -> Vec<BoardToken> {
    let mut tokens = Vec::new();
    if view.destroyed.contains(&id) {
        tokens.push(token(
            "/img/token-destroyed-city.png",
            format!("{id} est détruite"),
            "",
        ));
    }
    for (_, kind) in view.counters.iter().filter(|(city, _)| *city == id) {
        tokens.push(token(
            &format!("/img/token-{}.png", kind.code()),
            format!("{} à {id}", kind.name()),
            "",
        ));
    }
    if view.nyog_location == Some(id) {
        tokens.push(token(
            "/img/token-nyog-sothep.png",
            format!("Nyog'Sothep {}", id.at_phrase()),
            "",
        ));
    }
    tokens.extend(professor_token(view, id));
    for investigator in view
        .investigators
        .iter()
        .filter(|i| i.location == id && i.status == InvestigatorStatus::Active)
    {
        let mut drawn = token(
            &format!("/img/token-investigator-{}.png", investigator.id.code()),
            format!("{} — {} SAN", investigator.id.name(), investigator.san),
            "investigator",
        );
        drawn.at_sea = id.is_sea();
        drawn.mover = investigator.moves.as_ref().map(|moves| Mover {
            field: format!("move_{}", investigator.index),
            from: id.code(),
            targets: moves
                .destinations
                .iter()
                .map(|to| to.code())
                .collect::<Vec<_>>()
                .join(" "),
        });
        tokens.push(drawn);
    }
    let info = location(id);
    let count = tokens.len() as i32;
    for (index, drawn) in tokens.iter_mut().enumerate() {
        drawn.x = info.x as i32 + (2 * index as i32 - (count - 1)) * STACK_STEP / 2;
        drawn.y = info.y as i32;
    }
    tokens
}

fn professor_token(view: &GameView, id: LocationId) -> Option<BoardToken> {
    match view.professor {
        ProfessorSighting::Visible { at } if at == id => Some(token(
            "/img/token-professor.png",
            format!("Le professeur Petersen {}", id.at_phrase()),
            "",
        )),
        ProfessorSighting::LastSeen { at, turn } if at == id => {
            let mut drawn = token(
                "/img/token-professor.png",
                format!("Le professeur a été vu {} au tour {turn}", id.at_phrase()),
                "last-seen",
            );
            drawn.label = Some(format!("T{turn}"));
            Some(drawn)
        }
        _ => None,
    }
}

fn token(image: &str, title: String, class: &'static str) -> BoardToken {
    BoardToken {
        image: image.to_string(),
        title,
        x: 0,
        y: 0,
        class,
        label: None,
        at_sea: false,
        mover: None,
    }
}
