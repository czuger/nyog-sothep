use rand::Rng;
use rand::seq::SliceRandom;

use crate::domain::investigator_ai::paths::steps;
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::turn::{ClueKind, GameState};

/// The cities an investigator names to the professor.
pub(super) fn choose_cities<R: Rng>(
    state: &GameState,
    investigator: usize,
    question: ProfessorQuestion,
    rng: &mut R,
) -> Vec<LocationId> {
    match question {
        ProfessorQuestion::NyogSothepInTwoCities { optional } => {
            nyog_sothep_cities(state, optional, rng)
        }
        _ => {
            let count = question.cities_named_by_investigator().unwrap_or(0);
            unknown_encounters_near(state, investigator, count, rng)
        }
    }
}

/// Two cities that can still be Nyog'Sothep's: when it is known to be one of two, one of them
/// and a city already ruled out, which settles it. Declines when the city is already known.
fn nyog_sothep_cities<R: Rng>(state: &GameState, optional: bool, rng: &mut R) -> Vec<LocationId> {
    let (possible, ruled_out) = nyog_sothep_candidates(state);
    if optional && (state.nyog.is_summoned() || possible.len() <= 1) {
        return Vec::new();
    }
    let mut named: Vec<LocationId> = possible.clone();
    named.shuffle(rng);
    named.truncate(2);
    if named.len() == 2 && possible.len() == 2 {
        named.truncate(1);
    }
    let mut fillers: Vec<LocationId> = ruled_out
        .into_iter()
        .chain(map::cities())
        .filter(|city| !named.contains(city))
        .collect();
    fillers.dedup();
    named.extend(fillers.into_iter().take(2 - named.len()));
    named
}

/// The cities that may still be Nyog'Sothep's, and those the notebook rules out.
fn nyog_sothep_candidates(state: &GameState) -> (Vec<LocationId>, Vec<LocationId>) {
    let mut possible: Vec<LocationId> = map::cities().collect();
    let mut ruled_out = Vec::new();
    for clue in &state.clues {
        match &clue.kind {
            ClueKind::NyogSothepAmong {
                cities,
                present: true,
            } => possible.retain(|city| cities.contains(city)),
            ClueKind::NyogSothepAmong {
                cities,
                present: false,
            } => {
                possible.retain(|city| !cities.contains(city));
                ruled_out.extend(cities);
            }
            _ => {}
        }
    }
    (possible, ruled_out)
}

/// The nearest cities whose encounter the investigators do not know yet.
fn unknown_encounters_near<R: Rng>(
    state: &GameState,
    investigator: usize,
    count: usize,
    rng: &mut R,
) -> Vec<LocationId> {
    let here = state.investigators[investigator].location;
    let asked: Vec<LocationId> = state
        .clues
        .iter()
        .filter_map(|clue| match clue.kind {
            ClueKind::Encounter { city, .. } | ClueKind::EncounterKind { city, .. } => Some(city),
            _ => None,
        })
        .collect();
    let mut cities: Vec<LocationId> = map::cities()
        .filter(|city| *city != here && !state.is_destroyed(*city))
        .filter(|city| !asked.contains(city) && state.counters.placed_at(*city).is_none())
        .collect();
    cities.shuffle(rng);
    cities.sort_by_key(|city| steps(here, *city));
    cities.truncate(count);
    cities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;
    use crate::domain::turn::Clue;
    use LocationId::*;

    #[test]
    fn a_known_pair_is_settled_with_a_ruled_out_city() {
        let mut state = test_state();
        state.clues = vec![
            Clue {
                turn: 1,
                kind: ClueKind::NyogSothepAmong {
                    cities: vec![Boston, Lynn],
                    present: true,
                },
            },
            Clue {
                turn: 2,
                kind: ClueKind::NyogSothepAmong {
                    cities: vec![Arkham, Gloucester],
                    present: false,
                },
            },
        ];
        let question = ProfessorQuestion::NyogSothepInTwoCities { optional: true };
        let named = choose_cities(&state, 0, question, &mut rand::rng());
        assert_eq!(named.len(), 2);
        assert_eq!(
            named
                .iter()
                .filter(|city| [Boston, Lynn].contains(city))
                .count(),
            1
        );
        assert!(named.contains(&Arkham));
    }

    #[test]
    fn encounters_are_asked_nearby() {
        let state = test_state();
        let named = choose_cities(
            &state,
            0,
            ProfessorQuestion::EncountersInTwoCities,
            &mut rand::rng(),
        );
        assert_eq!(named.len(), 2);
        assert!(named.iter().all(|city| steps(Nantucket, *city) <= 3));
    }
}
