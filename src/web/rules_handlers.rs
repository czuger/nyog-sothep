use askama::Template;
use axum::response::Html;

use crate::error::AppError;

#[derive(Template)]
#[template(path = "rules.html")]
struct RulesPage;

/// `GET /rules`: the original French rules, as scanned from the magazine.
pub async fn show_rules() -> Result<Html<String>, AppError> {
    Ok(Html(RulesPage.render()?))
}
