//! The professor's answers to his prompts: parse the form, call the service, then redirect to
//! the game page, or show it again with the error.

use axum::Form;
use axum::extract::{Path, State};
use axum::response::Response;

use crate::domain::CounterKind;
use crate::domain::turn::{Answer, ProfessorAnswer, Teleport};
use crate::error::AppError;
use crate::web::app_state::AppState;
use crate::web::form_fields::{Fields, field, location, locations, one_location, respond};
use crate::web::relative_root::RelativeRoot;

/// `POST …/setup`: his start `city` (picked on the map) and `nyog_city`.
pub async fn setup(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = one_location(&fields, "city").and_then(|start| {
        let nyog_city = one_location(&fields, "nyog_city")?;
        Ok(ProfessorAnswer::Setup { start, nyog_city })
    });
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/teleport`: `teleport` is `{investigator}:{city}`, or `skip` declines.
pub async fn teleport(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = match field(&fields, "skip") {
        Some(_) => Ok(ProfessorAnswer::Teleport(None)),
        None => parse_teleport(field(&fields, "teleport").unwrap_or_default())
            .map(|teleport| ProfessorAnswer::Teleport(Some(teleport))),
    };
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/move`: his `destination`.
pub async fn movement(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = one_location(&fields, "destination").map(ProfessorAnswer::Location);
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/attack`: `target` is an investigator's index, or `none`.
pub async fn attack(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = match field(&fields, "target") {
        Some("none") => Ok(ProfessorAnswer::Attack(None)),
        target => target
            .and_then(|index| index.parse::<usize>().ok())
            .map(|index| ProfessorAnswer::Attack(Some(index)))
            .ok_or_else(|| AppError::InvalidForm("investigateur inconnu".to_string())),
    };
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/location`: the `city` picked on the map (a carry, a retreat or Nyog'Sothep's step).
pub async fn pick_location(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = one_location(&fields, "city").map(ProfessorAnswer::Location);
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/summon`: `summon` is `yes` or `no`.
pub async fn summon(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = Ok(ProfessorAnswer::Summon(
        field(&fields, "summon") == Some("yes"),
    ));
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/place`: the `counter` code, or `none`.
pub async fn place(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = match field(&fields, "counter") {
        Some("none") => Ok(ProfessorAnswer::Placement(None)),
        _ => counter(&fields).map(|kind| ProfessorAnswer::Placement(Some(kind))),
    };
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/counter`: the `counter` kept from Table 1, or discarded.
pub async fn pick_counter(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = counter(&fields).map(ProfessorAnswer::Counter);
    send(&state, game_id, &token, &root, answer).await
}

/// `POST …/name`: one `city` field per named city.
pub async fn name(
    State(state): State<AppState>,
    Path((game_id, token)): Path<(i64, String)>,
    root: RelativeRoot,
    Form(fields): Form<Fields>,
) -> Result<Response, AppError> {
    let answer = locations(&fields, "city").map(ProfessorAnswer::Cities);
    send(&state, game_id, &token, &root, answer).await
}

async fn send(
    state: &AppState,
    game_id: i64,
    token: &str,
    root: &RelativeRoot,
    answer: Result<ProfessorAnswer, AppError>,
) -> Result<Response, AppError> {
    respond(state, game_id, token, root, answer.map(Answer::Professor)).await
}

fn counter(fields: &Fields) -> Result<CounterKind, AppError> {
    field(fields, "counter")
        .and_then(CounterKind::from_code)
        .ok_or_else(|| AppError::InvalidForm("pion inconnu".to_string()))
}

/// Reads `{investigator}:{city}`.
fn parse_teleport(value: &str) -> Result<Teleport, AppError> {
    let invalid = || AppError::InvalidForm("téléportation inconnue".to_string());
    let (index, code) = value.split_once(':').ok_or_else(invalid)?;
    Ok(Teleport {
        investigator: index.parse().map_err(|_| invalid())?,
        destination: location(code)?,
    })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;
    use crate::db::Store;
    use crate::domain::LocationId;
    use crate::domain::turn::{RuleSet, Side};
    use crate::service::{GameView, create_game};
    use crate::web::form_fields::fields;
    use crate::web::{GamePage, Screen};

    #[test]
    fn a_teleport_names_the_investigator_and_the_city() {
        let teleport = parse_teleport("2:lynn").unwrap();
        assert_eq!(teleport.investigator, 2);
        assert_eq!(teleport.destination, LocationId::Lynn);
        assert!(parse_teleport("lynn").is_err());
    }

    #[tokio::test]
    async fn the_professor_sets_up_then_plays_on() {
        let folder = tempfile::tempdir().unwrap();
        let state = AppState {
            store: Store::open(folder.path()).unwrap(),
        };
        let game = create_game(&state.store, Side::Professor, RuleSet::Original)
            .await
            .unwrap();
        let path = || Path((game.game_id, game.token.clone()));
        let root = || RelativeRoot::of_path("/games/1/token/setup");
        let view = GameView::load(&state.store, game.game_id, &game.token)
            .await
            .unwrap();
        assert_eq!(GamePage::new(view, None, root()).screen, Screen::Setup);

        let sea = fields(&[("city", "nantucket_sound"), ("nyog_city", "boston")]);
        let response = setup(State(state.clone()), path(), root(), Form(sea))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let chosen = fields(&[("city", "worcester"), ("nyog_city", "concord")]);
        let response = setup(State(state.clone()), path(), root(), Form(chosen))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let view = GameView::load(&state.store, game.game_id, &game.token)
            .await
            .unwrap();
        assert_ne!(GamePage::new(view, None, root()).screen, Screen::Setup);
    }
}
