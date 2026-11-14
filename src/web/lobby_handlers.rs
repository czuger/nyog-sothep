use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::{Html, Redirect};

use crate::domain::turn::Side;
use crate::error::AppError;
use crate::service::create_game as create_game_service;
use crate::web::app_state::AppState;
use crate::web::form_fields::{Fields, field};
use crate::web::relative_root::RelativeRoot;

#[derive(Template)]
#[template(path = "lobby.html")]
struct LobbyPage {
    root: RelativeRoot,
}

/// `GET /`: the game creation page.
pub async fn show_lobby(root: RelativeRoot) -> Result<Html<String>, AppError> {
    Ok(Html(LobbyPage { root }.render()?))
}

/// `POST /games`: creates a game where the human plays the `side` field (the investigators
/// when absent) and sends them to its private URL.
pub async fn create_game(
    State(state): State<AppState>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Redirect, AppError> {
    let human = match field(&fields, "side") {
        Some(code) => Side::from_code(code)
            .ok_or_else(|| AppError::InvalidForm(format!("camp inconnu « {code} »")))?,
        None => Side::Investigators,
    };
    let game = create_game_service(&state.store, human).await?;
    Ok(Redirect::to(&format!("{root}{}", game.path())))
}
