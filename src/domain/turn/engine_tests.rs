//! Whole rounds played with scripted dice, scripted answers and a scripted professor.

use crate::domain::counter::CounterPlace;
use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::ScriptedDice;
use crate::domain::event_table::EventTable;
use crate::domain::fight_choice::FightChoice;
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::investigator_id::InvestigatorId;
use crate::domain::investigator_status::InvestigatorStatus;
use crate::domain::location_id::LocationId;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::turn::scripted_professor::ScriptedProfessor;
use crate::domain::turn::*;
use LocationId::*;

const IDS: [InvestigatorId; 4] = [
    InvestigatorId::Armitage,
    InvestigatorId::Carter,
    InvestigatorId::Barnes,
    InvestigatorId::Walker,
];

/// A game at the first movement prompt: SAN 15 each, five Fanatics in the professor's hand.
fn new_game(brain: &mut ScriptedProfessor) -> GameState {
    let mut script = vec![4; 12];
    for _ in 0..5 {
        script.extend([5, 6]);
    }
    let mut dice = ScriptedDice::new(&script, 1);
    let mut investigators = InvestigatorsAi::seeded(0);
    let (state, _) = Engine::new_game(
        &IDS,
        Side::Investigators,
        RuleSet::Original,
        &mut dice,
        brain,
        &mut investigators,
    );
    state
}

/// Puts the investigators somewhere: the first ones on land, the others at sea.
fn place(state: &mut GameState, cities: &[LocationId]) {
    for (index, investigator) in state.investigators.iter_mut().enumerate() {
        let at = cities.get(index).copied().unwrap_or(NantucketSound);
        investigator.location = at;
        investigator.previous = at;
    }
}

fn answer(
    state: &mut GameState,
    brain: &mut ScriptedProfessor,
    rolls: &[u8],
    answer: Answer,
) -> Result<Vec<String>, AnswerError> {
    let mut dice = ScriptedDice::new(rolls, 1);
    let mut investigators = InvestigatorsAi::seeded(0);
    let mut engine = Engine::new(state, &mut dice, brain, &mut investigators);
    engine.answer(answer)?;
    Ok(engine
        .into_log()
        .into_iter()
        .map(|line| line.text)
        .collect())
}

fn everyone_stays() -> Answer {
    Answer::Moves((0..4).map(|index| (index, MoveOrder::Stay)).collect())
}

fn put_counter(state: &mut GameState, kind: CounterKind, city: LocationId) {
    assert!(state.counters.draw(kind) || state.counters.hand().contains(&kind));
    assert!(state.counters.place(kind, city));
}

#[test]
fn a_new_game_waits_for_the_first_moves() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let state = new_game(&mut brain);
    assert_eq!(state.turn, 1);
    assert_eq!(state.prompt, Some(Prompt::Moves));
    assert!(
        state
            .investigators
            .iter()
            .all(|i| i.san == 15 && i.location == Nantucket)
    );
    assert_eq!(state.counters.hand(), vec![CounterKind::Fanatics; 5]);
    assert_eq!(state.professor.location, Worcester);
    assert_eq!(state.nyog.city, Concord);
    assert_eq!(state.route_sheet.len(), 2);
}

#[test]
fn the_replay_reveals_the_setup_and_the_dice() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut dice = ScriptedDice::new(&[4; 12], 1);
    let mut investigators = InvestigatorsAi::seeded(0);
    let (state, history) = Engine::new_game(
        &IDS,
        Side::Investigators,
        RuleSet::Original,
        &mut dice,
        &mut brain,
        &mut investigators,
    );
    let first = &history.replay[0];
    assert!(first.text.as_deref().unwrap().starts_with("Mai 1921."));
    assert!(first.board.is_some());
    let san = &history.replay[1];
    assert_eq!(
        san.text.as_deref(),
        Some("Le docteur Armitage commence avec 15 SAN.")
    );
    assert_eq!(san.dice, vec![4, 4, 4]);
    assert_eq!(san.board.as_ref().unwrap().investigators[0].san, 15);
    let setup =
        "Le professeur part en secret à Worcester et écrit que Nyog'Sothep apparaîtra à Concord.";
    assert!(
        history
            .replay
            .iter()
            .any(|step| step.text.as_deref() == Some(setup))
    );
    assert!(
        history
            .log
            .iter()
            .all(|line| !line.text.contains("Worcester"))
    );
    let last = history
        .replay
        .iter()
        .rev()
        .find_map(|step| step.board.as_ref());
    assert_eq!(last, Some(&ReplayBoard::of(&state)));
}

