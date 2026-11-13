//! Games played through the services, on a PoloDB database in a temporary folder.

use tempfile::TempDir;

use nyog_sothep::db::Store;
use nyog_sothep::domain::LocationId;
use nyog_sothep::domain::turn::{
    Answer, AnswerError, MoveOrder, ProfessorAnswer, ProfessorPrompt, Prompt, Side,
};
use nyog_sothep::error::AppError;
use nyog_sothep::service::{CreatedGame, GameView, ProfessorSighting, answer_prompt, create_game};

/// A store in a temporary folder, removed when the test drops it.
struct TestStore {
    store: Store,
    _folder: TempDir,
}

impl std::ops::Deref for TestStore {
    type Target = Store;

    fn deref(&self) -> &Store {
        &self.store
    }
}

async fn new_game() -> (TestStore, CreatedGame) {
    new_game_as(Side::Investigators).await
}

async fn new_game_as(human: Side) -> (TestStore, CreatedGame) {
    let folder = tempfile::tempdir().unwrap();
    let store = Store::open(folder.path()).unwrap();
    let game = create_game(&store, human).await.unwrap();
    (
        TestStore {
            store,
            _folder: folder,
        },
        game,
    )
}

async fn view(store: &Store, game: &CreatedGame) -> GameView {
    GameView::load(store, game.game_id, &game.token)
        .await
        .unwrap()
}

fn everyone(order: MoveOrder) -> Answer {
    Answer::Moves((0..4).map(|index| (index, order)).collect())
}

#[tokio::test]
async fn a_new_game_waits_for_the_investigators_in_nantucket() {
    let (store, game) = new_game().await;
    let view = view(&store, &game).await;
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
    assert!(game.path().starts_with(&format!("games/{}/", game.game_id)));
}

#[tokio::test]
async fn sailing_away_plays_the_next_round() {
    let (store, game) = new_game().await;
    let before = view(&store, &game).await;
    answer_prompt(
        &store,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::NantucketSound)),
    )
    .await
    .unwrap();
    let after = view(&store, &game).await;
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
    let (store, game) = new_game().await;
    let before = view(&store, &game).await;
    let refused = answer_prompt(
        &store,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::Boston)),
    )
    .await;
    assert!(matches!(
        refused,
        Err(AppError::Answer(AnswerError::IllegalMove(_)))
    ));
    let wrong_kind = answer_prompt(&store, game.game_id, &game.token, Answer::Cities(vec![])).await;
    assert!(matches!(
        wrong_kind,
        Err(AppError::Answer(AnswerError::Unexpected))
    ));
    assert_eq!(view(&store, &game).await, before);
}

#[tokio::test]
async fn a_wrong_token_finds_no_game() {
    let (store, game) = new_game().await;
    let loaded = GameView::load(&store, game.game_id, "not-the-token").await;
    assert!(matches!(loaded, Err(AppError::GameNotFound)));
    let answered = answer_prompt(&store, game.game_id, "nope", everyone(MoveOrder::Stay)).await;
    assert!(matches!(answered, Err(AppError::GameNotFound)));
}

#[tokio::test]
async fn games_can_be_played_for_many_rounds() {
    let (store, game) = new_game().await;
    answer_prompt(
        &store,
        game.game_id,
        &game.token,
        everyone(MoveOrder::Go(LocationId::NantucketSound)),
    )
    .await
    .unwrap();
    for _ in 0..5 {
        let current = view(&store, &game).await;
        if current.prompt != Some(Prompt::Moves) {
            break;
        }
        let orders = current
            .investigators
            .iter()
            .filter(|investigator| investigator.moves.is_some())
            .map(|investigator| (investigator.index, MoveOrder::Stay))
            .collect();
        answer_prompt(&store, game.game_id, &game.token, Answer::Moves(orders))
            .await
            .unwrap();
    }
    assert!(view(&store, &game).await.turn >= 3);
}

#[tokio::test]
async fn the_professor_plays_against_the_computer_investigators() {
    let (store, game) = new_game_as(Side::Professor).await;
    let first = view(&store, &game).await;
    assert_eq!(first.human, Side::Professor);
    assert_eq!(
        first.prompt,
        Some(Prompt::Professor(ProfessorPrompt::Setup))
    );
    let setup = ProfessorAnswer::Setup {
        start: LocationId::Providence,
        nyog_city: LocationId::Boston,
    };
    answer_prompt(&store, game.game_id, &game.token, Answer::Professor(setup))
        .await
        .unwrap();
    // The professor answers every prompt with the most passive choice; the computer's
    // investigators play everything else.
    for _ in 0..40 {
        let current = view(&store, &game).await;
        let Some(Prompt::Professor(prompt)) = current.prompt else {
            break;
        };
        let answer = match prompt {
            ProfessorPrompt::Movement { options } => ProfessorAnswer::Location(options.steps[0]),
            ProfessorPrompt::Table1 { options } => ProfessorAnswer::Counter(options[0]),
            ProfessorPrompt::Discard { hand } => ProfessorAnswer::Counter(hand[0]),
            ProfessorPrompt::Placement { .. } => ProfessorAnswer::Placement(None),
            ProfessorPrompt::Teleport { .. } => ProfessorAnswer::Teleport(None),
            ProfessorPrompt::Attack { candidates, forced } => {
                ProfessorAnswer::Attack(forced.then(|| candidates[0]))
            }
            ProfessorPrompt::Carry { options, .. }
            | ProfessorPrompt::Relocate { options, .. }
            | ProfessorPrompt::NyogStep { options } => ProfessorAnswer::Location(options[0]),
            ProfessorPrompt::Summon => ProfessorAnswer::Summon(false),
            ProfessorPrompt::NameCities { count } => {
                let ProfessorSighting::Visible { at } = current.professor else {
                    panic!("the professor sees himself");
                };
                let mut cities = vec![at];
                cities.extend(
                    [LocationId::Boston, LocationId::Lynn, LocationId::Arkham]
                        .into_iter()
                        .filter(|city| *city != at)
                        .take(count - 1),
                );
                ProfessorAnswer::Cities(cities)
            }
            other => panic!("unexpected prompt {other:?}"),
        };
        answer_prompt(&store, game.game_id, &game.token, Answer::Professor(answer))
            .await
            .unwrap();
    }
    let after = view(&store, &game).await;
    assert!(after.turn >= 3 || after.is_finished());
    assert!(matches!(after.professor, ProfessorSighting::Visible { .. }));
    assert_eq!(after.nyog_city, Some(LocationId::Boston));
    assert!(!after.route_sheet.is_empty());
}
