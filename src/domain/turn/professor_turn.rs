use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{SUMMONING_FANATICS_CITIES, SUMMONING_RANGE};
use crate::domain::table1::{self, HAND_LIMIT};
use crate::domain::travel::{self, Network};
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::fights::FightContext;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor::RouteStage;
use crate::domain::turn::professor_brain::{ProfessorMoveOptions, Teleport};

/// The farthest the Flying Horror carries the professor.
const FLYING_HORROR_RANGE: u8 = 3;
/// The farthest an investigator may stand from the professor to be teleported.
const TELEPORT_RANGE: u8 = 2;

impl Engine<'_> {
    /// Opens a round: Teleportation, then the professor's move (or Nyog'Sothep's), then a
    /// possible fight with an investigator met on arrival.
    pub(super) fn professor_move(&mut self) {
        self.state.turn += 1;
        if self.state.nyog.joined {
            self.state.phase = Phase::ProfessorPlace;
            self.nyog_move();
            return;
        }
        self.state.phase = Phase::ProfessorSummon;
        self.cast_teleportation();
        if self.state.is_finished() {
            return;
        }
        let from = self.state.professor.location;
        self.move_professor();
        let at = self.state.professor.location;
        if self.state.nyog.location == Some(at) && self.state.nyog.is_summoned() {
            self.state.nyog.joined = true;
            self.log(format!(
                "Le professeur rejoint Nyog'Sothep {}.",
                at.at_phrase()
            ));
        }
        if at != from {
            self.meet_on_arrival(at);
        }
    }

    /// Summons Nyog'Sothep when the professor can and wants to.
    pub(super) fn professor_summon(&mut self) {
        self.state.phase = Phase::ProfessorPlace;
        if self.can_summon() && self.brain.summon(self.state) {
            self.summon();
        }
    }

    /// Whether the summoning conditions are met.
    pub(super) fn can_summon(&self) -> bool {
        let state = &*self.state;
        !state.nyog.is_summoned()
            && state.counters.fanatics_cities().len() >= SUMMONING_FANATICS_CITIES
            && travel::distance(
                state.professor.location,
                state.nyog.city,
                Network::All,
                |_| true,
            )
            .is_some_and(|steps| steps <= SUMMONING_RANGE)
    }

    /// The professor may place a counter where he stands, then writes his stage note.
    pub(super) fn professor_place(&mut self) {
        self.state.phase = Phase::ProfessorDraw;
        let location = self.state.professor.location;
        let options = self.placement_options(location);
        let counter = match options.is_empty() {
            true => None,
            false => self
                .brain
                .placement(self.state, &options)
                .filter(|kind| options.contains(kind)),
        };
        if let Some(kind) = counter {
            self.state.counters.place(kind, location);
        }
        self.state.route_sheet.push(RouteStage {
            turn: self.state.turn,
            location,
            counter,
        });
    }

    /// The counters of the hand that may be placed in a location.
    pub(super) fn placement_options(&self, location: LocationId) -> Vec<CounterKind> {
        if !location.is_city() || self.state.counters.placed_at(location).is_some() {
            return Vec::new();
        }
        let mut options = self.state.counters.hand();
        options.retain(|kind| kind.is_placeable() && (!kind.port_only() || location.is_port()));
        options.dedup();
        options
    }

    /// The professor draws on Table 1, then discards down to five counters.
    pub(super) fn professor_draw(&mut self) {
        self.state.phase = Phase::Moves;
        self.draw_counter();
    }

    /// One Table 1 draw, with the hand limit.
    pub(super) fn draw_counter(&mut self) {
        let Some(options) = table1::roll(&self.state.counters, self.dice) else {
            return;
        };
        let kind = match options.as_slice() {
            [only] => *only,
            _ => {
                let picked = self.brain.table1_choice(self.state, &options);
                if options.contains(&picked) {
                    picked
                } else {
                    options[0]
                }
            }
        };
        self.state.counters.draw(kind);
        let hand = self.state.counters.hand();
        if hand.len() > HAND_LIMIT {
            let picked = self.brain.discard(self.state, &hand);
            let discarded = if hand.contains(&picked) {
                picked
            } else {
                hand[0]
            };
            self.state.counters.discard(discarded);
        }
    }

    fn cast_teleportation(&mut self) {
        if !self
            .state
            .counters
            .hand()
            .contains(&CounterKind::Teleportation)
        {
            return;
        }
        let options = self.teleport_options();
        if options.is_empty() {
            return;
        }
        let Some(teleport) = self
            .brain
            .teleport(self.state, &options)
            .filter(|picked| options.contains(picked))
        else {
            return;
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

    fn move_professor(&mut self) {
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
        let chosen = self.brain.movement(self.state, &options);
        let destination = if chosen.flying_horror && options.flying.contains(&chosen.destination) {
            self.state.counters.spend(CounterKind::FlyingHorror);
            self.log("Le professeur utilise l'Horreur volante !");
            self.clue(ClueKind::FlyingHorror);
            chosen.destination
        } else if options.steps.contains(&chosen.destination) {
            chosen.destination
        } else {
            from
        };
        self.state.professor.location = destination;
        self.announce_move(destination);
    }

    /// What the investigators learn of the professor's move.
    fn announce_move(&mut self, destination: LocationId) {
        let at = destination.at_phrase();
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
            self.log("Le professeur annonce qu'il se déplace en mer.");
            self.clue(ClueKind::AtSea);
        } else {
            self.log("Le professeur se déplace en secret.");
        }
    }

    /// The professor arrives in a city holding investigators: he announces it, and may (or must,
    /// on 1 or 2) reveal himself and attack one of them.
    fn meet_on_arrival(&mut self, at: LocationId) {
        let here = self.state.investigators_at(at);
        if !at.is_city() || here.is_empty() {
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
        let picked = self.brain.attack(self.state, &here, forced);
        let target = match picked.filter(|index| here.contains(index)) {
            Some(index) => index,
            None if forced => here[0],
            None => return,
        };
        let name = self.state.investigators[target].id.inner_name();
        self.log(format!(
            "Le professeur se révèle {} et attaque {name} !",
            at.at_phrase()
        ));
        self.start_fight(target, FightContext::ProfessorArrival);
    }
}
