use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

/// How far the professor may stand to answer « yes » to [`ProfessorQuestion::WithinThreeCities`].
const WITHIN_STEPS: u8 = 3;

impl Engine<'_> {
    /// The professor answers an event's question. Facts come from the state itself; the cities
    /// he names are asked of him and checked, so he can never lie.
    pub(super) fn professor_answers(&mut self, investigator: usize, question: ProfessorQuestion) {
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
            ProfessorQuestion::WithinThreeCities => self.within_reach(investigator),
            ProfessorQuestion::CountFanaticsCities => {
                let count = self.state.counters.fanatics_cities().len();
                self.log(format!(
                    "Le professeur a des Fanatiques en place dans {count} ville(s)."
                ));
                self.clue(ClueKind::FanaticsCount { count });
            }
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

    /// He says whether he stands within three steps of the investigator.
    fn within_reach(&mut self, investigator: usize) {
        let from = self.state.investigators[investigator].location;
        let inside = travel::distance(self.state.professor.location, from, Network::All, |_| true)
            .is_some_and(|steps| steps <= WITHIN_STEPS);
        let kind = ClueKind::Within {
            from,
            steps: WITHIN_STEPS,
            inside,
        };
        self.log(format!("Le professeur répond : {}", kind.describe()));
        self.clue(kind);
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
        if self.state.professor.location.is_sea() {
            self.log("Le professeur répond qu'il est en mer.");
            self.clue(ClueKind::AtSea);
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::NameCities { count }),
            self.state.phase,
        );
    }

    fn name_fanatics_cities(&mut self) {
        if self.state.counters.fanatics_cities().is_empty() {
            self.log("Le professeur n'a aucun groupe de Fanatiques en place.");
            self.clue(ClueKind::NoFanatics);
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::NameFanaticsCities),
            self.state.phase,
        );
    }

    /// The cities the professor names: his position, or a Fanatics city, must be among them.
    pub(super) fn answer_professor_names(
        &mut self,
        prompt: &ProfessorPrompt,
        named: &[LocationId],
    ) -> Result<(), AnswerError> {
        let clue = match prompt {
            ProfessorPrompt::NameCities { count } => {
                if !valid_names(named, *count) {
                    return Err(AnswerError::WrongCityCount(*count));
                }
                if !named.contains(&self.state.professor.location) {
                    return Err(AnswerError::MustNameOwnCity);
                }
                ClueKind::NamedCities {
                    cities: named.to_vec(),
                }
            }
            ProfessorPrompt::NameFanaticsCities => {
                if !valid_names(named, 2) {
                    return Err(AnswerError::WrongCityCount(2));
                }
                let fanatics = self.state.counters.fanatics_cities();
                if !named.iter().any(|city| fanatics.contains(city)) {
                    return Err(AnswerError::MustNameFanaticsCity);
                }
                ClueKind::FanaticsAmong {
                    cities: named.to_vec(),
                }
            }
            _ => return Err(AnswerError::Unexpected),
        };
        self.state.prompt = None;
        self.log(format!("Le professeur répond : {}", clue.describe()));
        self.clue(clue);
        Ok(())
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
pub(super) fn with_nearest_cities(city: LocationId, count: usize) -> Vec<LocationId> {
    let found = travel::distances(city, Network::All, |_| true);
    let mut others: Vec<LocationId> = map::cities().filter(|other| *other != city).collect();
    others.sort_by_key(|other| found[other.index()].unwrap_or(u8::MAX));
    let mut named: Vec<LocationId> = others.into_iter().take(count.saturating_sub(1)).collect();
    named.push(city);
    named.sort();
    named
}
