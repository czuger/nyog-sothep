//! The professor's AI plays full games against the investigators' AI, one side played from
//! outside the engine as the human would.
//!
//! Every answer of the professor's AI is checked for legality, the side played from outside
//! must never see an answer refused, every game must end without error, and the results are
//! printed (`cargo test --test simulation -- --nocapture`) to tune the balance;
//! `SIMULATION_DEBUG=1` also describes the games left unfinished.

use nyog_sothep::domain::investigator_ai::InvestigatorsAi;
use nyog_sothep::domain::professor_ai::PetersenAi;
use nyog_sothep::domain::turn::{
    Answer, Engine, GameState, InvestigatorBrain, Outcome, ProfessorBrain, ProfessorMove,
    ProfessorMoveOptions, Prompt, Side, Teleport,
};
use nyog_sothep::domain::{CounterKind, InvestigatorId, LocationId, RandomDice};

const GAMES: u64 = 150;
const SAME_GAME_SEEDS: u64 = 10;
const TURN_LIMIT: u32 = 120;

/// The AI, with every answer checked against the options it was given.
struct CheckedAi(PetersenAi);

fn distinct_cities(named: &[LocationId], count: usize) -> bool {
    let mut sorted = named.to_vec();
    sorted.sort();
    sorted.dedup();
    sorted.len() == count && named.len() == count && named.iter().all(|c| c.is_city())
}

impl ProfessorBrain for CheckedAi {
    fn setup(&mut self, state: &GameState) -> (LocationId, LocationId) {
        let (start, city) = self.0.setup(state);
        assert!(start.is_city() && city.is_city());
        (start, city)
    }

    fn teleport(&mut self, state: &GameState, options: &[Teleport]) -> Option<Teleport> {
        let picked = self.0.teleport(state, options);
        assert!(picked.is_none_or(|picked| options.contains(&picked)));
        picked
    }

    fn movement(&mut self, state: &GameState, options: &ProfessorMoveOptions) -> ProfessorMove {
        let picked = self.0.movement(state, options);
        let legal = match picked.flying_horror {
            true => options.flying.contains(&picked.destination),
            false => options.steps.contains(&picked.destination),
        };
        assert!(legal, "{picked:?}");
        picked
    }

    fn attack(&mut self, state: &GameState, candidates: &[usize], forced: bool) -> Option<usize> {
        let picked = self.0.attack(state, candidates, forced);
        assert!(picked.is_none_or(|picked| candidates.contains(&picked)));
        assert!(!forced || picked.is_some());
        picked
    }

    fn relocate(&mut self, state: &GameState, options: &[LocationId]) -> LocationId {
        let picked = self.0.relocate(state, options);
        assert!(options.contains(&picked));
        picked
    }

    fn carry(&mut self, state: &GameState, index: usize, options: &[LocationId]) -> LocationId {
        let picked = self.0.carry(state, index, options);
        assert!(options.contains(&picked));
        picked
    }

    fn placement(&mut self, state: &GameState, options: &[CounterKind]) -> Option<CounterKind> {
        let picked = self.0.placement(state, options);
        assert!(picked.is_none_or(|picked| options.contains(&picked)));
        picked
    }

    fn table1_choice(&mut self, state: &GameState, options: &[CounterKind]) -> CounterKind {
        let picked = self.0.table1_choice(state, options);
        assert!(options.contains(&picked));
        picked
    }

    fn discard(&mut self, state: &GameState, hand: &[CounterKind]) -> CounterKind {
        let picked = self.0.discard(state, hand);
        assert!(hand.contains(&picked));
        picked
    }

    fn summon(&mut self, state: &GameState) -> bool {
        self.0.summon(state)
    }

    fn nyog_step(&mut self, state: &GameState, options: &[LocationId]) -> LocationId {
        let picked = self.0.nyog_step(state, options);
        assert!(options.contains(&picked));
        picked
    }

    fn name_cities(&mut self, state: &GameState, count: usize) -> Vec<LocationId> {
        let named = self.0.name_cities(state, count);
        assert!(distinct_cities(&named, count), "{named:?}");
        assert!(named.contains(&state.professor.location));
        named
    }

