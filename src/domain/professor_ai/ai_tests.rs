use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_ai::{PetersenAi, test_state};
use crate::domain::travel;
use crate::domain::turn::{ProfessorBrain, ProfessorMoveOptions};
use LocationId::*;

#[test]
fn setup_picks_two_cities_with_a_well_connected_appearance_city() {
    for seed in 0..30 {
        let mut ai = PetersenAi::seeded(seed);
        let (start, city) = ai.setup(&test_state());
        assert!(start.is_city() && city.is_city());
        assert!(travel::road_neighbours(city).len() >= 3, "{city:?}");
        assert_ne!(start, Nantucket);
    }
}

#[test]
fn named_cities_always_include_the_real_position() {
    let mut ai = PetersenAi::seeded(3);
    let mut state = test_state();
    for city in map::cities() {
        state.professor.location = city;
        for count in [2, 3] {
            let named = ai.name_cities(&state, count);
            let mut distinct = named.clone();
            distinct.dedup();
            assert_eq!(distinct.len(), count);
            assert!(named.contains(&city));
            assert!(named.iter().all(|named| named.is_city()));
        }
    }
}

#[test]
fn fanatics_names_include_a_fanatics_city() {
    let mut ai = PetersenAi::seeded(4);
    let mut state = test_state();
    for city in [Boston, Providence] {
        state.counters.draw(CounterKind::Fanatics);
        state.counters.place(CounterKind::Fanatics, city);
    }
    for _ in 0..20 {
        let named = ai.name_fanatics_cities(&state);
        assert_eq!(named.len(), 2);
        assert_ne!(named[0], named[1]);
        assert!(named.contains(&Boston) || named.contains(&Providence));
    }
}

#[test]
fn moves_stay_within_the_options() {
    let mut ai = PetersenAi::seeded(5);
    let state = test_state();
    let from = state.professor.location;
    let mut steps = vec![from];
    steps.extend(travel::neighbours(from));
    let options = ProfessorMoveOptions {
        steps: steps.clone(),
        flying: Vec::new(),
    };
    for _ in 0..30 {
        let chosen = ai.movement(&state, &options);
        assert!(steps.contains(&chosen.destination) && !chosen.flying_horror);
    }
}

#[test]
fn the_professor_avoids_armed_investigators_and_hunts_the_unarmed() {
    let mut ai = PetersenAi::seeded(6);
    let mut state = test_state();
    state.investigators[0].weapon = true;
    assert_eq!(ai.attack(&state, &[0, 1], false), None);
    assert_eq!(ai.attack(&state, &[0, 1], true), Some(1));
    assert_eq!(ai.attack(&state, &[1], false), Some(1));
    state.investigators[1].medallion = true;
    assert_eq!(ai.attack(&state, &[1], false), None);
}

#[test]
fn fanatics_are_placed_away_from_armed_investigators() {
    let mut ai = PetersenAi::seeded(7);
    let mut state = test_state();
    state.counters.draw(CounterKind::Fanatics);
    state.counters.draw(CounterKind::Ghouls);
    let options = [CounterKind::Fanatics, CounterKind::Ghouls];
    assert_eq!(ai.placement(&state, &options), Some(CounterKind::Fanatics));
    state.investigators[0].weapon = true;
    state.investigators[0].location = Oxford;
    assert_ne!(ai.placement(&state, &options), Some(CounterKind::Fanatics));
}