#[test]
fn moves_are_checked_and_sea_skips_the_action_phase() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    let partial = Answer::Moves(vec![(0, MoveOrder::Stay)]);
    assert_eq!(
        answer(&mut state, &mut brain, &[], partial),
        Err(AnswerError::MissingMove("Randolph Carter"))
    );
    let illegal = Answer::Moves((0..4).map(|i| (i, MoveOrder::Go(Boston))).collect());
    assert_eq!(
        answer(&mut state, &mut brain, &[], illegal),
        Err(AnswerError::IllegalMove("Le docteur Armitage"))
    );
    let sail = Answer::Moves((0..4).map(|i| (i, MoveOrder::Go(NantucketSound))).collect());
    answer(&mut state, &mut brain, &[], sail).unwrap();
    assert_eq!(state.turn, 2);
    assert_eq!(state.prompt, Some(Prompt::Moves));
    assert!(
        state
            .investigators
            .iter()
            .all(|i| i.location == NantucketSound)
    );
}

#[test]
fn no_psychoanalyst_among_ruins() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston, Lynn]);
    state.destroyed.push(Boston);
    assert!(!MoveOptions::of(&state, 0, false).unwrap().psychoanalyst);
    assert!(MoveOptions::of(&state, 1, false).unwrap().psychoanalyst);
    let mut orders: Vec<(usize, MoveOrder)> = (0..4).map(|i| (i, MoveOrder::Stay)).collect();
    orders[0].1 = MoveOrder::Psychoanalyst;
    assert_eq!(
        answer(&mut state, &mut brain, &[], Answer::Moves(orders)),
        Err(AnswerError::IllegalMove("Le docteur Armitage"))
    );
}

#[test]
fn an_encounter_without_an_event() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    put_counter(&mut state, CounterKind::Dreams, Boston);
    let log = answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    assert_eq!(state.investigators[0].san, 13);
    assert!(log.iter().any(|line| line.contains("Rêves")));
    assert!(state.counters.placed_at(Boston).is_none());
    assert_eq!(state.prompt, Some(Prompt::Moves));
}

#[test]
fn ghouls_killed_by_a_weapon_then_an_event() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    state.investigators[0].weapon = true;
    put_counter(&mut state, CounterKind::Ghouls, Boston);
    answer(&mut state, &mut brain, &[5], everyone_stays()).unwrap();
    assert_eq!(state.investigators[0].san, 13);
    assert_eq!(state.prompt, Some(Prompt::Table { investigator: 0 }));
    answer(&mut state, &mut brain, &[2], Answer::Table(EventTable::Two)).unwrap();
    assert_eq!(state.investigators[0].san, 11);
    assert_eq!(state.prompt, Some(Prompt::Moves));
}

#[test]
fn a_border_roll_of_five_holds_the_investigator_one_turn() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Nashua]);
    let mut orders: Vec<(usize, MoveOrder)> = (1..4).map(|i| (i, MoveOrder::Stay)).collect();
    orders.push((0, MoveOrder::Go(Lowell)));
    answer(&mut state, &mut brain, &[5], Answer::Moves(orders.clone())).unwrap();
    assert_eq!(state.investigators[0].location, Nashua);
    assert!(state.investigators[0].free_border);
    state.prompt = Some(Prompt::Moves);
    state.phase = Phase::Moves;
    answer(&mut state, &mut brain, &[6], Answer::Moves(orders)).unwrap();
    assert_eq!(state.investigators[0].location, Lowell);
    assert!(!state.investigators[0].free_border);
}

