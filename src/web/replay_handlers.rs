use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::AppError;
use crate::service::ReplayView;
use crate::web::app_state::AppState;
use crate::web::relative_root::RelativeRoot;
use crate::web::replay_page::ReplayPage;

/// `GET /games/{game_id}/{token}/replay`: the replay of a finished game.
pub async fn show_replay(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
) -> Result<Html<String>, AppError> {
    let view = ReplayView::load(&state.store, game_id, &token).await?;
    Ok(Html(ReplayPage::new(view, root)?.render()?))
}
