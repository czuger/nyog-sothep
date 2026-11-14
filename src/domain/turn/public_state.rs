use crate::domain::investigator::STARTING_CITY;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::game_state::GameState;

impl GameState {
    /// The state as the investigators know it: the professor's hand, hidden counters and route
    /// sheet are gone, he stands where he was last seen, and Nyog'Sothep's city is a placeholder
    /// until it appears.
    pub fn public_copy(&self) -> GameState {
        let mut public = self.clone();
        public.counters = self.counters.without_secrets();
        public.route_sheet.clear();
        if !self.nyog.is_summoned() {
            public.nyog.city = STARTING_CITY;
            public.professor.location = self
                .clues
                .iter()
                .rev()
                .find_map(|clue| match clue.kind {
                    ClueKind::Spotted { at } => Some(at),
                    _ => None,
                })
                .unwrap_or(STARTING_CITY);
        }
        public
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::professor_ai::test_state;
    use crate::domain::{CounterKind, LocationId};

    #[test]
    fn the_secrets_do_not_reach_the_investigators() {
        let mut first = test_state();
        assert!(first.counters.draw(CounterKind::Ghouls));
        assert!(first.counters.place(CounterKind::Ghouls, LocationId::Lynn));
        assert!(first.counters.draw(CounterKind::FlyingHorror));
        let mut second = test_state();
        second.professor.location = LocationId::Newport;
        second.nyog.city = LocationId::Arkham;
        assert!(second.counters.draw(CounterKind::Storm));
        assert_eq!(first.public_copy(), second.public_copy());
        assert!(first.public_copy().counters.hand().is_empty());
        assert!(
            first
                .public_copy()
                .counters
                .placed_at(LocationId::Lynn)
                .is_none()
        );
    }
}