#[test]
fn a_border_roll_of_six_blocks_that_crossing_next_turn() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Nashua]);
    let mut orders: Vec<(usize, MoveOrder)> = (1..4).map(|i| (i, MoveOrder::Stay)).collect();
    orders.push((0, MoveOrder::Go(Lowell)));
    answer(&mut state, &mut brain, &[6], Answer::Moves(orders)).unwrap();
    assert_eq!(state.investigators[0].location, Nashua);
    state.investigators[0].skips_action = false;
    let options = MoveOptions::of(&state, 0, false).unwrap();
    assert!(!options.destinations.contains(&Lowell));
    assert!(options.destinations.contains(&Dunwich));
}

#[test]
fn the_unavoidable_fight_with_a_weapon() {
    let mut brain = ScriptedProfessor::new(Boston, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Lynn]);
    state.investigators[0].weapon = true;
    state.investigators[0].medallion = true;
    let mut orders: Vec<(usize, MoveOrder)> = (1..4).map(|i| (i, MoveOrder::Stay)).collect();
    orders.push((0, MoveOrder::Go(Boston)));
    answer(&mut state, &mut brain, &[], Answer::Moves(orders)).unwrap();
    assert_eq!(state.prompt, Some(Prompt::Table { investigator: 0 }));
    answer(&mut state, &mut brain, &[5], Answer::Table(EventTable::Two)).unwrap();
    let named = state.clues.iter().find_map(|clue| match &clue.kind {
        ClueKind::NamedCities { cities } => Some(cities.clone()),
        _ => None,
    });
    assert!(
        named.unwrap().contains(&Boston),
        "a bad bluff is replaced by the truth"
    );
    assert_eq!(
        state.prompt,
        Some(Prompt::Fight {
            investigator: 0,
            options: vec![FightChoice::Weapon, FightChoice::Hide]
        })
    );
    answer(
        &mut state,
        &mut brain,
        &[6],
        Answer::Fight(FightChoice::Weapon),
    )
    .unwrap();
    assert_eq!(state.professor.hp, 11);
    assert_eq!(state.investigators[0].damage_dealt, 3);
    assert_ne!(state.professor.location, Boston);
    assert!(
        state
            .clues
            .iter()
            .any(|c| c.kind == ClueKind::Retreated { from: Boston })
    );
}

#[test]
fn killing_the_professor_wins() {
    let mut brain = ScriptedProfessor::new(Boston, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    state.professor.hp = 2;
    state.investigators[0].weapon = true;
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    answer(
        &mut state,
        &mut brain,
        &[5, 2],
        Answer::Table(EventTable::Two),
    )
    .unwrap();
    assert_eq!(state.outcome, Some(Outcome::ProfessorKilled));
    assert_eq!(state.phase, Phase::Finished);
    assert_eq!(state.prompt, None);
    assert_eq!(
        answer(&mut state, &mut brain, &[], everyone_stays()),
        Err(AnswerError::GameOver)
    );
}

#[test]
fn killing_the_sixth_fanatics_wins() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    for _ in 0..5 {
        assert!(state.counters.spend(CounterKind::Fanatics));
    }
    place(&mut state, &[Boston]);
    state.investigators[0].weapon = true;
    state.investigators[0].medallion = true;
    put_counter(&mut state, CounterKind::Fanatics, Boston);
    assert_eq!(state.counters.fanatics_in_play(), 5);
    answer(&mut state, &mut brain, &[6], everyone_stays()).unwrap();
    assert_eq!(state.counters.fanatics_in_play(), 4);
    assert_eq!(state.outcome, Some(Outcome::FanaticsDestroyed));
    assert_eq!(state.phase, Phase::Finished);
    assert_eq!(state.prompt, None);
}

#[test]
fn the_professor_attacks_on_arrival_and_carries_the_unarmed() {
    let mut brain = ScriptedProfessor::new(Boston, Concord)
        .goes(Boston)
        .goes(Lynn);
    brain.attack = true;
    let mut state = new_game(&mut brain);
    place(&mut state, &[Lynn]);
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    answer(
        &mut state,
        &mut brain,
        &[5, 6],
        Answer::Table(EventTable::Two),
    )
    .unwrap();
    assert_eq!(state.turn, 2);
    assert_eq!(state.investigators[0].san, 11);
    assert_ne!(state.investigators[0].location, Lynn);
    assert!(
        state
            .clues
            .iter()
            .any(|c| c.kind == ClueKind::Spotted { at: Lynn })
    );
    assert_eq!(state.prompt, Some(Prompt::Moves));
}

