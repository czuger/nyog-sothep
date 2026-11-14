//! The human's answers to the game's prompts: parse the form, call the service, then redirect
//! to the game page, or show it again with the error.

use axum::Form;
use axum::extract::{Path, State};
use axum::response::Response;

use crate::domain::turn::{Answer, MoveOrder};
use crate::domain::{EventTable, FightChoice};
use crate::error::AppError;
use crate::web::app_state::AppState;
use crate::web::form_fields::{Fields, field, location, locations, respond};
use crate::web::relative_root::RelativeRoot;

/// `POST …/moves`: one `move_{index}` field per investigator.
pub async fn moves(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = parse_moves(&fields).map(Answer::Moves);
    respond(&state, game_id, &token, &root, answer).await
}

/// `POST …/play-again`: the `move_{index}` field of the investigator playing again.
pub async fn play_again(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = parse_moves(&fields).and_then(|orders| match orders.as_slice() {
        [(_, order)] => Ok(Answer::PlayAgain(*order)),
        _ => Err(AppError::InvalidForm(
            "un seul déplacement est attendu".to_string(),
        )),
    });
    respond(&state, game_id, &token, &root, answer).await
}

/// `POST …/table`: `table` is 2 or 3.
pub async fn table(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = field(&fields, "table")
        .and_then(EventTable::from_number)
        .map(Answer::Table)
        .ok_or_else(|| AppError::InvalidForm("table inconnue".to_string()));
    respond(&state, game_id, &token, &root, answer).await
}

/// `POST …/cities`: one `city` field per named city, or `skip` to decline.
pub async fn cities(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = match field(&fields, "skip") {
        Some(_) => Ok(Answer::Cities(Vec::new())),
        None => locations(&fields, "city").map(Answer::Cities),
    };
    respond(&state, game_id, &token, &root, answer).await
}

/// `POST …/fight`: `choice` is a [`FightChoice`] code.
pub async fn fight(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = field(&fields, "choice")
        .and_then(FightChoice::from_code)
        .map(Answer::Fight)
        .ok_or_else(|| AppError::InvalidForm("choix de combat inconnu".to_string()));
    respond(&state, game_id, &token, &root, answer).await
}

/// `POST …/banishment`: the `city` where Nyog'Sothep goes.
pub async fn banishment(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = locations(&fields, "city").and_then(|cities| match cities.as_slice() {
        [city] => Ok(Answer::BanishmentCity(*city)),
        _ => Err(AppError::InvalidForm("choisissez une ville".to_string())),
    });
    respond(&state, game_id, &token, &root, answer).await
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
                code => MoveOrder::Go(location(code)?),
            };
            Ok((index, order))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::http::header::LOCATION;

    use super::*;
    use crate::db::Store;
    use crate::domain::LocationId;
    use crate::domain::turn::{RuleSet, Side};
    use crate::service::create_game;
    use crate::web::form_fields::fields;

    async fn body_of(response: Response) -> String {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn an_answer_redirects_and_a_refusal_shows_the_page_again() {
        let folder = tempfile::tempdir().unwrap();
        let state = AppState {
            store: Store::open(folder.path()).unwrap(),
        };
        let game = create_game(&state.store, Side::Investigators, RuleSet::Original)
            .await
            .unwrap();
        let path = || Path((game.game_id, game.token.clone()));
        let root = || RelativeRoot::of_path("/games/1/token/moves");
        let wrong = fields(&[("move_0", "boston")]);
        let response = moves(State(state.clone()), path(), root(), Form(wrong))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let html = body_of(response).await;
        assert!(html.contains("Déplacement impossible pour"), "{html}");
        assert!(html.contains(r#"data-mover="move_0""#));
        assert!(html.contains(r#"href="../../../static/style.css""#));

        let sail: Fields = (0..4)
            .map(|index| (format!("move_{index}"), "nantucket_sound".to_string()))
            .collect();
        let response = moves(State(state.clone()), path(), root(), Form(sail))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let game_page = format!("../../../games/{}/{}", game.game_id, game.token);
        assert_eq!(response.headers()[LOCATION], game_page.as_str());

        let response = table(
            State(state),
            path(),
            root(),
            Form(fields(&[("table", "4")])),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(body_of(response).await.contains("table inconnue"));
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
}
