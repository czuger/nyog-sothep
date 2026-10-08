//! Games played through the services, on in-memory SQLite.

use sqlx::SqlitePool;

use nyog_sothep::db;
use nyog_sothep::domain::LocationId;
use nyog_sothep::domain::turn::{Answer, AnswerError, MoveOrder, Prompt};
use nyog_sothep::error::AppError;
use nyog_sothep::service::{CreatedGame, GameView, answer_prompt, create_game};

async fn new_game() -> (SqlitePool, CreatedGame) {
    let pool = db::connect("sqlite::memory:").await.unwrap();
    let game = create_game(&pool).await.unwrap();
    (pool, game)
}

async fn view(pool: &SqlitePool, game: &CreatedGame) -> GameView {
    GameView::load(pool, game.game_id, &game.token)
        .await
        .unwrap()
}

fn everyone(order: MoveOrder) -> Answer {
    Answer::Moves((0..4).map(|index| (index, order)).collect())
}

#[tokio::test]
async fn a_new_game_waits_for_the_investigators_in_nantucket() {
    let (pool, game) = new_game().await;
    let view = view(&pool, &game).await;
    assert_eq!(view.turn, 1);
    assert_eq!(view.prompt, Some(Prompt::Moves));
    assert_eq!(view.investigators.len(), 4);
    for investigator in &view.investigators {
        assert_eq!(investigator.location, LocationId::Nantucket);
        assert!((6..=21).contains(&investigator.san));
        assert!(investigator.moves.is_some());
    }
    assert_eq!(view.professor_hp, 14);
    assert!(view.log.iter().any(|line| line.text.contains("Nantucket")));
    assert!(
        game.path()
            .starts_with(&format!("/games/{}/", game.game_id))
    );
}

#[tokio::test]
async fn sailing_away_plays_the_next_round() {
    let (pool, game) = new_game().await;
    let before = view(&pool, &game).await;
    answer_prompt(
        &pool,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::NantucketSound)),
    )
    .await
    .unwrap();
    let after = view(&pool, &game).await;
    assert_eq!(after.turn, 2);
    assert!(after.version > before.version);
    assert!(
        after
            .investigators
            .iter()
            .all(|i| i.location == LocationId::NantucketSound)
    );
    assert!(after.log.len() > before.log.len());
}

#[tokio::test]
async fn a_refused_answer_changes_nothing() {
    let (pool, game) = new_game().await;
    let before = view(&pool, &game).await;
    let refused = answer_prompt(
        &pool,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::Boston)),
    )
    .await;
    assert!(matches!(
        refused,
        Err(AppError::Answer(AnswerError::IllegalMove(_)))
    ));
    let wrong_kind = answer_prompt(&pool, game.game_id, &game.token, Answer::Cities(vec![])).await;
    assert!(matches!(
        wrong_kind,
        Err(AppError::Answer(AnswerError::Unexpected))
    ));
    assert_eq!(view(&pool, &game).await, before);
}

#[tokio::test]
async fn a_wrong_token_finds_no_game() {
    let (pool, game) = new_game().await;
    let loaded = GameView::load(&pool, game.game_id, "not-the-token").await;
    assert!(matches!(loaded, Err(AppError::GameNotFound)));
    let answered = answer_prompt(&pool, game.game_id, "nope", everyone(MoveOrder::Stay)).await;
    assert!(matches!(answered, Err(AppError::GameNotFound)));
}

#[tokio::test]
async fn games_can_be_played_for_many_rounds() {
    let (pool, game) = new_game().await;
    answer_prompt(
        &pool,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::NantucketSound)),
    )
    .await
    .unwrap();
    for _ in 0..5 {
        let current = view(&pool, &game).await;
        if current.prompt != Some(Prompt::Moves) {
            break;
        }
        let orders = current
            .investigators
            .iter()
            .filter(|investigator| investigator.moves.is_some())
            .map(|investigator| (investigator.index, MoveOrder::Stay))
            .collect();
        answer_prompt(&pool, game.game_id, &game.token, Answer::Moves(orders))
            .await
            .unwrap();
    }
    assert!(view(&pool, &game).await.turn >= 3);
}
