use crate::domain::location_id::{LOCATION_COUNT, LocationId};
use crate::domain::travel::{self, Network};
use crate::domain::turn::{ClueKind, GameState};

/// The farthest the Flying Horror carries the professor.
const FLYING_HORROR_RANGE: u8 = 3;
/// The farthest a teleported investigator stood from the professor.
const TELEPORT_RANGE: u8 = 2;
/// How much less likely a location is when an armed investigator can strike it at once.
const ARMED_NEAR_FACTOR: f64 = 0.3;
/// How much less likely the sea is: he loses time there.
const SEA_FACTOR: f64 = 0.5;
/// How much more likely a location is close to Fanatics he has left.
const FANATICS_NEAR_FACTOR: f64 = 1.5;
/// Steps from a face-up Fanatics city that still count as close.
const FANATICS_RANGE: u8 = 2;

/// The locations where the investigators can still place the professor, given their notebook.
///
/// Each turn the set grows by one move (three with the Flying Horror), then every clue of the
/// turn narrows it. The professor's AI reads it to know how well hidden he is; the investigators' AI,
/// to know where to hunt him.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessorBelief {
    possible: [bool; LOCATION_COUNT],
}

impl ProfessorBelief {
    /// Rebuilds the set from the clues of the game.
    pub fn of(state: &GameState) -> ProfessorBelief {
        let mut belief = ProfessorBelief {
            possible: [true; LOCATION_COUNT],
        };
        let sea_announced_from = state
            .clues
            .iter()
            .find(|clue| clue.kind == ClueKind::SeaAnnouncements)
            .map(|clue| clue.turn);
        for turn in 1..=state.turn {
            let clues: Vec<&ClueKind> = state
                .clues
                .iter()
                .filter(|clue| clue.turn == turn)
                .map(|clue| &clue.kind)
                .collect();
            let range = match clues.contains(&&ClueKind::FlyingHorror) {
                true => FLYING_HORROR_RANGE,
                false => 1,
            };
            belief.expand(range);
            let announced = sea_announced_from.is_some_and(|from| from < turn);
            if announced && !clues.contains(&&ClueKind::AtSea) {
                belief.keep(|location| location.is_city());
            }
            for clue in clues {
                belief.apply(clue);
            }
        }
        belief
    }

    /// Whether the professor may be there.
    pub fn contains(&self, location: LocationId) -> bool {
        self.possible[location.index()]
    }

    /// How many locations are still possible.
    pub fn size(&self) -> usize {
        self.possible.iter().filter(|possible| **possible).count()
    }

    /// The possible locations, in board order.
    pub fn locations(&self) -> Vec<LocationId> {
        LocationId::ALL
            .into_iter()
            .filter(|location| self.contains(*location))
            .collect()
    }

    /// How likely each location is, summing to 1, from what the investigators can see: he keeps
    /// away from armed investigators and from the sea, and lingers near the Fanatics he left.
    pub fn weights(&self, state: &GameState) -> [f64; LOCATION_COUNT] {
        let armed: Vec<LocationId> = state
            .active_investigators()
            .into_iter()
            .map(|index| &state.investigators[index])
            .filter(|investigator| investigator.weapon)
            .map(|investigator| investigator.location)
            .collect();
        let fanatics = state.counters.fanatics_cities();
        let mut weights = [0.0; LOCATION_COUNT];
        for location in self.locations() {
            let near = |places: &[LocationId], range: u8| {
                places.iter().any(|at| {
                    travel::distance(*at, location, Network::All, |_| true)
                        .is_some_and(|steps| steps <= range)
                })
            };
            let mut weight = 1.0;
            if near(&armed, 1) {
                weight *= ARMED_NEAR_FACTOR;
            }
            if location.is_sea() {
                weight *= SEA_FACTOR;
            }
            if near(&fanatics, FANATICS_RANGE) {
                weight *= FANATICS_NEAR_FACTOR;
            }
            weights[location.index()] = weight;
        }
        normalized(weights)
    }

    fn expand(&mut self, range: u8) {
        let mut grown = self.possible;
        for from in self.locations() {
            if range == 1 {
                for next in travel::neighbours(from) {
                    grown[next.index()] = true;
                }
                continue;
            }
            for reached in travel::within(from, 1, range, Network::All, |_| true) {
                grown[reached.index()] = true;
            }
        }
        self.possible = grown;
    }

