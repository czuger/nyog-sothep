//! The professor's AI plays full games against a simple investigator bot.
//!
//! Every answer of the AI is checked for legality, every game must end without error, and the
//! results are printed (`cargo test --test simulation -- --nocapture`) to tune the
//! balance; `SIMULATION_DEBUG=1` also describes the games left unfinished.

use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{RngExt, SeedableRng};

use nyog_sothep::domain::map;
use nyog_sothep::domain::professor_ai::PetersenAi;
use nyog_sothep::domain::travel::{self, Network};
use nyog_sothep::domain::turn::{
    Answer, ClueKind, Engine, GameState, MoveOptions, MoveOrder, Outcome, ProfessorBrain,
    ProfessorMove, ProfessorMoveOptions, Prompt, Teleport,
};
use nyog_sothep::domain::{
    CounterKind, EventTable, FightChoice, InvestigatorId, LocationId, ProfessorQuestion, RandomDice,
};

const GAMES: u64 = 150;
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

/// A plain investigator player: armed investigators chase the last sighting, the others roam;
/// spell holders gather once Nyog'Sothep is out.
struct Bot {
    rng: StdRng,
}

impl Bot {
    fn answer(&mut self, state: &GameState, prompt: &Prompt) -> Answer {
        match prompt {
            Prompt::Moves => Answer::Moves(
                (0..state.investigators.len())
                    .filter_map(|index| {
                        MoveOptions::of(state, index, false)
                            .map(|options| (index, self.order(state, index, &options)))
                    })
                    .collect(),
            ),
            Prompt::PlayAgain { investigator } => {
                let options = MoveOptions::of(state, *investigator, true).expect("plays again");
                Answer::PlayAgain(self.order(state, *investigator, &options))
            }
            Prompt::Table { .. } => Answer::Table(match self.rng.random_bool(0.5) {
                true => EventTable::Two,
                false => EventTable::Three,
            }),
            Prompt::Cities { question, .. } => {
                let count = question.cities_named_by_investigator().unwrap_or(0);
                let optional =
                    *question == ProfessorQuestion::NyogSothepInTwoCities { optional: true };
                let cities: Vec<LocationId> = map::cities().collect();
                let named = match optional && self.rng.random_bool(0.3) {
                    true => Vec::new(),
                    false => cities.sample(&mut self.rng, count).copied().collect(),
                };
                Answer::Cities(named)
            }
            Prompt::Fight { options, .. } => Answer::Fight(options[0]).clone_for(options),
            Prompt::BanishmentCity => {
                let far = map::cities()
                    .filter(|city| state.investigators_at(*city).is_empty())
                    .max_by_key(|city| {
                        travel::distance(*city, LocationId::Nantucket, Network::All, |_| true)
                    })
                    .unwrap_or(LocationId::Gloucester);
                Answer::BanishmentCity(far)
            }
        }
    }

    fn order(&mut self, state: &GameState, index: usize, options: &MoveOptions) -> MoveOrder {
        let me = &state.investigators[index];
        let goal = self.goal(state, index);
        if me.san <= 6 && options.psychoanalyst && !(state.nyog.is_summoned() && me.spell) {
            return MoveOrder::Psychoanalyst;
        }
        if let Some(goal) = goal {
            let closest = options.destinations.iter().copied().min_by_key(|to| {
                travel::distance(*to, goal, Network::All, |_| true).unwrap_or(u8::MAX)
            });
            let here = travel::distance(me.location, goal, Network::All, |_| true);
            if let Some(to) = closest
                && travel::distance(to, goal, Network::All, |_| true) < here
            {
                return MoveOrder::Go(to);
            }
            return MoveOrder::Stay;
        }
        match options.destinations.choose(&mut self.rng) {
            Some(to) if self.rng.random_bool(0.8) => MoveOrder::Go(*to),
            _ => MoveOrder::Stay,
        }
    }

    fn goal(&self, state: &GameState, index: usize) -> Option<LocationId> {
        let me = &state.investigators[index];
        if let Some(nyog) = state.nyog.location {
            if me.spell {
                return rally(state);
            }
            return flee(state, me.location, nyog);
        }
        if !me.weapon {
            return None;
        }
        state.clues.iter().rev().find_map(|clue| match clue.kind {
            ClueKind::Spotted { at } if clue.turn + 3 >= state.turn => Some(at),
            _ => None,
        })
    }
}

/// The open city closest, in total, to the active spell holders.
fn rally(state: &GameState) -> Option<LocationId> {
    let holders: Vec<LocationId> = state
        .active_investigators()
        .into_iter()
        .filter(|index| state.investigators[*index].spell)
        .map(|index| state.investigators[index].location)
        .collect();
    map::cities()
        .filter(|city| !state.is_destroyed(*city))
        .min_by_key(|city| {
            holders
                .iter()
                .map(|at| {
                    u32::from(travel::distance(*at, *city, Network::All, |_| true).unwrap_or(30))
                })
                .max()
        })
}

/// A neighbouring city farther from Nyog'Sothep, if any.
fn flee(state: &GameState, from: LocationId, nyog: LocationId) -> Option<LocationId> {
    let away = |city: LocationId| travel::distance(city, nyog, Network::All, |_| true).unwrap_or(0);
    travel::neighbours(from)
        .iter()
        .copied()
        .filter(|city| !state.is_destroyed(*city))
        .max_by_key(|city| away(*city))
        .filter(|city| away(*city) > away(from))
}

trait FightPreference {
    fn clone_for(self, options: &[FightChoice]) -> Answer;
}

impl FightPreference for Answer {
    fn clone_for(self, options: &[FightChoice]) -> Answer {
        let preferred = [FightChoice::Weapon, FightChoice::Sign, FightChoice::Hide];
        let choice = preferred
            .into_iter()
            .find(|choice| options.contains(choice))
            .unwrap_or(options[0]);
        Answer::Fight(choice)
    }
}

fn play(seed: u64) -> (Option<Outcome>, u32) {
    let mut dice = RandomDice::seeded(seed);
    let mut ai = CheckedAi(PetersenAi::seeded(seed));
    let mut bot = Bot {
        rng: StdRng::seed_from_u64(seed),
    };
    let mut ids = InvestigatorId::ALL.to_vec();
    ids.truncate(4);
    let (mut state, _) = Engine::new_game(&ids, &mut dice, &mut ai);
    while !state.is_finished() && state.turn <= TURN_LIMIT {
        let prompt = state
            .prompt
            .clone()
            .expect("an unfinished game waits for the human");
        let answer = bot.answer(&state, &prompt);
        let mut engine = Engine::new(&mut state, &mut dice, &mut ai);
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
            "seed {seed}: summoned {} fanatics cities {} removed {} hand {:?} active {} prof {:?} nyog {:?} hp {} spells {} signs {} sans {:?} target {:?}",
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
        );
    }
    (state.outcome, state.turn)
}

#[test]
fn the_ai_plays_full_games() {
    let mut wins = [0u32; 4];
    let mut unfinished = 0;
    let mut turns = 0;
    for seed in 0..GAMES {
        let (outcome, length) = play(seed);
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
