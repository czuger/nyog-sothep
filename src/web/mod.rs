//! HTTP layer: routes, handlers and pages.

mod answer_handlers;
mod app_state;
mod board_view;
mod error_page;
mod game_handlers;
mod game_page;
mod lobby_handlers;
mod router;
mod screen;

pub use app_state::AppState;
pub use board_view::{BoardToken, BoardView, Mover, Place};
pub use game_page::{Choice, CityQuestion, GamePage, RouteLine};
pub use router::build_router;
pub use screen::Screen;
