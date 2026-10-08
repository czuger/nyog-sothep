use askama::Template;
use axum::extract::State;
use axum::response::{Html, Redirect};

use crate::error::AppError;
use crate::service::create_game as create_game_service;
use crate::web::app_state::AppState;

#[derive(Template)]
#[template(path = "lobby.html")]
struct LobbyPage;

/// `GET /`: the game creation page.
pub async fn show_lobby() -> Result<Html<String>, AppError> {
    Ok(Html(LobbyPage.render()?))
}

/// `POST /games`: creates a game and sends the human to its private URL.
pub async fn create_game(State(state): State<AppState>) -> Result<Redirect, AppError> {
    let game = create_game_service(&state.pool).await?;
    Ok(Redirect::to(&game.path()))
}
