use crate::db::{GameDocument, Store};
use crate::domain::RandomDice;
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::professor_ai::PetersenAi;
use crate::domain::turn::{Answer, Engine, GameState, History};
use crate::error::AppError;
use crate::service::token_tilts::refresh_tilts;

/// Applies the human's answer to the pending prompt; the computer's side then plays until the
/// human must decide again, all inside one write transaction.
///
/// # Errors
/// `GameNotFound` for a wrong id or token, `Answer` when the engine refuses the answer (nothing
/// is then written), or a database error.
pub async fn answer_prompt(
    store: &Store,
    game_id: i64,
    token: &str,
    answer: Answer,
) -> Result<(), AppError> {
    let token = token.to_string();
    store
        .run(move |store| {
            store.write(|transaction| {
                let mut game = GameDocument::find(transaction, game_id, &token)?
                    .ok_or(AppError::GameNotFound)?;
                let history = play(&mut game.state, answer)?;
                game.tilts = refresh_tilts(&game.tilts, &game.state);
                game.save(transaction)?;
                game.record_history(transaction, &history)
            })
        })
        .await
}

/// Runs the engine on the answer with real dice and the computer's AIs.
fn play(state: &mut GameState, answer: Answer) -> Result<History, AppError> {
    let mut dice = RandomDice::new();
    let mut professor = PetersenAi::new();
    let mut investigators = InvestigatorsAi::new();
    let mut engine = Engine::new(state, &mut dice, &mut professor, &mut investigators);
    engine.answer(answer)?;
    Ok(engine.into_history())
}
