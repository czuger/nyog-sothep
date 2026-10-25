use sqlx::SqlitePool;

use crate::db::{EventRecord, GameRecord, load_state, save_state};
use crate::domain::RandomDice;
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::professor_ai::PetersenAi;
use crate::domain::turn::{Answer, Engine, GameState, LogLine};
use crate::error::AppError;
use crate::service::token_tilts::refresh_tilts;
use crate::service::write_transaction::begin_write;

/// Applies the human's answer to the pending prompt; the computer's side then plays until the
/// human must decide again, all inside one write transaction.
///
/// # Errors
/// `GameNotFound` for a wrong id or token, `Answer` when the engine refuses the answer (nothing
/// is then written), or a database error.
pub async fn answer_prompt(
    pool: &SqlitePool,
    game_id: i64,
    token: &str,
    answer: Answer,
) -> Result<(), AppError> {
    let mut transaction = begin_write(pool).await?;
    let mut game = GameRecord::load(&mut transaction, game_id, token)
        .await?
        .ok_or(AppError::GameNotFound)?;
    let mut state = load_state(&mut transaction, &game).await?;
    let log = play(&mut state, answer)?;
    save_state(&mut transaction, &mut game, &state).await?;
    refresh_tilts(&mut transaction, game_id, &state).await?;
    EventRecord::insert_all(&mut transaction, game_id, &log).await?;
    transaction.commit().await?;
    Ok(())
}

/// Runs the engine on the answer with real dice and the computer's AIs.
fn play(state: &mut GameState, answer: Answer) -> Result<Vec<LogLine>, AppError> {
    let mut dice = RandomDice::new();
    let mut professor = PetersenAi::new();
    let mut investigators = InvestigatorsAi::new();
    let mut engine = Engine::new(state, &mut dice, &mut professor, &mut investigators);
    engine.answer(answer)?;
    Ok(engine.into_log())
}
