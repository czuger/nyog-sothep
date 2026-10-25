//! The replay of a finished game: every recorded event, secrets included, and only once over.

use askama::Template;
use sqlx::SqlitePool;

use nyog_sothep::db::{self, GameRecord, load_state, save_state};
use nyog_sothep::domain::turn::{Answer, MoveOrder, Outcome, Phase, Side};
use nyog_sothep::error::AppError;
use nyog_sothep::service::{CreatedGame, ReplayView, answer_prompt, create_game};
use nyog_sothep::web::ReplayPage;

/// Ends a stored game as if the investigators had killed the professor.
async fn finish(pool: &SqlitePool, game: &CreatedGame) {
    let mut connection = pool.acquire().await.unwrap();
    let mut record = GameRecord::load(&mut connection, game.game_id, &game.token)
        .await
        .unwrap()
        .unwrap();
    let mut state = load_state(&mut connection, &record).await.unwrap();
    state.outcome = Some(Outcome::ProfessorKilled);
    state.phase = Phase::Finished;
    state.prompt = None;
    save_state(&mut connection, &mut record, &state)
        .await
        .unwrap();
}

#[tokio::test]
async fn a_finished_game_is_replayed_with_its_secrets() {
    let pool = db::connect("sqlite::memory:").await.unwrap();
    let game = create_game(&pool, Side::Investigators).await.unwrap();
    let stay = Answer::Moves((0..4).map(|index| (index, MoveOrder::Stay)).collect());
    answer_prompt(&pool, game.game_id, &game.token, stay)
        .await
        .unwrap();
    assert!(matches!(
        ReplayView::load(&pool, game.game_id, &game.token).await,
        Err(AppError::GameNotOver)
    ));
    finish(&pool, &game).await;

    let view = ReplayView::load(&pool, game.game_id, &game.token)
        .await
        .unwrap();
    assert!(view.steps[0].board.is_some());
    let texts: Vec<&str> = view
        .steps
        .iter()
        .filter_map(|step| step.text.as_deref())
        .collect();
    assert!(texts[0].starts_with("Mai 1921."));
    assert!(texts.iter().any(|text| text.contains("commence avec")));
    assert!(
        texts
            .iter()
            .any(|text| text.starts_with("Le professeur part en secret"))
    );
    assert!(texts.iter().any(|text| text.contains("reste à Nantucket")));
    assert!(view.steps.iter().any(|step| !step.dice.is_empty()));

    let page = ReplayPage::new(view).unwrap();
    assert!(
        !page.data.contains('<'),
        "the JSON could close its script element"
    );
    let html = page.render().unwrap();
    assert!(html.contains(r#"<script type="application/json" id="replay-data">"#));
    assert!(html.contains("/static/replay.js"));
    assert!(html.contains("Victoire"));
    assert!(html.contains(&format!(r#"href="{}">Retour à la partie"#, game.path())));
}

#[tokio::test]
async fn a_replay_needs_the_games_token() {
    let pool = db::connect("sqlite::memory:").await.unwrap();
    let game = create_game(&pool, Side::Professor).await.unwrap();
    assert!(matches!(
        ReplayView::load(&pool, game.game_id, "wrong").await,
        Err(AppError::GameNotFound)
    ));
}
