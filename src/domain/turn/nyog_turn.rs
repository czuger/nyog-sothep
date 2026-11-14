use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep;
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
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

    /// Nyog'Sothep and the professor move together: nothing when the rules say they rest, else
    /// one step (by road, or
    /// by sea like the professor), along the shortest path when spell holders have gathered.
    /// The professor picks the step.
    pub(super) fn nyog_move(&mut self) {
        let Some(from) = self.state.nyog.location else {
            return;
        };
        if self.rules().nyog_rests(self) {
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
    /// and the spell holders among them may cast the spell.
    pub(super) fn nyog_enters(&mut self, city: LocationId, may_cast: bool) {
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
            self.rules().cast_banishment(self, city);
        }
    }

    fn meet_nyog(&mut self, investigator: usize) {
        let target = &mut self.state.investigators[investigator];
        if !target.sign {
            self.kill(investigator);
            return;
        }
        let loss =
            nyog_sothep::meeting_san_loss(target.seen_nyog) - target.protection.nyog_relief();
        target.seen_nyog = true;
        let name = target.id.name();
        self.log(format!(
            "{name} se trouve face à Nyog'Sothep et perd {loss} SAN."
        ));
        self.change_san(investigator, -loss);
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
}
