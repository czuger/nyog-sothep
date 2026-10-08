use std::path::Path;

use axum::Router;
use axum::http::HeaderValue;
use axum::http::header::CACHE_CONTROL;
use axum::routing::{get, post};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::web::app_state::AppState;
use crate::web::{answer_handlers, game_handlers, lobby_handlers};

/// Builds every route of the application.
///
/// # Arguments
/// * `asset_root` - directory holding the `img/` and `static/` folders.
pub fn build_router(state: AppState, asset_root: &Path) -> Router {
    let game_path = "/games/{game_id}/{token}";
    Router::new()
        .route("/", get(lobby_handlers::show_lobby))
        .route("/games", post(lobby_handlers::create_game))
        .route(game_path, get(game_handlers::show_game))
        .route(&format!("{game_path}/moves"), post(answer_handlers::moves))
        .route(&format!("{game_path}/table"), post(answer_handlers::table))
        .route(
            &format!("{game_path}/cities"),
            post(answer_handlers::cities),
        )
        .route(
            &format!("{game_path}/play-again"),
            post(answer_handlers::play_again),
        )
        .route(&format!("{game_path}/fight"), post(answer_handlers::fight))
        .route(
            &format!("{game_path}/banishment"),
            post(answer_handlers::banishment),
        )
        .nest_service("/img", ServeDir::new(asset_root.join("img")))
        .nest_service("/static", ServeDir::new(asset_root.join("static")))
        // Assets keep stable URLs, so browsers must revalidate them (a 304 when unchanged) to see edits.
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .with_state(state)
}
