use std::path::Path;

use axum::Router;
use axum::http::HeaderValue;
use axum::http::header::CACHE_CONTROL;
use axum::routing::{get, post};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::web::app_state::AppState;
use crate::web::{
    answer_handlers, game_handlers, lobby_handlers, professor_handlers, replay_handlers,
    rules_handlers,
};

/// The private URL of a game.
const GAME_PATH: &str = "/games/{game_id}/{token}";

/// Builds every route of the application.
///
/// # Arguments
/// * `asset_root` - directory holding the `img/` and `static/` folders.
pub fn build_router(state: AppState, asset_root: &Path) -> Router {
    Router::new()
        .route("/", get(lobby_handlers::show_lobby))
        .route("/games", post(lobby_handlers::create_game))
        .route("/rules", get(rules_handlers::show_rules))
        .route(GAME_PATH, get(game_handlers::show_game))
        .route(
            &format!("{GAME_PATH}/replay"),
            get(replay_handlers::show_replay),
        )
        .merge(investigator_routes())
        .merge(professor_routes())
        .nest_service("/img", ServeDir::new(asset_root.join("img")))
        .nest_service("/static", ServeDir::new(asset_root.join("static")))
        // Assets keep stable URLs, so browsers must revalidate them (a 304 when unchanged) to see edits.
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .with_state(state)
}

/// One `POST` per prompt of the investigators.
fn investigator_routes() -> Router<AppState> {
    Router::new()
        .route(&prompt_path("moves"), post(answer_handlers::moves))
        .route(&prompt_path("table"), post(answer_handlers::table))
        .route(&prompt_path("cities"), post(answer_handlers::cities))
        .route(
            &prompt_path("play-again"),
            post(answer_handlers::play_again),
        )
        .route(&prompt_path("fight"), post(answer_handlers::fight))
        .route(
            &prompt_path("banishment"),
            post(answer_handlers::banishment),
        )
}

/// One `POST` per kind of answer of the professor.
fn professor_routes() -> Router<AppState> {
    Router::new()
        .route(&prompt_path("setup"), post(professor_handlers::setup))
        .route(&prompt_path("teleport"), post(professor_handlers::teleport))
        .route(&prompt_path("move"), post(professor_handlers::movement))
        .route(&prompt_path("attack"), post(professor_handlers::attack))
        .route(
            &prompt_path("location"),
            post(professor_handlers::pick_location),
        )
        .route(&prompt_path("summon"), post(professor_handlers::summon))
        .route(&prompt_path("place"), post(professor_handlers::place))
        .route(
            &prompt_path("counter"),
            post(professor_handlers::pick_counter),
        )
        .route(&prompt_path("name"), post(professor_handlers::name))
}

fn prompt_path(prompt: &str) -> String {
    format!("{GAME_PATH}/{prompt}")
}
