//! The human's answers to the game's prompts: parse the form, call the service, then redirect
//! to the game page, or show it again with the error.

use askama::Template;
use axum::Form;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};

use crate::domain::turn::{Answer, MoveOrder};
use crate::domain::{EventTable, FightChoice, LocationId};
use crate::error::AppError;
use crate::service::{GameView, answer_prompt};
use crate::web::app_state::AppState;
use crate::web::game_page::GamePage;

/// A submitted form, as its name/value pairs (city lists repeat the same name).
type Fields = Vec<(String, String)>;

/// `POST …/moves`: one `move_{index}` field per investigator.
pub async fn moves(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = parse_moves(&fields).map(Answer::Moves);
    respond(&state, game_id, &token, answer).await
}

/// `POST …/play-again`: the `move_{index}` field of the investigator playing again.
pub async fn play_again(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = parse_moves(&fields).and_then(|orders| match orders.as_slice() {
        [(_, order)] => Ok(Answer::PlayAgain(*order)),
        _ => Err(AppError::InvalidForm(
            "un seul déplacement est attendu".to_string(),
        )),
    });
    respond(&state, game_id, &token, answer).await
}

/// `POST …/table`: `table` is 2 or 3.
pub async fn table(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = field(&fields, "table")
        .and_then(EventTable::from_number)
        .map(Answer::Table)
        .ok_or_else(|| AppError::InvalidForm("table inconnue".to_string()));
    respond(&state, game_id, &token, answer).await
}

/// `POST …/cities`: one `city` field per named city, or `skip` to decline.
pub async fn cities(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = match field(&fields, "skip") {
        Some(_) => Ok(Answer::Cities(Vec::new())),
        None => named_cities(&fields).map(Answer::Cities),
    };
    respond(&state, game_id, &token, answer).await
}

/// `POST …/fight`: `choice` is a [`FightChoice`] code.
pub async fn fight(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = field(&fields, "choice")
        .and_then(FightChoice::from_code)
        .map(Answer::Fight)
        .ok_or_else(|| AppError::InvalidForm("choix de combat inconnu".to_string()));
    respond(&state, game_id, &token, answer).await
}

/// `POST …/banishment`: the `city` where Nyog'Sothep goes.
pub async fn banishment(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = named_cities(&fields).and_then(|cities| match cities.as_slice() {
        [city] => Ok(Answer::BanishmentCity(*city)),
        _ => Err(AppError::InvalidForm("choisissez une ville".to_string())),
    });
    respond(&state, game_id, &token, answer).await
}

/// Sends the answer, then redirects to the game page, or shows it again with the error.
async fn respond(
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

fn field<'a>(fields: &'a Fields, name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
}

fn named_cities(fields: &Fields) -> Result<Vec<LocationId>, AppError> {
    fields
        .iter()
        .filter(|(key, _)| key == "city")
        .map(|(_, code)| {
            LocationId::from_code(code)
                .ok_or_else(|| AppError::InvalidForm(format!("lieu inconnu « {code} »")))
        })
        .collect()
}

/// Reads the `move_{index}` fields.
fn parse_moves(fields: &Fields) -> Result<Vec<(usize, MoveOrder)>, AppError> {
    fields
        .iter()
        .filter_map(|(key, value)| {
            let index = key.strip_prefix("move_")?.parse::<usize>().ok()?;
            Some((index, value))
        })
        .map(|(index, value)| {
            let order = match value.as_str() {
                "stay" => MoveOrder::Stay,
                "psychoanalyst" => MoveOrder::Psychoanalyst,
                code => MoveOrder::Go(
                    LocationId::from_code(code)
                        .ok_or_else(|| AppError::InvalidForm(format!("lieu inconnu « {code} »")))?,
                ),
            };
            Ok((index, order))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;

    use super::*;
    use crate::db;
    use crate::service::create_game;

    async fn body_of(response: Response) -> String {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn an_answer_redirects_and_a_refusal_shows_the_page_again() {
        let pool = db::connect("sqlite::memory:").await.unwrap();
        let state = AppState { pool };
        let game = create_game(&state.pool).await.unwrap();
        let path = || Path((game.game_id, game.token.clone()));
        let wrong = fields(&[("move_0", "boston")]);
        let response = moves(State(state.clone()), path(), Form(wrong))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let html = body_of(response).await;
        assert!(html.contains("Déplacement impossible pour"), "{html}");
        assert!(html.contains(r#"data-mover="move_0""#));

        let sail: Fields = (0..4)
            .map(|index| (format!("move_{index}"), "nantucket_sound".to_string()))
            .collect();
        let response = moves(State(state.clone()), path(), Form(sail))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);

        let response = table(State(state), path(), Form(fields(&[("table", "4")])))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(body_of(response).await.contains("table inconnue"));
    }

    fn fields(pairs: &[(&str, &str)]) -> Fields {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    #[test]
    fn moves_are_read_by_investigator() {
        let parsed = parse_moves(&fields(&[
            ("move_0", "stay"),
            ("move_2", "psychoanalyst"),
            ("move_1", "boston"),
            ("other", "x"),
        ]))
        .unwrap();
        assert_eq!(
            parsed,
            vec![
                (0, MoveOrder::Stay),
                (2, MoveOrder::Psychoanalyst),
                (1, MoveOrder::Go(LocationId::Boston))
            ]
        );
        assert!(parse_moves(&fields(&[("move_0", "paris")])).is_err());
    }

    #[test]
    fn cities_repeat_the_same_field() {
        let named = named_cities(&fields(&[("city", "lynn"), ("city", "arkham")])).unwrap();
        assert_eq!(named, vec![LocationId::Lynn, LocationId::Arkham]);
    }
}