#[test]
fn going_mad_raises_fanatics_in_the_city() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    state.investigators[0].san = 3;
    put_counter(&mut state, CounterKind::Ghouls, Boston);
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    assert_eq!(state.investigators[0].status, InvestigatorStatus::Mad);
    let counter = state.counters.placed_at(Boston).unwrap();
    assert_eq!(counter.kind, CounterKind::Fanatics);
    assert_eq!(
        counter.place,
        CounterPlace::Placed {
            city: Boston,
            discovered: true
        }
    );
}

/// Five Fanatics placed, the professor in Nyog'Sothep's city, ready to summon.
fn ready_to_summon(brain: &mut ScriptedProfessor) -> GameState {
    let mut state = new_game(brain);
    for city in [Boston, Lynn, Arkham, Lowell, Dedham] {
        put_counter(&mut state, CounterKind::Fanatics, city);
    }
    state.professor.location = Concord;
    state.prompt = None;
    state.phase = Phase::ProfessorSummon;
    brain.summons = true;
    state
}

fn run(state: &mut GameState, brain: &mut ScriptedProfessor, rolls: &[u8]) {
    let mut dice = ScriptedDice::new(rolls, 1);
    let mut investigators = InvestigatorsAi::seeded(0);
    Engine::new(state, &mut dice, brain, &mut investigators).run();
}

#[test]
fn three_banishers_at_the_summoning_can_win_at_once() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = ready_to_summon(&mut brain);
    place(&mut state, &[Concord, Concord, Concord]);
    for investigator in &mut state.investigators[..3] {
        investigator.spell = true;
        investigator.sign = true;
    }
    run(&mut state, &mut brain, &[1]);
    assert!(state.nyog.is_summoned() && state.nyog.joined);
    assert!(state.is_destroyed(Concord));
    assert_eq!(state.investigators[0].san, 11);
    assert_eq!(state.outcome, Some(Outcome::Banished));
    assert!(state.investigators[..3].iter().all(|i| i.banished));
}

#[test]
fn nyog_sothep_kills_investigators_without_a_sign() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = ready_to_summon(&mut brain);
    place(&mut state, &[Concord, Concord]);
    run(&mut state, &mut brain, &[]);
    assert_eq!(state.investigators[0].status, InvestigatorStatus::Dead);
    assert_eq!(state.outcome, Some(Outcome::NyogSothepReigns));
}

#[test]
fn a_partial_banishment_lets_the_investigators_move_nyog_sothep() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = ready_to_summon(&mut brain);
    place(&mut state, &[Concord, Concord, Concord]);
    for investigator in &mut state.investigators[..3] {
        investigator.spell = true;
        investigator.sign = true;
    }
    run(&mut state, &mut brain, &[2]);
    assert_eq!(state.prompt, Some(Prompt::BanishmentCity));
    let wrong = Answer::BanishmentCity(GulfOfMaine);
    assert_eq!(
        answer(&mut state, &mut brain, &[], wrong),
        Err(AnswerError::NotACity)
    );
    answer(
        &mut state,
        &mut brain,
        &[],
        Answer::BanishmentCity(Gloucester),
    )
    .unwrap();
    assert_eq!(state.nyog.location, Some(Gloucester));
    assert!(!state.nyog.joined);
    assert!(state.is_destroyed(Gloucester));
    assert_eq!(state.prompt, Some(Prompt::Moves));
    let options = MoveOptions::of(&state, 0, false).unwrap();
    assert!(
        !options.destinations.contains(&Concord),
        "destroyed cities are closed"
    );
}

#[test]
fn playing_again_gives_a_new_move_and_action_phase() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    answer(
        &mut state,
        &mut brain,
        &[4],
        Answer::Table(EventTable::Three),
    )
    .unwrap();
    assert_eq!(state.prompt, Some(Prompt::PlayAgain { investigator: 0 }));
    let psychoanalyst = Answer::PlayAgain(MoveOrder::Psychoanalyst);
    assert!(answer(&mut state, &mut brain, &[], psychoanalyst).is_err());
    answer(
        &mut state,
        &mut brain,
        &[],
        Answer::PlayAgain(MoveOrder::Go(Lynn)),
    )
    .unwrap();
    assert_eq!(state.investigators[0].location, Lynn);
    assert_eq!(state.prompt, Some(Prompt::Table { investigator: 0 }));
}

