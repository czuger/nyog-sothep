//! Loading and saving a whole [`GameState`] across its tables.

use sqlx::SqliteConnection;

use crate::db::clue_record::ClueRecord;
use crate::db::counter_record::CounterRecord;
use crate::db::destroyed_city_record::DestroyedCityRecord;
use crate::db::event_record::EventRecord;
use crate::db::game_record::GameRecord;
use crate::db::investigator_record::InvestigatorRecord;
use crate::db::professor_record::ProfessorRecord;
use crate::db::replay_step_record::ReplayStepRecord;
use crate::db::route_stage_record::RouteStageRecord;
use crate::domain::turn::{GameState, History};
use crate::error::AppError;

/// Rebuilds the state of a game from its rows.
pub async fn load_state(
    connection: &mut SqliteConnection,
    game: &GameRecord,
) -> Result<GameState, AppError> {
    Ok(GameState {
        human: game.human()?,
        turn: game.turn(),
        phase: game.phase()?,
        prompt: game.prompt()?,
        outcome: game.outcome()?,
        professor: ProfessorRecord::load(connection, game.id).await?,
        investigators: InvestigatorRecord::load_all(connection, game.id).await?,
        counters: CounterRecord::load_all(connection, game.id).await?,
        nyog: game.nyog()?,
        destroyed: DestroyedCityRecord::load_all(connection, game.id).await?,
        route_sheet: RouteStageRecord::load_all(connection, game.id).await?,
        clues: ClueRecord::load_all(connection, game.id).await?,
        sea_announcements: game.sea_announcements(),
        move_in_the_open: game.move_in_the_open(),
    })
}

/// Writes the state of a game to its rows and bumps the game's version.
pub async fn save_state(
    connection: &mut SqliteConnection,
    game: &mut GameRecord,
    state: &GameState,
) -> Result<(), AppError> {
    game.save(connection, state).await?;
    ProfessorRecord::save(connection, game.id, &state.professor).await?;
    InvestigatorRecord::replace_all(connection, game.id, &state.investigators).await?;
    CounterRecord::replace_all(connection, game.id, &state.counters).await?;
    DestroyedCityRecord::append_new(connection, game.id, &state.destroyed).await?;
    RouteStageRecord::append_new(connection, game.id, &state.route_sheet).await?;
    ClueRecord::append_new(connection, game.id, &state.clues).await?;
    Ok(())
}

/// Appends what an engine wrote to the game's log and replay.
pub async fn append_history(
    connection: &mut SqliteConnection,
    game_id: i64,
    history: &History,
) -> Result<(), AppError> {
    EventRecord::insert_all(connection, game_id, &history.log).await?;
    ReplayStepRecord::insert_all(connection, game_id, &history.replay).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;
    use crate::domain::investigator_ai::InvestigatorsAi;
    use crate::domain::professor_ai::PetersenAi;
    use crate::domain::turn::{
        Answer, Engine, MoveOrder, ProfessorAnswer, ProfessorPrompt, Prompt, Side,
    };
    use crate::domain::{InvestigatorId, LocationId, RandomDice};

    const IDS: [InvestigatorId; 4] = [
        InvestigatorId::Armitage,
        InvestigatorId::Malone,
        InvestigatorId::Johansen,
        InvestigatorId::Walker,
    ];

    /// Stores a new game and checks that it reads back unchanged.
    async fn store(connection: &mut SqliteConnection, state: &GameState) -> GameRecord {
        let game_id = GameRecord::insert(connection, "token", state)
            .await
            .unwrap();
        let mut game = GameRecord::load(connection, game_id, "token")
            .await
            .unwrap()
            .unwrap();
        save_state(connection, &mut game, state).await.unwrap();
        assert_eq!(&load_state(connection, &game).await.unwrap(), state);
        game
    }

    #[tokio::test]
    async fn a_state_survives_a_round_trip() {
        let pool = connect("sqlite::memory:").await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let mut dice = RandomDice::seeded(11);
        let mut ai = PetersenAi::seeded(11);
        let mut investigators = InvestigatorsAi::seeded(11);
        let (mut state, _) = Engine::new_game(
            &IDS,
            Side::Investigators,
            &mut dice,
            &mut ai,
            &mut investigators,
        );
        let mut game = store(&mut connection, &state).await;
        let game_id = game.id;

        let sail = (0..4)
            .map(|index| (index, MoveOrder::Go(LocationId::NantucketSound)))
            .collect();
        Engine::new(&mut state, &mut dice, &mut ai, &mut investigators)
            .answer(Answer::Moves(sail))
            .unwrap();
        state.destroyed.push(LocationId::Boston);
        save_state(&mut connection, &mut game, &state)
            .await
            .unwrap();
        let game = GameRecord::load(&mut connection, game_id, "token")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(game.version, 3);
        assert_eq!(load_state(&mut connection, &game).await.unwrap(), state);
        assert!(
            GameRecord::load(&mut connection, game_id, "wrong")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn a_professor_game_survives_a_round_trip() {
        let pool = connect("sqlite::memory:").await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let mut dice = RandomDice::seeded(3);
        let mut ai = PetersenAi::seeded(3);
        let mut investigators = InvestigatorsAi::seeded(3);
        let (mut state, _) = Engine::new_game(
            &IDS,
            Side::Professor,
            &mut dice,
            &mut ai,
            &mut investigators,
        );
        assert_eq!(
            state.prompt,
            Some(Prompt::Professor(ProfessorPrompt::Setup))
        );
        store(&mut connection, &state).await;

        let setup = ProfessorAnswer::Setup {
            start: LocationId::Worcester,
            nyog_city: LocationId::Concord,
        };
        Engine::new(&mut state, &mut dice, &mut ai, &mut investigators)
            .answer(Answer::Professor(setup))
            .unwrap();
        assert_eq!(state.human, Side::Professor);
        assert!(matches!(state.prompt, Some(Prompt::Professor(_))));
        store(&mut connection, &state).await;
    }
}
