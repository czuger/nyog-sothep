//! HTTP layer: routes, handlers and pages.

mod answer_handlers;
mod app_state;
mod board_view;
mod embedded_assets;
mod error_page;
mod form_fields;
mod game_handlers;
mod game_page;
mod lobby_handlers;
mod professor_handlers;
mod professor_screens;
mod relative_root;
mod replay_handlers;
mod replay_page;
mod router;
mod rules_handlers;
mod screen;

pub use app_state::AppState;
pub use board_view::{BoardToken, BoardView, Mover, Place};
pub use game_page::{Choice, CityQuestion, CounterChoice, GamePage, RouteLine};
pub use relative_root::RelativeRoot;
pub use replay_page::ReplayPage;
pub use router::build_router;
pub use screen::Screen;