#[test]
fn the_investigator_names_a_city_and_learns_its_encounter() {
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Boston]);
    state.investigators[0].sign = true;
    state.investigators[0].medallion = true;
    put_counter(&mut state, CounterKind::Ghouls, Lynn);
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    answer(
        &mut state,
        &mut brain,
        &[6, 5, 5],
        Answer::Table(EventTable::Three),
    )
    .unwrap();
    let asked = Prompt::Cities {
        investigator: 0,
        question: ProfessorQuestion::EncounterInOneCity,
    };
    assert_eq!(state.prompt, Some(asked));
    let two = Answer::Cities(vec![Boston, Lynn]);
    assert_eq!(
        answer(&mut state, &mut brain, &[], two),
        Err(AnswerError::WrongCityCount(1))
    );
    answer(&mut state, &mut brain, &[], Answer::Cities(vec![Lynn])).unwrap();
    let learned = ClueKind::EncounterKind {
        city: Lynn,
        kind: Some(CounterKind::Ghouls),
    };
    assert!(state.clues.iter().any(|clue| clue.kind == learned));
}

#[test]
fn teleportation_moves_a_nearby_investigator() {
    let mut brain = ScriptedProfessor::new(Boston, Concord);
    let mut state = new_game(&mut brain);
    place(&mut state, &[Lynn]);
    assert!(state.counters.discard(CounterKind::Fanatics));
    assert!(state.counters.draw(CounterKind::Teleportation));
    brain.teleport = true;
    answer(&mut state, &mut brain, &[], everyone_stays()).unwrap();
    answer(&mut state, &mut brain, &[5], Answer::Table(EventTable::Two)).unwrap();
    assert_eq!(state.turn, 2);
    assert_ne!(state.investigators[0].location, Lynn);
    assert_eq!(state.investigators[0].san, 13);
    assert!(
        state
            .clues
            .iter()
            .any(|c| c.kind == ClueKind::Teleported { near: Lynn })
    );
}

/// A game where the human plays the professor, waiting for his setup: SAN 15 each.
fn professor_game(brain: &mut ScriptedProfessor) -> GameState {
    let mut dice = ScriptedDice::new(&[4; 12], 1);
    let mut investigators = InvestigatorsAi::seeded(0);
    let (state, _) = Engine::new_game(
        &IDS,
        Side::Professor,
        RuleSet::Original,
        &mut dice,
        brain,
        &mut investigators,
    );
    state
}

fn professor(answer: ProfessorAnswer) -> Answer {
    Answer::Professor(answer)
}

