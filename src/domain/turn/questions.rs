use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::{ActionStep, Phase};

impl Engine<'_> {
    /// The professor answers an event's question. Facts come from the state itself; his bluffs
    /// are checked, so he can never lie.
    pub(super) fn professor_answers(&mut self, question: ProfessorQuestion) {
        match question {
            ProfessorQuestion::NameCities(count) => self.name_cities(usize::from(count)),
            ProfessorQuestion::RevealPosition => self.reveal_position(),
            ProfessorQuestion::RevealPositionAndFanatics => {
                self.reveal_position();
                let count = self
                    .state
                    .counters
                    .placed()
                    .filter(|(_, counter)| counter.kind == crate::domain::CounterKind::Fanatics)
                    .count();
                self.log(format!("Il a {count} groupe(s) de Fanatiques en place."));
                self.clue(ClueKind::FanaticsCount { count });
            }
            ProfessorQuestion::RevealAndMoveInTheOpen => {
                self.reveal_position();
                self.state.move_in_the_open = true;
                self.clue(ClueKind::MoveInTheOpen);
            }
            ProfessorQuestion::AnnounceSeaMoves => {
                self.state.sea_announcements = true;
                self.clue(ClueKind::SeaAnnouncements);
            }
            ProfessorQuestion::NameFanaticsCities => self.name_fanatics_cities(),
            ProfessorQuestion::EncountersInTwoCities
            | ProfessorQuestion::NyogSothepInTwoCities { .. }
            | ProfessorQuestion::EncounterInOneCity => {}
        }
    }

    pub(super) fn answer_cities(
        &mut self,
        investigator: usize,
        question: ProfessorQuestion,
        cities: &[LocationId],
    ) -> Result<(), AnswerError> {
        let count = question.cities_named_by_investigator().unwrap_or(0);
        let optional = matches!(
            question,
            ProfessorQuestion::NyogSothepInTwoCities { optional: true }
        );
        let mut distinct = cities.to_vec();
        distinct.sort();
        distinct.dedup();
        if !(optional && cities.is_empty()) && (distinct.len() != count || cities.len() != count) {
            return Err(AnswerError::WrongCityCount(count));
        }
        if cities.iter().any(|city| !city.is_city()) {
            return Err(AnswerError::NotACity);
        }
        let plays_again = matches!(
            self.state.phase,
            Phase::Action {
                step: ActionStep::Question { plays_again: true },
                ..
            }
        );
        self.state.prompt = None;
        self.answer_named(question, cities);
        self.after_event(investigator, plays_again);
        Ok(())
    }

    /// The professor answers about the cities the investigator named.
    fn answer_named(&mut self, question: ProfessorQuestion, cities: &[LocationId]) {
        match question {
            ProfessorQuestion::EncountersInTwoCities => {
                for city in cities {
                    let present = self.state.counters.placed_at(*city).is_some();
                    let kind = ClueKind::Encounter {
                        city: *city,
                        present,
                    };
                    self.log(kind.describe());
                    self.clue(kind);
                }
            }
            ProfessorQuestion::NyogSothepInTwoCities { .. } if !cities.is_empty() => {
                let present = cities.contains(&self.state.nyog.city);
                let kind = ClueKind::NyogSothepAmong {
                    cities: cities.to_vec(),
                    present,
                };
                self.log(kind.describe());
                self.clue(kind);
            }
            ProfessorQuestion::EncounterInOneCity => {
                for city in cities {
                    let kind = self
                        .state
                        .counters
                        .placed_at(*city)
                        .map(|counter| counter.kind);
                    let clue = ClueKind::EncounterKind { city: *city, kind };
                    self.log(clue.describe());
                    self.clue(clue);
                }
            }
            _ => {}
        }
    }

    fn reveal_position(&mut self) {
        let at = self.state.professor.location;
        self.log(format!(
            "Le professeur révèle sa position : il est {}.",
            at.at_phrase()
        ));
        self.clue(ClueKind::Spotted { at });
    }

    fn name_cities(&mut self, count: usize) {
        let at = self.state.professor.location;
        if at.is_sea() {
            self.log("Le professeur répond qu'il est en mer.");
            self.clue(ClueKind::AtSea);
            return;
        }
        let named = self.brain.name_cities(self.state, count);
        let named = match valid_names(&named, count) && named.contains(&at) {
            true => named,
            false => with_nearest_cities(at, count),
        };
        let clue = ClueKind::NamedCities { cities: named };
        self.log(format!("Le professeur répond : {}", clue.describe()));
        self.clue(clue);
    }

    fn name_fanatics_cities(&mut self) {
        let fanatics = self.state.counters.fanatics_cities();
        let Some(first) = fanatics.first().copied() else {
            self.log("Le professeur n'a aucun groupe de Fanatiques en place.");
            self.clue(ClueKind::NoFanatics);
            return;
        };
        let named = self.brain.name_fanatics_cities(self.state);
        let named = match valid_names(&named, 2) && named.iter().any(|city| fanatics.contains(city))
        {
            true => named,
            false => with_nearest_cities(first, 2),
        };
        let clue = ClueKind::FanaticsAmong { cities: named };
        self.log(format!("Le professeur répond : {}", clue.describe()));
        self.clue(clue);
    }
}

/// Whether a list holds exactly `count` distinct cities.
fn valid_names(named: &[LocationId], count: usize) -> bool {
    let mut distinct = named.to_vec();
    distinct.sort();
    distinct.dedup();
    named.len() == count && distinct.len() == count && named.iter().all(|city| city.is_city())
}

/// `city` and the cities nearest to it, `count` in all, in board order.
fn with_nearest_cities(city: LocationId, count: usize) -> Vec<LocationId> {
    let found = travel::distances(city, Network::All, |_| true);
    let mut others: Vec<LocationId> = map::cities().filter(|other| *other != city).collect();
    others.sort_by_key(|other| found[other.index()].unwrap_or(u8::MAX));
    let mut named: Vec<LocationId> = others.into_iter().take(count.saturating_sub(1)).collect();
    named.push(city);
    named.sort();
    named
}
