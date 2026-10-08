use serde::{Deserialize, Serialize};

use crate::domain::counter::{Counter, CounterPlace};
use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;

/// Every counter of the box and where each one is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterPool {
    counters: Vec<Counter>,
}

impl CounterPool {
    /// A full box, every counter in the reserve.
    pub fn full() -> CounterPool {
        let counters = CounterKind::ALL
            .into_iter()
            .flat_map(|kind| {
                (0..kind.quantity()).map(move |_| Counter {
                    kind,
                    place: CounterPlace::Reserve,
                })
            })
            .collect();
        CounterPool { counters }
    }

    /// A pool rebuilt from stored counters.
    pub fn from_counters(counters: Vec<Counter>) -> CounterPool {
        CounterPool { counters }
    }

    /// Every counter.
    pub fn all(&self) -> &[Counter] {
        &self.counters
    }

    /// Whether the reserve holds a counter of this kind.
    pub fn in_reserve(&self, kind: CounterKind) -> bool {
        self.position(kind, CounterPlace::Reserve).is_some()
    }

    /// The professor's hand, sorted by kind.
    pub fn hand(&self) -> Vec<CounterKind> {
        let mut hand: Vec<CounterKind> = self
            .counters
            .iter()
            .filter(|counter| counter.place == CounterPlace::Hand)
            .map(|counter| counter.kind)
            .collect();
        hand.sort();
        hand
    }

    /// The counter placed in a city, hidden or discovered.
    pub fn placed_at(&self, city: LocationId) -> Option<Counter> {
        self.counters
            .iter()
            .find(|counter| counter.city() == Some(city))
            .copied()
    }

    /// Placed counters with their city.
    pub fn placed(&self) -> impl Iterator<Item = (LocationId, Counter)> + '_ {
        self.counters
            .iter()
            .filter_map(|counter| counter.city().map(|city| (city, *counter)))
    }

    /// Cities holding a Fanatics counter, in board order.
    pub fn fanatics_cities(&self) -> Vec<LocationId> {
        let mut cities: Vec<LocationId> = self
            .placed()
            .filter(|(_, counter)| counter.kind == CounterKind::Fanatics)
            .map(|(city, _)| city)
            .collect();
        cities.sort();
        cities
    }

    /// Moves a counter of this kind from the reserve to the hand.
    ///
    /// # Returns
    /// `false` when the reserve has none.
    pub fn draw(&mut self, kind: CounterKind) -> bool {
        self.change(kind, CounterPlace::Reserve, CounterPlace::Hand)
    }

    /// Puts a counter of this kind from the hand back into the reserve.
    pub fn discard(&mut self, kind: CounterKind) -> bool {
        self.change(kind, CounterPlace::Hand, CounterPlace::Reserve)
    }

    /// Takes a counter of this kind out of the hand for good (used Flying Horror or
    /// Teleportation).
    pub fn spend(&mut self, kind: CounterKind) -> bool {
        self.change(kind, CounterPlace::Hand, CounterPlace::Removed)
    }

    /// Places a counter of this kind from the hand, hidden, in a city.
    pub fn place(&mut self, kind: CounterKind, city: LocationId) -> bool {
        let placed = CounterPlace::Placed {
            city,
            discovered: false,
        };
        self.change(kind, CounterPlace::Hand, placed)
    }

    /// Puts a Fanatics counter face up in a city, from the reserve or else from the hand.
    ///
    /// # Returns
    /// `false` when every Fanatics counter is already on the board or out of the game.
    pub fn raise_fanatics(&mut self, city: LocationId) -> bool {
        let placed = CounterPlace::Placed {
            city,
            discovered: true,
        };
        self.change(CounterKind::Fanatics, CounterPlace::Reserve, placed)
            || self.change(CounterKind::Fanatics, CounterPlace::Hand, placed)
    }

    /// Changes the place of the counter standing in a city.
    pub fn set_place_at(&mut self, city: LocationId, place: CounterPlace) {
        if let Some(counter) = self
            .counters
            .iter_mut()
            .find(|counter| counter.city() == Some(city))
        {
            counter.place = place;
        }
    }

    /// Turns every placed Fanatics counter face up.
    pub fn reveal_fanatics(&mut self) {
        for counter in &mut self.counters {
            if let CounterPlace::Placed { city, .. } = counter.place
                && counter.kind == CounterKind::Fanatics
            {
                counter.place = CounterPlace::Placed {
                    city,
                    discovered: true,
                };
            }
        }
    }

    fn position(&self, kind: CounterKind, place: CounterPlace) -> Option<usize> {
        self.counters
            .iter()
            .position(|counter| counter.kind == kind && counter.place == place)
    }

    fn change(&mut self, kind: CounterKind, from: CounterPlace, to: CounterPlace) -> bool {
        match self.position(kind, from) {
            Some(index) => {
                self.counters[index].place = to;
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_move_between_reserve_hand_and_board() {
        let mut pool = CounterPool::full();
        assert!(pool.draw(CounterKind::FlyingHorror));
        assert!(!pool.draw(CounterKind::FlyingHorror));
        assert!(pool.draw(CounterKind::Fanatics));
        assert_eq!(
            pool.hand(),
            vec![CounterKind::Fanatics, CounterKind::FlyingHorror]
        );
        assert!(pool.place(CounterKind::Fanatics, LocationId::Boston));
        assert_eq!(pool.fanatics_cities(), vec![LocationId::Boston]);
        assert!(pool.spend(CounterKind::FlyingHorror));
        assert!(pool.hand().is_empty());
        pool.set_place_at(LocationId::Boston, CounterPlace::Removed);
        assert!(pool.placed_at(LocationId::Boston).is_none());
    }

    #[test]
    fn mad_investigators_raise_fanatics_face_up() {
        let mut pool = CounterPool::full();
        assert!(pool.raise_fanatics(LocationId::Lynn));
        let counter = pool.placed_at(LocationId::Lynn).unwrap();
        assert_eq!(
            counter.place,
            CounterPlace::Placed {
                city: LocationId::Lynn,
                discovered: true
            }
        );
    }
}