#[test]
fn the_human_professor_sets_up_draws_and_moves() {
    let mut brain = ScriptedProfessor::new(Boston, Boston);
    let mut state = professor_game(&mut brain);
    assert_eq!(
        state.prompt,
        Some(Prompt::Professor(ProfessorPrompt::Setup))
    );
    let at_sea = professor(ProfessorAnswer::Setup {
        start: GulfOfMaine,
        nyog_city: Concord,
    });
    assert_eq!(
        answer(&mut state, &mut brain, &[], at_sea),
        Err(AnswerError::NotACity)
    );
    let setup = professor(ProfessorAnswer::Setup {
        start: Worcester,
        nyog_city: Concord,
    });
    answer(&mut state, &mut brain, &[1, 1], setup).unwrap();
    let offered = vec![CounterKind::FlyingHorror, CounterKind::Ghouls];
    assert_eq!(
        state.prompt,
        Some(Prompt::Professor(ProfessorPrompt::Table1 {
            options: offered
        }))
    );
    let rolls = [5, 6, 5, 6, 5, 6];
    let kept = professor(ProfessorAnswer::Counter(CounterKind::Ghouls));
    answer(&mut state, &mut brain, &rolls, kept).unwrap();
    assert_eq!(state.turn, 1);
    assert_eq!(state.counters.hand().len(), 4);
    let Some(Prompt::Professor(ProfessorPrompt::Movement { options })) = state.prompt.clone()
    else {
        panic!("{:?}", state.prompt);
    };
    assert_eq!(options.steps[0], Worcester);
    assert!(options.flying.is_empty());

    let before = state.clone();
    let far = professor(ProfessorAnswer::Location(Boston));
    assert_eq!(
        answer(&mut state, &mut brain, &[], far),
        Err(AnswerError::NotAnOption)
    );
    assert_eq!(state, before);
    assert_eq!(
        answer(&mut state, &mut brain, &[], everyone_stays()),
        Err(AnswerError::Unexpected)
    );

    let step = professor(ProfessorAnswer::Location(options.steps[1]));
    answer(&mut state, &mut brain, &[], step).unwrap();
    assert_eq!(state.professor.location, options.steps[1]);
    let Some(Prompt::Professor(ProfessorPrompt::Placement { options })) = state.prompt.clone()
    else {
        panic!("{:?}", state.prompt);
    };
    assert!(options.contains(&CounterKind::Fanatics));
    let placed = professor(ProfessorAnswer::Placement(Some(CounterKind::Fanatics)));
    answer(&mut state, &mut brain, &[5, 6], placed).unwrap();
    let stage = state.route_sheet.last().unwrap();
    assert_eq!(stage.counter, Some(CounterKind::Fanatics));
    // The computer's investigators moved and played their action phases: the professor's
    // next decision comes, at the latest, with his next move.
    assert!(state.investigators.iter().any(|i| i.location != Nantucket));
    assert_eq!(
        state.prompt.as_ref().map(Prompt::side),
        Some(Side::Professor)
    );
}

#[test]
fn the_human_professor_must_name_his_own_city() {
    let mut brain = ScriptedProfessor::new(Boston, Boston);
    let mut state = professor_game(&mut brain);
    state.professor.location = Worcester;
    state.prompt = Some(Prompt::Professor(ProfessorPrompt::NameCities { count: 2 }));
    state.phase = Phase::Action {
        investigator: 3,
        step: ActionStep::AfterEvent { plays_again: false },
    };
    let lie = professor(ProfessorAnswer::Cities(vec![Boston, Lynn]));
    assert_eq!(
        answer(&mut state, &mut brain, &[], lie),
        Err(AnswerError::MustNameOwnCity)
    );
    let three = professor(ProfessorAnswer::Cities(vec![Boston, Lynn, Worcester]));
    assert_eq!(
        answer(&mut state, &mut brain, &[], three),
        Err(AnswerError::WrongCityCount(2))
    );
    let truth = professor(ProfessorAnswer::Cities(vec![Lynn, Worcester]));
    answer(&mut state, &mut brain, &[], truth).unwrap();
    let clue = ClueKind::NamedCities {
        cities: vec![Lynn, Worcester],
    };
    assert!(state.clues.iter().any(|c| c.kind == clue));
}

#[test]
fn the_human_professor_carries_an_unarmed_investigator_then_the_round_goes_on() {
    let mut brain = ScriptedProfessor::new(Boston, Boston);
    let mut state = professor_game(&mut brain);
    place(&mut state, &[Boston]);
    state.professor.location = Boston;
    state.nyog = crate::domain::NyogSothep::new(Concord);
    state.turn = 1;
    state.prompt = None;
    state.phase = Phase::Action {
        investigator: 0,
        step: ActionStep::Fight,
    };
    run(&mut state, &mut brain, &[]);
    assert_eq!(state.investigators[0].san, 11);
    let Some(Prompt::Professor(ProfessorPrompt::Carry {
        investigator: 0,
        options,
    })) = state.prompt.clone()
    else {
        panic!("{:?}", state.prompt);
    };
    let carried = professor(ProfessorAnswer::Location(options[0]));
    answer(&mut state, &mut brain, &[], carried).unwrap();
    assert_eq!(state.investigators[0].location, options[0]);
    assert_eq!(state.turn, 2);
    assert!(matches!(
        state.prompt,
        Some(Prompt::Professor(ProfessorPrompt::Movement { .. }))
    ));
}