    fn keep(&mut self, keep: impl Fn(LocationId) -> bool) {
        let before = self.possible;
        for location in LocationId::ALL {
            self.possible[location.index()] &= keep(location);
        }
        // Contradictory clues cannot happen with honest answers; never end with an empty set.
        if self.size() == 0 {
            self.possible = before;
        }
    }

    fn only(&mut self, places: &[LocationId]) {
        self.keep(|location| places.contains(&location));
    }

    fn apply(&mut self, clue: &ClueKind) {
        match clue {
            ClueKind::Spotted { at } => {
                self.possible = [false; LOCATION_COUNT];
                self.possible[at.index()] = true;
            }
            ClueKind::NamedCities { cities } | ClueKind::MetInvestigator { cities } => {
                self.only(cities)
            }
            ClueKind::AtSea => self.keep(|location| location.is_sea()),
            ClueKind::Retreated { from } => {
                self.possible = [false; LOCATION_COUNT];
                for next in travel::neighbours(*from) {
                    self.possible[next.index()] = true;
                }
            }
            ClueKind::Teleported { near } => {
                let around = travel::within(*near, 0, TELEPORT_RANGE, Network::All, |_| true);
                self.only(&around);
            }
            _ => {}
        }
    }
}

/// Scales weights so that they sum to 1 (all zero stays all zero).
pub fn normalized(mut weights: [f64; LOCATION_COUNT]) -> [f64; LOCATION_COUNT] {
    let total: f64 = weights.iter().sum();
    if total > 0.0 {
        for weight in &mut weights {
            *weight /= total;
        }
    }
    weights
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::turn::Clue;
    use LocationId::*;

    fn with_clues(turn: u32, clues: Vec<(u32, ClueKind)>) -> GameState {
        let mut state = crate::domain::professor_ai::test_state();
        state.turn = turn;
        state.clues = clues
            .into_iter()
            .map(|(turn, kind)| Clue { turn, kind })
            .collect();
        state
    }

    #[test]
    fn a_sighting_grows_by_one_move_per_turn() {
        let state = with_clues(2, vec![(1, ClueKind::Spotted { at: Nashua })]);
        let belief = ProfessorBelief::of(&state);
        assert_eq!(belief.locations(), vec![Nashua, Dunwich, Lowell]);
    }

    #[test]
    fn named_cities_and_sea_announcements_narrow_it() {
        let clues = vec![
            (1, ClueKind::Spotted { at: Boston }),
            (1, ClueKind::SeaAnnouncements),
            (
                2,
                ClueKind::NamedCities {
                    cities: vec![Lynn, Providence, Concord],
                },
            ),
        ];
        let belief = ProfessorBelief::of(&with_clues(2, clues));
        assert_eq!(belief.locations(), vec![Concord, Lynn]);
        let belief = ProfessorBelief::of(&with_clues(
            3,
            vec![
                (2, ClueKind::Spotted { at: Boston }),
                (1, ClueKind::SeaAnnouncements),
            ],
        ));
        assert!(!belief.contains(MassachusettsBay));
    }

    #[test]
    fn the_flying_horror_widens_the_turn() {
        let clues = vec![
            (1, ClueKind::Spotted { at: Nashua }),
            (2, ClueKind::FlyingHorror),
        ];
        let belief = ProfessorBelief::of(&with_clues(2, clues));
        assert!(belief.contains(Concord) && belief.contains(Lawrence));
        assert!(!belief.contains(Worcester));
    }

    #[test]
    fn weights_shy_away_from_armed_investigators() {
        let mut state = with_clues(2, vec![(1, ClueKind::Spotted { at: Nashua })]);
        state.investigators[0].weapon = true;
        state.investigators[0].location = Lawrence;
        let weights = ProfessorBelief::of(&state).weights(&state);
        assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!(weights[Dunwich.index()] > weights[Lowell.index()]);
        assert_eq!(weights[Worcester.index()], 0.0);
    }

    #[test]
    fn a_retreat_leaves_the_neighbours_of_the_fight() {
        let clues = vec![
            (1, ClueKind::Spotted { at: Lynn }),
            (1, ClueKind::Retreated { from: Lynn }),
        ];
        let belief = ProfessorBelief::of(&with_clues(1, clues));
        assert_eq!(belief.locations(), vec![Arkham, Boston, MassachusettsBay]);
    }
}
