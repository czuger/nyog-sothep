//! Tunes the AIs' weights by self-play.
//!
//! `cargo run --release --example tune_ai -- [rounds] [games]` (defaults: 4 rounds of 400
//! games). Each round tunes one side, alternately the professor and the investigators: every
//! weight in turn is nudged up then down by a fifth, the same seeded games are played against
//! the other side's current weights, and a change is kept when the side's score rises by more
//! than three standard errors (a win counts 1, a game left unfinished ½): with some fifty tries
//! a round, two would let a lucky one through every round. Each round plays new
//! seeds so that no change is kept for a lucky few games. The weights found are printed at the
//! end, to be pasted into the `Default` implementations.

use std::thread;

use nyog_sothep::domain::investigator_ai::{InvestigatorWeights, InvestigatorsAi};
use nyog_sothep::domain::professor_ai::{PetersenAi, ProfessorWeights};
use nyog_sothep::domain::turn::{Engine, InvestigatorBrain, Side};
use nyog_sothep::domain::{InvestigatorId, RandomDice};

const TURN_LIMIT: u32 = 120;
const NUDGE: f64 = 0.2;
/// Standard errors a gain must exceed to be kept.
const SIGNIFICANCE: f64 = 3.0;
const DEFAULT_ROUNDS: usize = 4;
const DEFAULT_GAMES: u64 = 400;

/// The weights of both sides.
#[derive(Clone)]
struct Weights {
    professor: ProfessorWeights,
    investigators: InvestigatorWeights,
}

impl Weights {
    /// The named weights of one side.
    fn of_side(&mut self, side: Side) -> Vec<(&'static str, &mut f64)> {
        match side {
            Side::Professor => self.professor.fields_mut(),
            Side::Investigators => self.investigators.fields_mut(),
        }
    }
}

/// The professor's score in one game: 1 if he wins, ½ if it is left unfinished.
fn professor_score(seed: u64, weights: &Weights) -> f64 {
    let mut dice = RandomDice::seeded(seed);
    let mut professor = PetersenAi::seeded(seed).with_weights(weights.professor.clone());
    let mut investigators =
        InvestigatorsAi::seeded(seed).with_weights(weights.investigators.clone());
    let ids = &InvestigatorId::ALL[..4];
    let (mut state, _) = Engine::new_game(
        ids,
        Side::Investigators,
        &mut dice,
        &mut professor,
        &mut investigators,
    );
    while !state.is_finished() && state.turn <= TURN_LIMIT {
        let Some(prompt) = state.prompt.clone() else {
            break;
        };
        let answer = investigators.answer(&state.public_copy(), &prompt);
        let mut engine = Engine::new(&mut state, &mut dice, &mut professor, &mut investigators);
        if let Err(error) = engine.answer(answer) {
            panic!("seed {seed}: refused answer: {error}");
        }
    }
    match state.outcome {
        Some(outcome) if outcome.won_by(Side::Professor) => 1.0,
        Some(_) => 0.0,
        None => 0.5,
    }
}

/// One side's score in each of the given games, played on every core.
fn scores(side: Side, seeds: &[u64], weights: &Weights) -> Vec<f64> {
    let cores = thread::available_parallelism().map_or(1, usize::from);
    let chunk = seeds.len().div_ceil(cores).max(1);
    let professor: Vec<f64> = thread::scope(|scope| {
        let handles: Vec<_> = seeds
            .chunks(chunk)
            .map(|chunk| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|seed| professor_score(*seed, weights))
                        .collect::<Vec<f64>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| {
                handle
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });
    match side {
        Side::Professor => professor,
        Side::Investigators => professor.into_iter().map(|score| 1.0 - score).collect(),
    }
}

/// The mean gain of `candidate` over `baseline`, game by game, and its standard error.
fn gain(candidate: &[f64], baseline: &[f64]) -> (f64, f64) {
    let differences: Vec<f64> = candidate
        .iter()
        .zip(baseline)
        .map(|(new, old)| new - old)
        .collect();
    let count = differences.len() as f64;
    let mean = differences.iter().sum::<f64>() / count;
    let variance = differences
        .iter()
        .map(|difference| (difference - mean).powi(2))
        .sum::<f64>()
        / (count - 1.0).max(1.0);
    (mean, (variance / count).sqrt())
}

/// Nudges each weight of `side` in turn and keeps the changes that clearly help it.
fn tune_round(side: Side, seeds: &[u64], weights: &mut Weights) {
    let mut baseline = scores(side, seeds, weights);
    let average = |scores: &[f64]| scores.iter().sum::<f64>() / scores.len() as f64;
    println!("{side:?}: {:.3} to start with", average(&baseline));
    let count = weights.of_side(side).len();
    for field in 0..count {
        for factor in [1.0 + NUDGE, 1.0 - NUDGE] {
            let mut candidate = weights.clone();
            let (name, value) = {
                let mut fields = candidate.of_side(side);
                let (name, value) = &mut fields[field];
                **value *= factor;
                (*name, **value)
            };
            let tried = scores(side, seeds, &candidate);
            let (mean, error) = gain(&tried, &baseline);
            if mean > SIGNIFICANCE * error && mean > 0.0 {
                println!("  {name} → {value:.3}: {mean:+.3} ± {error:.3}, kept");
                *weights = candidate;
                baseline = tried;
                break;
            }
        }
    }
    println!("{side:?}: {:.3} at the end", average(&baseline));
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let rounds = arguments
        .first()
        .and_then(|rounds| rounds.parse().ok())
        .unwrap_or(DEFAULT_ROUNDS);
    let games = arguments
        .get(1)
        .and_then(|games| games.parse().ok())
        .unwrap_or(DEFAULT_GAMES);
    let mut weights = Weights {
        professor: ProfessorWeights::default(),
        investigators: InvestigatorWeights::default(),
    };
    for round in 0..rounds {
        let side = match round % 2 {
            0 => Side::Professor,
            _ => Side::Investigators,
        };
        let first = 1_000 + round as u64 * games;
        let seeds: Vec<u64> = (first..first + games).collect();
        tune_round(side, &seeds, &mut weights);
    }
    println!("{:#?}", weights.professor);
    println!("{:#?}", weights.investigators);
}