    fn name_fanatics_cities(&mut self, state: &GameState) -> Vec<LocationId> {
        let named = self.0.name_fanatics_cities(state);
        let fanatics = state.counters.fanatics_cities();
        assert!(distinct_cities(&named, 2), "{named:?}");
        assert!(named.iter().any(|city| fanatics.contains(city)));
        named
    }
}

fn play(seed: u64, human: Side) -> (Option<Outcome>, u32) {
    let mut dice = RandomDice::seeded(seed);
    let mut professor = CheckedAi(PetersenAi::seeded(seed));
    let mut investigators = InvestigatorsAi::seeded(seed);
    let mut ids = InvestigatorId::ALL.to_vec();
    ids.truncate(4);
    let (mut state, _) =
        Engine::new_game(&ids, human, &mut dice, &mut professor, &mut investigators);
    while !state.is_finished() && state.turn <= TURN_LIMIT {
        let prompt = state
            .prompt
            .clone()
            .expect("an unfinished game waits for the human");
        let answer = match &prompt {
            Prompt::Professor(asked) => Answer::Professor(professor.answer(&state, asked)),
            _ => investigators.answer(&state.public_copy(), &prompt),
        };
        let mut engine = Engine::new(&mut state, &mut dice, &mut professor, &mut investigators);
        engine
            .answer(answer.clone())
            .unwrap_or_else(|error| panic!("seed {seed}: {prompt:?} {answer:?}: {error}"));
    }
    if state.outcome.is_none() && std::env::var("SIMULATION_DEBUG").is_ok() {
        let removed = state
            .counters
            .all()
            .iter()
            .filter(|c| {
                c.kind == CounterKind::Fanatics
                    && c.place == nyog_sothep::domain::CounterPlace::Removed
            })
            .count();
        eprintln!(
            "seed {seed}: summoned {} fanatics cities {} removed {} hand {:?} active {} prof {:?} nyog {:?} hp {} spells {} signs {} sans {:?} target {:?} nyog at {:?} joined {}",
            state.nyog.is_summoned(),
            state.counters.fanatics_cities().len(),
            removed,
            state.counters.hand(),
            state.active_investigators().len(),
            state.professor.location,
            state.nyog.city,
            state.professor.hp,
            state
                .investigators
                .iter()
                .filter(|i| i.is_active() && i.spell)
                .count(),
            state
                .investigators
                .iter()
                .filter(|i| i.is_active() && i.sign)
                .count(),
            state
                .investigators
                .iter()
                .map(|i| (i.san, i.location))
                .collect::<Vec<_>>(),
            state.nyog.target,
            state.nyog.location,
            state.nyog.joined,
        );
    }
    (state.outcome, state.turn)
}

#[test]
fn the_ais_play_full_games() {
    simulate(Side::Investigators);
}

/// Answering the professor's prompts from outside the engine plays exactly the same games as
/// letting the engine ask his brain.
#[test]
fn either_side_can_be_played_from_outside() {
    for seed in 0..SAME_GAME_SEEDS {
        assert_eq!(
            play(seed, Side::Professor),
            play(seed, Side::Investigators),
            "seed {seed}"
        );
    }
}

fn simulate(human: Side) {
    let mut wins = [0u32; 4];
    let mut unfinished = 0;
    let mut turns = 0;
    for seed in 0..GAMES {
        let (outcome, length) = play(seed, human);
        turns += length;
        match outcome {
            Some(Outcome::ProfessorKilled) => wins[0] += 1,
            Some(Outcome::Banished) => wins[1] += 1,
            Some(Outcome::NyogSothepReigns) => wins[2] += 1,
            Some(Outcome::InvestigatorsLost) => wins[3] += 1,
            None => unfinished += 1,
        }
    }
    println!(
        "{GAMES} games, {:.1} turns on average: professor killed {}, banished {}, Nyog'Sothep reigns {}, investigators lost {}, unfinished after {TURN_LIMIT} turns {}",
        f64::from(turns) / GAMES as f64,
        wins[0],
        wins[1],
        wins[2],
        wins[3],
        unfinished
    );
    assert!(wins.iter().sum::<u32>() > 0);
}
