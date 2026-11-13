//! The replay of a finished game: every recorded event, secrets included, and only once over.

use askama::Template;
use tempfile::TempDir;

use nyog_sothep::db::{GameDocument, Store};
use nyog_sothep::domain::turn::{Answer, MoveOrder, Outcome, Phase, Side};
use nyog_sothep::error::AppError;
use nyog_sothep::service::{CreatedGame, ReplayView, answer_prompt, create_game};
use nyog_sothep::web::{RelativeRoot, ReplayPage};

/// An empty store, and the folder holding it.
fn open_store() -> (TempDir, Store) {
    let folder = tempfile::tempdir().unwrap();
    let store = Store::open(folder.path()).unwrap();
    (folder, store)
}

/// Ends a stored game as if the investigators had killed the professor.
fn finish(store: &Store, game: &CreatedGame) {
    store
        .write(|transaction| {
            let mut stored = GameDocument::find(transaction, game.game_id, &game.token)?.unwrap();
            stored.state.outcome = Some(Outcome::ProfessorKilled);
            stored.state.phase = Phase::Finished;
            stored.state.prompt = None;
            stored.save(transaction)
        })
        .unwrap();
}

#[tokio::test]
async fn a_finished_game_is_replayed_with_its_secrets() {
    let (_folder, store) = open_store();
    let game = create_game(&store, Side::Investigators).await.unwrap();
    let stay = Answer::Moves((0..4).map(|index| (index, MoveOrder::Stay)).collect());
    answer_prompt(&store, game.game_id, &game.token, stay)
        .await
        .unwrap();
    assert!(matches!(
        ReplayView::load(&store, game.game_id, &game.token).await,
        Err(AppError::GameNotOver)
    ));
    finish(&store, &game);

    let view = ReplayView::load(&store, game.game_id, &game.token)
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

    let page = ReplayPage::new(view, RelativeRoot::of_path("/games/1/token/replay")).unwrap();
    assert!(
        !page.data.contains('<'),
        "the JSON could close its script element"
    );
    let html = page.render().unwrap();
    assert!(html.contains(r#"<script type="application/json" id="replay-data">"#));
    assert!(html.contains(r#"src="../../../static/replay.js""#));
    assert!(html.contains("Victoire"));
    assert!(html.contains(&format!(
        r#"href="../../../{}">Retour à la partie"#,
        game.path()
    )));
}

#[tokio::test]
async fn a_replay_needs_the_games_token() {
    let (_folder, store) = open_store();
    let game = create_game(&store, Side::Professor).await.unwrap();
    assert!(matches!(
        ReplayView::load(&store, game.game_id, "wrong").await,
        Err(AppError::GameNotFound)
    ));
}
