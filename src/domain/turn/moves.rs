use crate::domain::dice::Dice;
use crate::domain::location_id::LocationId;
use crate::domain::travel;
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::move_options::MoveOptions;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::prompt::{MoveOrder, Prompt};

impl Engine<'_> {
    /// Opens the investigators' movement: lost turns are counted, then the human moves the
    /// others.
    pub(super) fn start_moves(&mut self) {
        for index in 0..self.state.investigators.len() {
            let mover = &mut self.state.investigators[index];
            mover.skips_action = false;
            if mover.is_active() && mover.skip_turns > 0 {
                mover.skip_turns -= 1;
                mover.skips_action = true;
                let name = mover.id.name();
                self.log(format!("{name} perd son tour."));
            }
        }
        let anyone_moves = (0..self.state.investigators.len())
            .any(|index| MoveOptions::of(self.state, index, false).is_some());
        if anyone_moves {
            self.ask(Prompt::Moves, Phase::Moves);
        } else {
            self.state.phase = first_action();
        }
    }

    pub(super) fn answer_moves(
        &mut self,
        orders: &[(usize, MoveOrder)],
    ) -> Result<(), AnswerError> {
        let mut checked = Vec::new();
        for index in 0..self.state.investigators.len() {
            let Some(options) = MoveOptions::of(self.state, index, false) else {
                continue;
            };
            let name = self.state.investigators[index].id.name();
            let order = orders
                .iter()
                .find(|(investigator, _)| *investigator == index)
                .map(|(_, order)| *order)
                .ok_or(AnswerError::MissingMove(name))?;
            if !options.allows(order) {
                return Err(AnswerError::IllegalMove(name));
            }
            checked.push((index, order));
        }
        self.state.prompt = None;
        self.state.phase = first_action();
        for (index, order) in checked {
            self.apply_move(index, order);
        }
        Ok(())
    }

    pub(super) fn answer_play_again(
        &mut self,
        investigator: usize,
        order: MoveOrder,
    ) -> Result<(), AnswerError> {
        let name = self.state.investigators[investigator].id.name();
        let allowed = MoveOptions::of(self.state, investigator, true)
            .is_some_and(|options| options.allows(order));
        if !allowed {
            return Err(AnswerError::IllegalMove(name));
        }
        self.state.prompt = None;
        self.action_step(investigator, ActionStep::Start);
        match order {
            MoveOrder::Go(to) => {
                let free_pass = self.state.investigators[investigator].free_border;
                self.travel(investigator, to, free_pass);
            }
            _ => self.log(format!("{name} reste sur place.")),
        }
        Ok(())
    }

    /// Applies one order of the movement phase; last turn's restrictions end with it.
    fn apply_move(&mut self, index: usize, order: MoveOrder) {
        let mover = &mut self.state.investigators[index];
        let (car_broken, free_pass) = (mover.car_broken, mover.free_border);
        mover.car_broken = false;
        mover.free_border = false;
        mover.forbidden_city = None;
        mover.blocked_crossing = None;
        let (id, at) = (mover.id, mover.location.at_phrase());
        match order {
            MoveOrder::Stay if car_broken => {
                self.state.investigators[index].skips_action = true;
                self.log(format!(
                    "La voiture {} refuse de démarrer : {} perd son tour.",
                    id.of_name(),
                    id.pronoun()
                ));
            }
            MoveOrder::Stay => self.log(format!("{} reste {at}.", id.name())),
            MoveOrder::Psychoanalyst => {
                let gain = i32::from(self.dice.d6());
                let mover = &mut self.state.investigators[index];
                mover.san += gain;
                mover.skips_action = true;
                self.log(format!(
                    "{} consulte un psychanalyste {at} et regagne {gain} SAN.",
                    id.name()
                ));
            }
            MoveOrder::Go(to) => self.travel(index, to, free_pass),
        }
    }

    /// Moves an investigator, rolling for a border crossing.
    fn travel(&mut self, index: usize, to: LocationId, free_pass: bool) {
        let mover = &self.state.investigators[index];
        let (id, from) = (mover.id, mover.location);
        if travel::crosses_border(from, to) && !free_pass {
            match self.dice.d6() {
                5 => {
                    self.state.investigators[index].free_border = true;
                    self.log(format!(
                        "{} est retenu{} à la frontière, mais pourra la passer sans encombre au prochain tour.",
                        id.name(),
                        id.agreement()
                    ));
                    return;
                }
                6 => {
                    self.state.investigators[index].blocked_crossing = Some(to);
                    self.log(format!(
                        "{} est refoulé{} à la frontière et ne pourra pas la repasser vers {to} au prochain tour.",
                        id.name(),
                        id.agreement()
                    ));
                    return;
                }
                _ => {}
            }
        }
        self.state.investigators[index].go_to(to);
        self.log(format!("{} se rend {}.", id.name(), to.at_phrase()));
    }
}

fn first_action() -> Phase {
    Phase::Action {
        investigator: 0,
        step: ActionStep::Start,
    }
}
