//! The professor's AI plays full games against the investigators' AI, one side played from
//! outside the engine as the human would.
//!
//! Every answer of the professor's AI is checked for legality, the side played from outside
//! must never see an answer refused, every game must end without error, at most a few may reach
//! the turn limit, and the results are printed (`cargo test --test simulation -- --nocapture`)
//! to watch the balance; `SIMULATION_DEBUG=1` also describes the games left unfinished, and
//! `SIMULATION_TRACE=<seed>` prints the end of that game's log.

use nyog_sothep::domain::investigator_ai::InvestigatorsAi;
use nyog_sothep::domain::professor_ai::PetersenAi;
use nyog_sothep::domain::turn::{
    Answer, Engine, GameState, InvestigatorBrain, Outcome, ProfessorBrain, ProfessorMove,
    ProfessorMoveOptions, Prompt, ReplayBoard, ReplayStep, RuleSet, Side, Teleport,
};
use nyog_sothep::domain::{CounterKind, InvestigatorId, LocationId, RandomDice};

const GAMES: u64 = 150;
const SAME_GAME_SEEDS: u64 = 10;
const TURN_LIMIT: u32 = 120;
const TRACE_LINES: usize = 80;
/// The share of games, in percent, allowed to reach the turn limit.
const MAX_UNFINISHED_PERCENT: u64 = 3;

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

/// Plays a game to its end (or the turn limit), with the replay steps written on the way;
/// `SIMULATION_TRACE=<seed>` prints the end of that game's log.
fn play_recorded(seed: u64, human: Side, rules: RuleSet) -> (GameState, Vec<ReplayStep>) {
    let mut dice = RandomDice::seeded(seed);
    let mut professor = CheckedAi(PetersenAi::seeded(seed));
    let mut investigators = InvestigatorsAi::seeded(seed);
    let mut ids = InvestigatorId::ALL.to_vec();
    ids.truncate(4);
    let (mut state, history) = Engine::new_game(
        &ids,
        human,
        rules,
        &mut dice,
        &mut professor,
        &mut investigators,
    );
    let mut replay = history.replay;
    let mut log: Vec<String> = history.log.into_iter().map(|line| line.text).collect();
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
        let history = engine.into_history();
        replay.extend(history.replay);
        log.extend(history.log.into_iter().map(|line| line.text));
    }
    if std::env::var("SIMULATION_TRACE").is_ok_and(|traced| traced == seed.to_string()) {
        for line in &log[log.len().saturating_sub(TRACE_LINES)..] {
            eprintln!("{line}");
        }
    }
    (state, replay)
}

fn play(seed: u64, human: Side, rules: RuleSet) -> (Option<Outcome>, u32, i32) {
    let (state, _) = play_recorded(seed, human, rules);
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
        let trapped = state
            .active_investigators()
            .into_iter()
            .filter(|index| {
                let at = state.investigators[*index].location;
                nyog_sothep::domain::travel::neighbours(at)
                    .iter()
                    .all(|next| next.is_city() && state.is_destroyed(*next))
            })
            .count();
        eprintln!("seed {seed}: trapped investigators {trapped}");
    }
    (state.outcome, state.turn, state.professor.hp)
}

#[test]
fn the_ais_play_full_games() {
    simulate(Side::Investigators, RuleSet::Original);
}

#[test]
fn the_ais_play_full_fan_made_games() {
    simulate(Side::Investigators, RuleSet::FanMade);
}

/// Answering the professor's prompts from outside the engine plays exactly the same games as
/// letting the engine ask his brain.
#[test]
fn either_side_can_be_played_from_outside() {
    for rules in RuleSet::ALL {
        for seed in 0..SAME_GAME_SEEDS {
            assert_eq!(
                play(seed, Side::Professor, rules),
                play(seed, Side::Investigators, rules),
                "seed {seed}, {rules:?}"
            );
        }
    }
}

/// The replay starts from a board, every step tells what happened or shows dice, and its boards
/// end on the final state of the game.
#[test]
fn the_replay_follows_the_whole_game() {
    for (seed, rules) in
        (0..SAME_GAME_SEEDS).flat_map(|seed| RuleSet::ALL.map(|rules| (seed, rules)))
    {
        for human in [Side::Investigators, Side::Professor] {
            let (state, replay) = play_recorded(seed, human, rules);
            assert!(replay[0].board.is_some(), "seed {seed}");
            assert!(
                replay
                    .iter()
                    .all(|step| step.text.is_some() || !step.dice.is_empty()),
                "seed {seed}"
            );
            let last = replay.iter().rev().find_map(|step| step.board.as_ref());
            assert_eq!(last, Some(&ReplayBoard::of(&state)), "seed {seed}");
        }
    }
}

fn simulate(human: Side, rules: RuleSet) {
    let mut wins = [0u32; 5];
    let mut unfinished: u64 = 0;
    let mut turns = 0;
    let mut hit_points: i32 = 0;
    for seed in 0..GAMES {
        let (outcome, length, hp) = play(seed, human, rules);
        turns += length;
        hit_points += hp;
        match outcome {
            Some(Outcome::ProfessorKilled) => wins[0] += 1,
            Some(Outcome::Banished) => wins[1] += 1,
            Some(Outcome::NyogSothepReigns) => wins[2] += 1,
            Some(Outcome::InvestigatorsLost) => wins[3] += 1,
            Some(Outcome::FanaticsDestroyed) => wins[4] += 1,
            None => unfinished += 1,
        }
    }
    println!(
        "{}, {GAMES} games, {:.1} turns on average: professor killed {}, Fanatics destroyed {}, banished {}, Nyog'Sothep reigns {}, investigators lost {}, unfinished after {TURN_LIMIT} turns {}; the professor ends with {:.1} hit points on average",
        rules.label(),
        f64::from(turns) / GAMES as f64,
        wins[0],
        wins[4],
        wins[1],
        wins[2],
        wins[3],
        unfinished,
        f64::from(hit_points) / GAMES as f64
    );
    assert!(wins.iter().sum::<u32>() > 0);
    assert!(
        unfinished * 100 <= GAMES * MAX_UNFINISHED_PERCENT,
        "{unfinished} games stalled"
    );
}
