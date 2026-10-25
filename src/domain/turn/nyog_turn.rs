use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{self, BANISHERS_NEEDED, BanishmentResult};
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor_answers::pick;
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

impl Engine<'_> {
    /// The professor reveals himself and his Fanatics, and Nyog'Sothep appears in its city.
    pub(super) fn summon(&mut self) {
        let city = self.state.nyog.city;
        let professor = self.state.professor.location;
        self.state.counters.reveal_fanatics();
        self.log(format!(
            "Le professeur Petersen invoque Nyog'Sothep ! Il se révèle {}, montre ses notes d'étape et le papier où il avait écrit « {city} » : l'Immonde Brumeux y apparaît dans un épais brouillard.",
            professor.at_phrase()
        ));
        self.clue(ClueKind::Spotted { at: professor });
        self.clue(ClueKind::Summoned { city });
        self.state.nyog.location = Some(city);
        self.state.nyog.joined = professor == city;
        self.check_end();
        if !self.state.is_finished() {
            self.nyog_enters(city, true);
        }
    }

    /// Nyog'Sothep and the professor move together: nothing on 1 or 2, else one step (by road, or
    /// by sea like the professor), along the shortest path when spell holders have gathered.
    /// The professor picks the step.
    pub(super) fn nyog_move(&mut self) {
        let Some(from) = self.state.nyog.location else {
            return;
        };
        if self.dice.d6() <= 2 {
            self.log("Le professeur et Nyog'Sothep ne bougent pas ce tour-ci.");
            return;
        }
        let mut options = travel::neighbours(from).to_vec();
        if let Some(target) = self.state.nyog.target {
            let path = travel::first_steps_towards(from, target, Network::All, |_| true);
            if !path.is_empty() {
                options = path;
            }
        }
        if options.is_empty() {
            self.log("Le professeur et Nyog'Sothep ne bougent pas ce tour-ci.");
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::NyogStep { options }),
            self.state.phase,
        );
    }

    pub(super) fn answer_nyog_step(
        &mut self,
        options: &[LocationId],
        picked: LocationId,
    ) -> Result<(), AnswerError> {
        let to = pick(options, picked)?;
        self.state.prompt = None;
        self.state.professor.location = to;
        self.state.nyog.location = Some(to);
        self.log(format!(
            "Nyog'Sothep et le professeur avancent {}.",
            to.at_phrase()
        ));
        self.clue(ClueKind::Spotted { at: to });
        self.nyog_enters(to, true);
        Ok(())
    }

    /// Nyog'Sothep enters a city: it is destroyed, the investigators there meet the Foul One,
    /// and three spell holders among them cast the spell.
    fn nyog_enters(&mut self, city: LocationId, may_cast: bool) {
        if city.is_sea() {
            return;
        }
        if !self.state.is_destroyed(city) {
            self.state.destroyed.push(city);
            self.log(format!("{city} est détruite."));
        }
        for investigator in self.state.investigators_at(city) {
            self.meet_nyog(investigator);
        }
        self.check_end();
        if may_cast && !self.state.is_finished() {
            self.try_banishment(city);
        }
    }

    fn meet_nyog(&mut self, investigator: usize) {
        let target = &mut self.state.investigators[investigator];
        if !target.sign {
            self.kill(investigator);
            return;
        }
        let loss = nyog_sothep::meeting_san_loss(target.seen_nyog);
        target.seen_nyog = true;
        let name = target.id.name();
        self.log(format!(
            "{name} se trouve face à Nyog'Sothep et perd {loss} SAN."
        ));
        self.change_san(investigator, -loss);
    }

    fn try_banishment(&mut self, city: LocationId) {
        let casters: Vec<usize> = self
            .state
            .investigators_at(city)
            .into_iter()
            .filter(|index| self.state.investigators[*index].spell)
            .collect();
        if casters.len() < BANISHERS_NEEDED {
            return;
        }
        let roll = self.dice.d6();
        let result = nyog_sothep::banishment(roll, casters.len());
        self.log(format!(
            "{} investigateurs lancent le sort de renvoi : {roll} au dé, moins {} participants.",
            casters.len(),
            casters.len()
        ));
        match result {
            BanishmentResult::Banished => {
                for caster in casters {
                    self.state.investigators[caster].banished = true;
                }
                self.finish(Outcome::Banished);
            }
            BanishmentResult::Displaced => {
                self.log("Le sort agit en partie : les investigateurs choisissent la ville où envoyer Nyog'Sothep.");
                self.ask(Prompt::BanishmentCity, Phase::Banishment);
            }
            BanishmentResult::Fails => self.log("Le sort échoue : Nyog'Sothep reste libre."),
            BanishmentResult::Disaster | BanishmentResult::Backlash => {
                let loss = result.san_loss();
                self.log(format!(
                    "Le sort se retourne contre eux : chaque participant perd {loss} SAN, et Nyog'Sothep reste libre."
                ));
                for caster in casters {
                    self.change_san(caster, -loss);
                }
            }
        }
    }

    pub(super) fn answer_banishment_city(&mut self, city: LocationId) -> Result<(), AnswerError> {
        if !city.is_city() {
            return Err(AnswerError::NotACity);
        }
        self.state.prompt = None;
        self.state.phase = Phase::ProfessorPlace;
        self.state.nyog.location = Some(city);
        self.state.nyog.joined = self.state.professor.location == city;
        self.state.nyog.target = None;
        self.log(format!("Nyog'Sothep est envoyé {}.", city.at_phrase()));
        self.nyog_enters(city, false);
        Ok(())
    }

    /// Three spell holders gathered in a city become Nyog'Sothep's only goal.
    pub(super) fn update_banishment_target(&mut self) {
        if !self.state.nyog.is_summoned() {
            return;
        }
        let target = self
            .state
            .active_investigators()
            .into_iter()
            .map(|index| self.state.investigators[index].location)
            .find(|location| {
                location.is_city()
                    && self
                        .state
                        .investigators_at(*location)
                        .into_iter()
                        .filter(|index| self.state.investigators[*index].spell)
                        .count()
                        >= BANISHERS_NEEDED
            });
        if target.is_some()
            && target != self.state.nyog.target
            && let Some(city) = target
        {
            self.log(format!(
                "Trois investigateurs connaissant le sort de renvoi sont réunis à {city} : Nyog'Sothep et le professeur fondent sur eux."
            ));
        }
        self.state.nyog.target = target;
    }
}
