//! Loading and saving a whole [`GameState`] across its tables.

use sqlx::SqliteConnection;

use crate::db::clue_record::ClueRecord;
use crate::db::counter_record::CounterRecord;
use crate::db::destroyed_city_record::DestroyedCityRecord;
use crate::db::game_record::GameRecord;
use crate::db::investigator_record::InvestigatorRecord;
use crate::db::professor_record::ProfessorRecord;
use crate::db::route_stage_record::RouteStageRecord;
use crate::domain::turn::GameState;
use crate::error::AppError;

/// Rebuilds the state of a game from its rows.
pub async fn load_state(
    connection: &mut SqliteConnection,
    game: &GameRecord,
) -> Result<GameState, AppError> {
    Ok(GameState {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;
    use crate::domain::professor_ai::PetersenAi;
    use crate::domain::turn::{Answer, Engine, MoveOrder};
    use crate::domain::{InvestigatorId, LocationId, RandomDice};

    #[tokio::test]
    async fn a_state_survives_a_round_trip() {
        let pool = connect("sqlite::memory:").await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let mut dice = RandomDice::seeded(11);
        let mut ai = PetersenAi::seeded(11);
        let ids = [
            InvestigatorId::Poirot,
            InvestigatorId::Hastings,
            InvestigatorId::Captain,
            InvestigatorId::Lemon,
        ];
        let (mut state, _) = Engine::new_game(&ids, &mut dice, &mut ai);
        let game_id = GameRecord::insert(&mut connection, "token", &state)
            .await
            .unwrap();
        let mut game = GameRecord::load(&mut connection, game_id, "token")
            .await
            .unwrap()
            .unwrap();
        save_state(&mut connection, &mut game, &state)
            .await
            .unwrap();
        assert_eq!(load_state(&mut connection, &game).await.unwrap(), state);

        let sail = (0..4)
            .map(|index| (index, MoveOrder::Go(LocationId::NantucketSound)))
            .collect();
        Engine::new(&mut state, &mut dice, &mut ai)
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
}
