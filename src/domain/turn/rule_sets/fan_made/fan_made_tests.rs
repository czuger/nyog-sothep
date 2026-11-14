//! The fan-made rules played by the engine, with scripted dice.

use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::ScriptedDice;
use crate::domain::investigator_ai::InvestigatorsAi;
use crate::domain::investigator_id::InvestigatorId;
use crate::domain::investigator_status::InvestigatorStatus;
use crate::domain::item::{Gear, Item, REVOLVER, SIGN, Weapon};
use crate::domain::location_id::LocationId::{self, *};
use crate::domain::professor_ai::test_state;
use crate::domain::turn::scripted_professor::ScriptedProfessor;
use crate::domain::turn::*;

const IDS: [InvestigatorId; 4] = [
    InvestigatorId::Armitage,
    InvestigatorId::Carter,
    InvestigatorId::Barnes,
    InvestigatorId::Walker,
];

/// A fan-made game state at its first movement prompt, the investigators in Boston.
fn fan_state() -> GameState {
    let mut state = test_state();
    state.rules = RuleSet::FanMade;
    for investigator in &mut state.investigators {
        investigator.location = Boston;
        investigator.previous = Boston;
    }
    state
}

/// Runs `act` on an engine over `state`, with these dice; returns the log.
fn with_engine(
    state: &mut GameState,
    rolls: &[u8],
    act: impl FnOnce(&mut Engine<'_>),
) -> Vec<String> {
    let mut dice = ScriptedDice::new(rolls, 1);
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut investigators = InvestigatorsAi::seeded(0);
    let mut engine = Engine::new(state, &mut dice, &mut brain, &mut investigators);
    act(&mut engine);
    engine
        .into_log()
        .into_iter()
        .map(|line| line.text)
        .collect()
}

#[test]
fn the_professor_draws_on_the_table_of_his_place() {
    // SAN 4+4+4+3 = 15, then four starting draws and the first turn's draw in Worcester
    // (a crossroads: 3 gives Dreams).
    let mut script = vec![4; 12];
    script.extend([3; 5]);
    let mut dice = ScriptedDice::new(&script, 1);
    let mut brain = ScriptedProfessor::new(Worcester, Concord);
    let mut investigators = InvestigatorsAi::seeded(0);
    let (state, _) = Engine::new_game(
        &IDS,
        Side::Investigators,
        RuleSet::FanMade,
        &mut dice,
        &mut brain,
        &mut investigators,
    );
    assert_eq!(state.rules, RuleSet::FanMade);
    assert_eq!(state.prompt, Some(Prompt::Moves));
    assert_eq!(state.counters.hand(), vec![CounterKind::Dreams; 5]);
    assert!(state.investigators.iter().all(|i| i.starting_san == 15));
}

#[test]
fn events_are_rolled_on_a_d20_without_choosing_a_table() {
    let mut state = fan_state();
    state.investigators[0].san = 10;
    for investigator in state.investigators.iter_mut().skip(1) {
        investigator.location = NantucketSound;
    }
    let log = with_engine(&mut state, &[12], |engine| engine.fan_event(0));
    assert!(
        log[0].contains("table A — Premières pistes (1d20 : 12)"),
        "{log:?}"
    );
    assert_eq!(state.investigators[0].san, 12);
}

#[test]
fn no_event_and_no_psychoanalyst_once_nyog_sothep_is_out() {
    let mut state = fan_state();
    assert!(MoveOptions::of(&state, 0, false).unwrap().psychoanalyst);
    state.nyog.location = Some(Concord);
    assert!(!MoveOptions::of(&state, 0, false).unwrap().psychoanalyst);
    let log = with_engine(&mut state, &[12], |engine| engine.fan_event(0));
    assert!(log.is_empty());
    assert_eq!(
        state.phase,
        Phase::Action {
            investigator: 0,
            step: ActionStep::Fight
        }
    );
}

#[test]
fn san_never_goes_above_its_start() {
    let mut state = fan_state();
    state.investigators[0].san = 13;
    with_engine(&mut state, &[], |engine| {
        assert_eq!(engine.gain_san(0, 5), 2);
    });
    assert_eq!(state.investigators[0].san, 15);
}

#[test]
fn an_item_already_held_goes_to_a_companion_or_gives_san() {
    let mut state = fan_state();
    state.investigators[0].take(REVOLVER);
    state.investigators[0].san = 10;
    with_engine(&mut state, &[], |engine| engine.fan_receive(0, REVOLVER));
    assert!(state.investigators[1].weapon);
    for investigator in state.investigators.iter_mut().skip(1) {
        investigator.take(REVOLVER);
    }
    with_engine(&mut state, &[], |engine| engine.fan_receive(0, REVOLVER));
    assert_eq!(state.investigators[0].san, 11);
}

#[test]
fn the_better_weapon_is_kept_and_the_other_given() {
    let mut state = fan_state();
    state.investigators[0].take(REVOLVER);
    state.investigators[1].take(SIGN);
    with_engine(&mut state, &[], |engine| {
        engine.fan_receive(0, Item::Weapon(Weapon::Shotgun))
    });
    assert_eq!(state.investigators[0].weapon_kind, Weapon::Shotgun);
    assert!(state.investigators[1].holds(REVOLVER));
}

#[test]
fn a_third_object_is_given_away() {
    let mut state = fan_state();
    for gear in [Gear::Pass, Gear::Laudanum] {
        state.investigators[0].take(Item::Gear(gear));
    }
    with_engine(&mut state, &[], |engine| {
        engine.fan_receive(0, Item::Gear(Gear::Dynamite))
    });
    assert_eq!(state.investigators[0].gear.len(), 2);
    assert!(state.investigators[1].holds(Item::Gear(Gear::Dynamite)));
}

#[test]
fn fanatics_arrive_every_three_turns_without_limit() {
    let mut state = fan_state();
    for (turn, expected) in [(2, 0), (3, 1), (6, 2), (9, 3), (10, 3)] {
        state.turn = turn;
        with_engine(&mut state, &[], |engine| engine.fanatics_arrive());
        let held = state
            .counters
            .hand()
            .into_iter()
            .filter(|kind| *kind == CounterKind::Fanatics)
            .count();
        assert_eq!(held, expected, "turn {turn}");
    }
}

/// Three protected spell holders in Concord, which Nyog'Sothep enters.
fn spell_holders_in_concord() -> GameState {
    let mut state = fan_state();
    state.nyog.location = Some(Concord);
    for investigator in state.investigators.iter_mut().take(3) {
        investigator.location = Concord;
        investigator.take(SIGN);
        investigator.spell = true;
    }
    state
}

#[test]
fn each_failed_cast_helps_the_next_until_a_partial_banishment() {
    let mut state = spell_holders_in_concord();
    with_engine(&mut state, &[2, 4], |engine| {
        engine.nyog_enters(Concord, true)
    });
    assert_eq!(state.nyog.failed_banishments, 2);
    assert_eq!(state.prompt, Some(Prompt::BanishmentCity));
    assert_eq!(state.investigators[0].san, 11);
}

#[test]
fn a_natural_one_kills_the_caster_and_a_six_banishes() {
    let mut state = spell_holders_in_concord();
    with_engine(&mut state, &[1, 6], |engine| {
        engine.nyog_enters(Concord, true)
    });
    assert_eq!(state.investigators[0].status, InvestigatorStatus::Dead);
    assert_eq!(state.outcome, Some(Outcome::Banished));
    assert!(state.investigators[1].banished);
}

#[test]
fn the_game_is_lost_when_nobody_knows_the_spell_any_more() {
    let mut state = fan_state();
    state.nyog.location = Some(Concord);
    state.investigators[0].spell = true;
    with_engine(&mut state, &[], |engine| engine.check_end());
    assert_eq!(state.outcome, None);
    state.investigators[0].status = InvestigatorStatus::Mad;
    with_engine(&mut state, &[], |engine| engine.check_end());
    assert_eq!(state.outcome, Some(Outcome::NyogSothepReigns));
}

#[test]
fn a_wounded_professor_moves_in_the_open() {
    let mut state = fan_state();
    with_engine(&mut state, &[], |engine| {
        engine.rules().professor_wounded(engine)
    });
    assert!(state.move_in_the_open);
}

#[test]
fn the_pass_crosses_borders_freely() {
    let mut state = fan_state();
    let rules = RuleSet::FanMade.rules();
    assert!(!rules.free_crossing(&state.investigators[0]));
    state.investigators[0].take(Item::Gear(Gear::Pass));
    assert!(rules.free_crossing(&state.investigators[0]));
}

#[test]
fn powder_reveals_the_neighbouring_counters() {
    let mut state = fan_state();
    state.investigators[0].take(Item::Gear(Gear::IbnGhaziPowder));
    let neighbour: LocationId = crate::domain::travel::road_neighbours(Boston)[0];
    assert!(state.counters.draw(CounterKind::Ghouls));
    assert!(state.counters.place(CounterKind::Ghouls, neighbour));
    with_engine(&mut state, &[], |engine| engine.fan_event(0));
    assert!(!state.investigators[0].holds(Item::Gear(Gear::IbnGhaziPowder)));
    assert!(matches!(
        state.counters.placed_at(neighbour).unwrap().place,
        crate::domain::CounterPlace::Placed {
            discovered: true,
            ..
        }
    ));
}
