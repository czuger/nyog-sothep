use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::AppError;
use crate::service::GameView;
use crate::web::app_state::AppState;
use crate::web::game_page::GamePage;

/// `GET /games/{game_id}/{token}`: the game page.
pub async fn show_game(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
) -> Result<Html<String>, AppError> {
    let view = GameView::load(&state.pool, game_id, &token).await?;
    Ok(Html(GamePage::new(view, None).render()?))
}
