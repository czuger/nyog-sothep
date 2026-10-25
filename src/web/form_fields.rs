//! Reading the submitted forms, and the common answer flow of the prompt handlers.

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};

use crate::domain::LocationId;
use crate::domain::turn::Answer;
use crate::error::AppError;
use crate::service::{GameView, answer_prompt};
use crate::web::app_state::AppState;
use crate::web::game_page::GamePage;

/// A submitted form, as its name/value pairs (city lists repeat the same name).
pub type Fields = Vec<(String, String)>;

/// Sends the answer, then redirects to the game page, or shows it again with the error.
pub async fn respond(
    state: &AppState,
    game_id: i64,
    token: &str,
    answer: Result<Answer, AppError>,
) -> Result<Response, AppError> {
    let result = match answer {
        Ok(answer) => answer_prompt(&state.pool, game_id, token, answer).await,
        Err(error) => Err(error),
    };
    match result {
        Ok(()) => Ok(Redirect::to(&format!("/games/{game_id}/{token}")).into_response()),
        Err(error) if !error.is_internal() && !matches!(error, AppError::GameNotFound) => {
            let view = GameView::load(&state.pool, game_id, token).await?;
            let page = GamePage::new(view, Some(error.to_string()));
            Ok((StatusCode::BAD_REQUEST, Html(page.render()?)).into_response())
        }
        Err(error) => Err(error),
    }
}

/// The value of a field, if sent.
pub fn field<'a>(fields: &'a Fields, name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
}

/// Every location named in a field, in order.
pub fn locations(fields: &Fields, name: &str) -> Result<Vec<LocationId>, AppError> {
    fields
        .iter()
        .filter(|(key, _)| key == name)
        .map(|(_, code)| location(code))
        .collect()
}

/// The location of a code.
pub fn location(code: &str) -> Result<LocationId, AppError> {
    LocationId::from_code(code)
        .ok_or_else(|| AppError::InvalidForm(format!("lieu inconnu « {code} »")))
}

/// The single location of a field.
pub fn one_location(fields: &Fields, name: &str) -> Result<LocationId, AppError> {
    match locations(fields, name)?.as_slice() {
        [one] => Ok(*one),
        _ => Err(AppError::InvalidForm("choisissez un lieu".to_string())),
    }
}

#[cfg(test)]
pub fn fields(pairs: &[(&str, &str)]) -> Fields {
    pairs
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cities_repeat_the_same_field() {
        let named = locations(&fields(&[("city", "lynn"), ("city", "arkham")]), "city").unwrap();
        assert_eq!(named, vec![LocationId::Lynn, LocationId::Arkham]);
        assert!(one_location(&fields(&[("city", "lynn"), ("city", "arkham")]), "city").is_err());
    }
}
