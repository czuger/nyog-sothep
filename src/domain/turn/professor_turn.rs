use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::location_id::LocationId;
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::fights::FightContext;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor_answers::pick;
use crate::domain::turn::professor_brain::{ProfessorMoveOptions, Teleport};
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

/// The farthest the Flying Horror carries the professor.
const FLYING_HORROR_RANGE: u8 = 3;
/// The farthest an investigator may stand from the professor to be teleported.
const TELEPORT_RANGE: u8 = 2;

impl Engine<'_> {
    /// Opens a round: Nyog'Sothep's move once the professor has joined it, otherwise the offer
    /// of a Teleportation before he moves.
    pub(super) fn professor_move(&mut self) {
        self.state.turn += 1;
        if self.state.nyog.joined {
            self.state.phase = Phase::ProfessorPlace;
            self.nyog_move();
            return;
        }
        self.state.phase = Phase::ProfessorMovement;
        if !self
            .state
            .counters
            .hand()
            .contains(&CounterKind::Teleportation)
        {
            return;
        }
        let options = self.teleport_options();
        if !options.is_empty() {
            self.ask(
                Prompt::Professor(ProfessorPrompt::Teleport { options }),
                Phase::ProfessorMovement,
            );
        }
    }

    pub(super) fn answer_teleport(
        &mut self,
        options: &[Teleport],
        picked: Option<Teleport>,
    ) -> Result<(), AnswerError> {
        let picked = picked.map(|teleport| pick(options, teleport)).transpose()?;
        self.state.prompt = None;
        let Some(teleport) = picked else {
            return Ok(());
        };
        self.state.counters.spend(CounterKind::Teleportation);
        let target = &mut self.state.investigators[teleport.investigator];
        let near = target.location;
        target.go_to(teleport.destination);
        let (name, agreement) = (target.id.name(), target.id.agreement());
        self.log(format!(
            "Le professeur lance une Téléportation : {name} est transporté{agreement} {} et perd 2 SAN.",
            teleport.destination.at_phrase()
        ));
        self.clue(ClueKind::Teleported { near });
        self.change_san(teleport.investigator, -2);
        Ok(())
    }

    /// Every Teleportation the professor could cast now.
    fn teleport_options(&self) -> Vec<Teleport> {
        let state = &*self.state;
        let near = travel::distances(state.professor.location, Network::All, |_| true);
        let passable = |city: LocationId| !state.is_destroyed(city);
        state
            .active_investigators()
            .into_iter()
            .filter(|index| {
                let location = state.investigators[*index].location;
                location.is_city() && near[location.index()].is_some_and(|d| d <= TELEPORT_RANGE)
            })
            .flat_map(|investigator| {
                let from = state.investigators[investigator].location;
                travel::within(from, 1, 2, Network::Roads, passable)
                    .into_iter()
                    .map(move |destination| Teleport {
                        investigator,
                        destination,
                    })
            })
            .collect()
    }

    /// Asks for the professor's move: staying, one step, or the Flying Horror.
    pub(super) fn professor_movement(&mut self) {
        let from = self.state.professor.location;
        let mut steps = vec![from];
        steps.extend(travel::neighbours(from));
        let flying = match self
            .state
            .counters
            .hand()
            .contains(&CounterKind::FlyingHorror)
        {
            true => travel::within(from, 1, FLYING_HORROR_RANGE, Network::All, |_| true),
            false => Vec::new(),
        };
        let options = ProfessorMoveOptions { steps, flying };
        self.ask(
            Prompt::Professor(ProfessorPrompt::Movement { options }),
            Phase::ProfessorSummon,
        );
    }

    /// Moves the professor (with the Flying Horror only when the destination is not an ordinary
    /// step), then he may meet investigators on arrival.
    pub(super) fn answer_movement(
        &mut self,
        options: &ProfessorMoveOptions,
        destination: LocationId,
    ) -> Result<(), AnswerError> {
        let flying = !options.steps.contains(&destination);
        if flying && !options.flying.contains(&destination) {
            return Err(AnswerError::NotAnOption);
        }
        self.state.prompt = None;
        if flying {
            self.state.counters.spend(CounterKind::FlyingHorror);
            self.log("Le professeur utilise l'Horreur volante !");
            self.clue(ClueKind::FlyingHorror);
        }
        let from = self.state.professor.location;
        self.state.professor.location = destination;
        self.announce_move(from, destination);
        if self.state.nyog.location == Some(destination) && self.state.nyog.is_summoned() {
            self.state.nyog.joined = true;
            self.log(format!(
                "Le professeur rejoint Nyog'Sothep {}.",
                destination.at_phrase()
            ));
        }
        if destination != from {
            self.meet_on_arrival(destination);
        }
        Ok(())
    }

    /// What the investigators learn of the professor's move.
    fn announce_move(&mut self, from: LocationId, destination: LocationId) {
        let at = destination.at_phrase();
        let revealed = match destination == from {
            true => format!("Le professeur reste en secret {at}."),
            false => format!("Le professeur se rend en secret {at}."),
        };
        if self.state.nyog.is_summoned() {
            self.log(format!("Le professeur se rend {at}."));
            self.clue(ClueKind::Spotted { at: destination });
        } else if self.state.move_in_the_open {
            self.state.move_in_the_open = false;
            self.log(format!(
                "Le professeur se déplace à découvert : il est {at}."
            ));
            self.clue(ClueKind::Spotted { at: destination });
        } else if self.state.sea_announcements && destination.is_sea() {
            self.log_revealing("Le professeur annonce qu'il se déplace en mer.", revealed);
            self.clue(ClueKind::AtSea);
        } else {
            self.log_revealing("Le professeur se déplace en secret.", revealed);
        }
    }

    /// The professor arrives in a city holding investigators: he announces it, and may (or must,
    /// on 1 or 2) reveal himself and attack one of them.
    fn meet_on_arrival(&mut self, at: LocationId) {
        let candidates = self.state.investigators_at(at);
        if !at.is_city() || candidates.is_empty() {
            return;
        }
        let mut cities: Vec<LocationId> = self
            .state
            .active_investigators()
            .into_iter()
            .map(|index| self.state.investigators[index].location)
            .filter(|location| location.is_city())
            .collect();
        cities.sort();
        cities.dedup();
        self.log("Le professeur annonce qu'il vient d'arriver dans une ville où se trouve un investigateur.");
        self.clue(ClueKind::MetInvestigator { cities });
        if self.state.nyog.is_summoned() {
            return;
        }
        let forced = self.dice.d6() <= 2;
        self.ask(
            Prompt::Professor(ProfessorPrompt::Attack { candidates, forced }),
            Phase::ProfessorSummon,
        );
    }

    pub(super) fn answer_attack(
        &mut self,
        candidates: &[usize],
        forced: bool,
        picked: Option<usize>,
    ) -> Result<(), AnswerError> {
        let target = match picked {
            Some(index) => pick(candidates, index)?,
            None if forced => return Err(AnswerError::MustAttack),
            None => {
                self.state.prompt = None;
                return Ok(());
            }
        };
        self.state.prompt = None;
        let at = self.state.professor.location;
        let name = self.state.investigators[target].id.inner_name();
        self.log(format!(
            "Le professeur se révèle {} et attaque {name} !",
            at.at_phrase()
        ));
        self.start_fight(target, FightContext::ProfessorArrival);
        Ok(())
    }

    /// Offers to summon Nyog'Sothep when the professor can.
    pub(super) fn professor_summon(&mut self) {
        self.state.phase = Phase::ProfessorPlace;
        if self.state.can_summon() {
            self.ask(
                Prompt::Professor(ProfessorPrompt::Summon),
                Phase::ProfessorPlace,
            );
        }
    }

    pub(super) fn answer_summon(&mut self, summons: bool) {
        self.state.prompt = None;
        if summons {
            self.summon();
        }
    }
}
