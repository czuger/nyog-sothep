use askama::Template;
use axum::response::Html;

use crate::error::AppError;
use crate::web::relative_root::RelativeRoot;

#[derive(Template)]
#[template(path = "rules.html")]
struct RulesPage {
    root: RelativeRoot,
}

/// `GET /rules`: the original French rules, as scanned from the magazine.
pub async fn show_rules(root: RelativeRoot) -> Result<Html<String>, AppError> {
    Ok(Html(RulesPage { root }.render()?))
}
