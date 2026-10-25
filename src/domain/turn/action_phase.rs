use crate::domain::counter::{Counter, CounterPlace};
use crate::domain::encounter::{self, CounterFate};
use crate::domain::event_effect::EventEffect;
use crate::domain::event_table::EventTable;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::fights::FightContext;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::prompt::Prompt;

impl Engine<'_> {
    /// Runs the action phase of one investigator from the given step.
    pub(super) fn action(&mut self, investigator: usize, step: ActionStep) {
        match step {
            ActionStep::Start => self.action_start(investigator),
            ActionStep::AfterEvent { plays_again } => self.after_event(investigator, plays_again),
            ActionStep::Fight => self.action_fight(investigator),
            ActionStep::Done => {
                self.action_step(investigator + 1, ActionStep::Start);
                self.update_banishment_target();
            }
            // Waiting steps without a prompt cannot happen; close the phase rather than loop.
            ActionStep::Event
            | ActionStep::Question { .. }
            | ActionStep::Replay
            | ActionStep::FightChoice => self.action_step(investigator, ActionStep::Done),
        }
    }

    /// Encounter, then the event table choice.
    fn action_start(&mut self, investigator: usize) {
        if investigator >= self.state.investigators.len() {
            self.update_banishment_target();
            self.state.phase = Phase::ProfessorMove;
            return;
        }
        let actor = &self.state.investigators[investigator];
        if !actor.is_active() || actor.location.is_sea() {
            self.action_step(investigator, ActionStep::Done);
            return;
        }
        if actor.skips_action {
            self.action_step(investigator, ActionStep::Fight);
            return;
        }
        if let Some(counter) = self.state.counters.placed_at(actor.location)
            && !self.encounter(investigator, counter)
        {
            self.action_step(investigator, ActionStep::Fight);
            return;
        }
        if self.state.investigators[investigator].is_active() {
            self.ask(
                Prompt::Table { investigator },
                Phase::Action {
                    investigator,
                    step: ActionStep::Event,
                },
            );
        }
    }

    /// Resolves an encounter; returns whether an event may still be drawn.
    fn encounter(&mut self, investigator: usize, counter: Counter) -> bool {
        let city = self.state.investigators[investigator].location;
        let outcome = encounter::resolve(
            counter.kind,
            &self.state.investigators[investigator],
            self.dice,
        );
        self.log(outcome.text.clone());
        let place = match outcome.fate {
            CounterFate::Stays => CounterPlace::Placed {
                city,
                discovered: true,
            },
            CounterFate::Reserve => CounterPlace::Reserve,
            CounterFate::Removed => CounterPlace::Removed,
        };
        self.state.counters.set_place_at(city, place);
        let back_destroyed = self
            .state
            .is_destroyed(self.state.investigators[investigator].previous);
        let actor = &mut self.state.investigators[investigator];
        actor.sign &= !outcome.loses_sign;
        actor.medallion = (actor.medallion && !outcome.loses_medallion) || outcome.kills_fanatics;
        actor.fanatics_killed += u32::from(outcome.kills_fanatics);
        actor.skip_turns += outcome.lost_turns;
        if outcome.pushed_back && !back_destroyed {
            actor.go_back();
        }
        if outcome.forbids_city {
            actor.forbidden_city = Some(city);
        }
        self.change_san(investigator, -outcome.san_loss);
        outcome.event_allowed
    }

    /// Rolls on the chosen table and applies the event; the professor's answer, if any, comes
    /// before the « play again » move.
    pub(super) fn draw_event(&mut self, investigator: usize, table: EventTable) {
        let actor = &self.state.investigators[investigator];
        let dice_count = actor.event_dice();
        let name = actor.id.name();
        let roll = self.dice.roll(dice_count);
        self.log(format!(
            "{name} tire un événement sur la table {} ({dice_count}d6 : {roll}) : « {} »",
            table.number(),
            table.text(roll)
        ));
        let effect = table.effect(roll);
        self.apply_event(investigator, &effect);
        if self.state.is_finished() || !self.state.investigators[investigator].is_active() {
            self.action_step(investigator, ActionStep::Done);
            return;
        }
        self.action_step(
            investigator,
            ActionStep::AfterEvent {
                plays_again: effect.plays_again,
            },
        );
        if let Some(question) = effect.question {
            if question.cities_named_by_investigator().is_some() {
                let step = ActionStep::Question {
                    plays_again: effect.plays_again,
                };
                self.ask(
                    Prompt::Cities {
                        investigator,
                        question,
                    },
                    Phase::Action { investigator, step },
                );
                return;
            }
            self.professor_answers(question);
        }
    }

    fn apply_event(&mut self, investigator: usize, effect: &EventEffect) {
        let back_destroyed = self
            .state
            .is_destroyed(self.state.investigators[investigator].previous);
        let actor = &mut self.state.investigators[investigator];
        actor.weapon = (actor.weapon || effect.gains_weapon) && !effect.loses_weapon;
        actor.sign |= effect.gains_sign;
        actor.medallion |= effect.gains_medallion;
        actor.spell |= effect.learns_spell;
        actor.car_broken |= effect.breaks_car;
        actor.skip_turns += u8::from(effect.skips_next_turn);
        if effect.goes_back && !back_destroyed {
            actor.go_back();
            let (name, at) = (actor.id.name(), actor.location.at_phrase());
            self.log(format!("{name} retourne {at}."));
        }
        if effect.san > 0 {
            self.state.investigators[investigator].san += effect.san;
        } else {
            self.change_san(investigator, effect.san);
        }
    }

    /// After the event: play again, or go on to the fight check.
    pub(super) fn after_event(&mut self, investigator: usize, plays_again: bool) {
        if plays_again && self.state.investigators[investigator].is_active() {
            self.ask(
                Prompt::PlayAgain { investigator },
                Phase::Action {
                    investigator,
                    step: ActionStep::Replay,
                },
            );
        } else {
            self.action_step(investigator, ActionStep::Fight);
        }
    }

    /// The unavoidable fight when the investigator ends up in the professor's city.
    fn action_fight(&mut self, investigator: usize) {
        self.action_step(investigator, ActionStep::Done);
        let actor = &self.state.investigators[investigator];
        let here = actor.location;
        if !actor.is_active()
            || !here.is_city()
            || self.state.nyog.is_summoned()
            || self.state.professor.location != here
        {
            return;
        }
        self.log(format!(
            "{} tombe sur le professeur {} : le combat est inévitable !",
            actor.id.name(),
            here.at_phrase()
        ));
        self.start_fight(investigator, FightContext::ActionPhase);
    }
}
