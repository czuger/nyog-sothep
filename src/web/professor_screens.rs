//! What the professor's screens show: the choices of his pending prompt.

use crate::domain::map;
use crate::domain::turn::{ProfessorPrompt, Prompt};
use crate::domain::{CounterKind, LocationId, SUMMONING_FANATICS_CITIES};
use crate::web::game_page::{Choice, CounterChoice, GamePage};

impl GamePage {
    /// The professor's pending prompt.
    pub fn professor_prompt(&self) -> Option<&ProfessorPrompt> {
        match &self.view.prompt {
            Some(Prompt::Professor(prompt)) => Some(prompt),
            _ => None,
        }
    }

    /// Every city, by name, for Nyog'Sothep's city at the setup.
    pub fn city_choices(&self) -> Vec<Choice> {
        let mut cities: Vec<LocationId> = map::cities().collect();
        cities.sort_by_key(|city| city.to_string());
        cities
            .into_iter()
            .map(|city| Choice {
                value: city.code().to_string(),
                label: city.to_string(),
            })
            .collect()
    }

    /// The Teleportations he may cast: `{investigator}:{city}` values.
    pub fn teleport_choices(&self) -> Vec<Choice> {
        let Some(ProfessorPrompt::Teleport { options }) = self.professor_prompt() else {
            return Vec::new();
        };
        options
            .iter()
            .map(|teleport| {
                let name = self.investigator_name(teleport.investigator);
                Choice {
                    value: format!("{}:{}", teleport.investigator, teleport.destination.code()),
                    label: format!("{name} vers {}", teleport.destination),
                }
            })
            .collect()
    }

    /// His moves: staying, the ordinary steps, then the Flying Horror's destinations.
    pub fn destination_choices(&self) -> Vec<Choice> {
        let Some(ProfessorPrompt::Movement { options }) = self.professor_prompt() else {
            return Vec::new();
        };
        let mut choices: Vec<Choice> = options
            .steps
            .iter()
            .enumerate()
            .map(|(index, to)| Choice {
                value: to.code().to_string(),
                label: match index {
                    0 => format!("Rester {}", to.at_phrase()),
                    _ => format!("Aller {}", to.at_phrase()),
                },
            })
            .collect();
        choices.extend(
            options
                .flying
                .iter()
                .filter(|to| !options.steps.contains(to))
                .map(|to| Choice {
                    value: to.code().to_string(),
                    label: format!("Horreur volante : {to}"),
                }),
        );
        choices
    }

    /// The investigators he may attack on arrival.
    pub fn attack_choices(&self) -> Vec<Choice> {
        let Some(ProfessorPrompt::Attack { candidates, .. }) = self.professor_prompt() else {
            return Vec::new();
        };
        candidates
            .iter()
            .map(|index| Choice {
                value: index.to_string(),
                label: format!("Attaquer {}", self.investigator_name(*index)),
            })
            .collect()
    }

    /// The die made him reveal himself: he must attack.
    pub fn attack_forced(&self) -> bool {
        matches!(
            self.professor_prompt(),
            Some(ProfessorPrompt::Attack { forced: true, .. })
        )
    }

    /// What to pick on the map for a carry, a retreat or Nyog'Sothep's step.
    pub fn location_text(&self) -> String {
        match self.professor_prompt() {
            Some(ProfessorPrompt::Carry { .. }) => format!(
                "{} n'avait rien pour se défendre : choisissez sur la carte où vous l'emmenez, à deux villes de là.",
                self.prompted_name()
            ),
            Some(ProfessorPrompt::Relocate {
                pushed_back: true, ..
            }) => "Le signe protecteur vous repousse : choisissez sur la carte où vous reculez d'un cran.".to_string(),
            Some(ProfessorPrompt::Relocate { .. }) => {
                "Blessé, vous battez en retraite : choisissez sur la carte où vous reculez d'un cran.".to_string()
            }
            _ => "Le dé fait avancer Nyog'Sothep : choisissez sur la carte la ville où vous l'emmenez avec vous.".to_string(),
        }
    }

    /// The counters of his pending prompt: to place, to keep from Table 1, or to discard.
    pub fn counter_choices(&self) -> Vec<CounterChoice> {
        let kinds: &[CounterKind] = match self.professor_prompt() {
            Some(ProfessorPrompt::Placement { options } | ProfessorPrompt::Table1 { options }) => {
                options
            }
            Some(ProfessorPrompt::Discard { hand }) => hand,
            _ => &[],
        };
        let mut kinds = kinds.to_vec();
        kinds.dedup();
        kinds.into_iter().map(counter_choice).collect()
    }

    /// His hand, for the banner.
    pub fn hand_counters(&self) -> Vec<CounterChoice> {
        self.view.hand.iter().copied().map(counter_choice).collect()
    }

    /// How many cities he must name, and why.
    pub fn names_question(&self) -> (usize, &'static str) {
        match self.professor_prompt() {
            Some(ProfessorPrompt::NameCities { count }) => (
                *count,
                "Un investigateur vous oblige à citer des villes : votre position doit en faire partie.",
            ),
            _ => (
                2,
                "Un investigateur vous oblige à citer deux villes : l'une d'elles doit abriter des Fanatiques.",
            ),
        }
    }

    /// French summary of his progress towards the summoning.
    pub fn summoning_status(&self) -> String {
        let mut cities: Vec<LocationId> = self
            .view
            .counters
            .iter()
            .filter(|counter| counter.kind == CounterKind::Fanatics)
            .map(|counter| counter.city)
            .collect();
        cities.dedup();
        let count = cities.len();
        match self.view.can_summon {
            true => format!(
                "Fanatiques dans {count} villes sur {SUMMONING_FANATICS_CITIES} : vous pouvez invoquer Nyog'Sothep."
            ),
            false => format!("Fanatiques dans {count} villes sur {SUMMONING_FANATICS_CITIES}."),
        }
    }

    fn investigator_name(&self, index: usize) -> &'static str {
        self.view
            .investigators
            .get(index)
            .map_or("", |investigator| investigator.id.name())
    }
}

fn counter_choice(kind: CounterKind) -> CounterChoice {
    CounterChoice {
        value: kind.code(),
        label: kind.name(),
        image: format!("/img/token-{}.png", kind.code()),
    }
}
