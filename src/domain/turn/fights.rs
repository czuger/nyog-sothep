use crate::domain::fight_choice::FightChoice;
use crate::domain::item::Protection;
use crate::domain::location_id::LocationId;
use crate::domain::professor_fight::{self, PROFESSOR_HP, ProfessorAfterFight};
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::{ActionStep, Phase};
use crate::domain::turn::professor_answers::pick;
use crate::domain::turn::professor_prompt::ProfessorPrompt;
use crate::domain::turn::prompt::Prompt;

/// Who started a fight, which tells where the engine resumes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FightContext {
    /// The professor attacked on arrival.
    ProfessorArrival,
    /// The investigator ended the action phase in the professor's city.
    ActionPhase,
}

impl Engine<'_> {
    /// Starts a fight: the professor is revealed, and the investigator chooses how to face him
    /// when they have more than one way.
    pub(super) fn start_fight(&mut self, investigator: usize, context: FightContext) {
        let at = self.state.professor.location;
        self.clue(ClueKind::Spotted { at });
        let options = FightChoice::options(&self.state.investigators[investigator]);
        if let [only] = options.as_slice() {
            self.resolve_fight(investigator, *only);
            return;
        }
        let phase = match context {
            FightContext::ProfessorArrival => Phase::ProfessorFight { investigator },
            FightContext::ActionPhase => Phase::Action {
                investigator,
                step: ActionStep::FightChoice,
            },
        };
        self.ask(
            Prompt::Fight {
                investigator,
                options,
            },
            phase,
        );
    }

    pub(super) fn answer_fight(
        &mut self,
        investigator: usize,
        options: &[FightChoice],
        choice: FightChoice,
    ) -> Result<(), AnswerError> {
        if !options.contains(&choice) {
            return Err(AnswerError::IllegalFightChoice);
        }
        self.state.prompt = None;
        self.state.phase = match self.state.phase {
            Phase::ProfessorFight { .. } => Phase::ProfessorSummon,
            _ => Phase::Action {
                investigator,
                step: ActionStep::Done,
            },
        };
        self.resolve_fight(investigator, choice);
        Ok(())
    }

    fn resolve_fight(&mut self, investigator: usize, choice: FightChoice) {
        let fighter = &self.state.investigators[investigator];
        let outcome = professor_fight::resolve(choice, fighter.weapon_kind.bonus(), &mut self.dice);
        let (id, protection) = (fighter.id, fighter.protection);
        let name = id.name();
        match choice {
            FightChoice::Weapon if outcome.damage == 0 => {
                self.log(format!("{name} tire sur le professeur et le manque."))
            }
            FightChoice::Weapon => self.log(format!(
                "{name} tire sur le professeur et lui fait perdre {} points de vie.",
                outcome.damage
            )),
            FightChoice::Dynamite => self.log(format!(
                "{name} lance sa dynamite sur le professeur et lui fait perdre {} points de vie.",
                outcome.damage
            )),
            FightChoice::Sign => self.log(match protection {
                Protection::Sign => format!(
                    "{name} brandit son signe protecteur : {} perd 2 SAN et son signe, mais repousse le professeur.",
                    id.pronoun()
                ),
                Protection::MnarStone => format!(
                    "{name} brandit sa pierre étoilée de Mnar : {} perd 2 SAN, mais repousse le professeur.",
                    id.pronoun()
                ),
            }),
            FightChoice::Hide => self.log(format!(
                "Grâce à son médaillon, {} passe pour un Fanatique : il ne se passe rien.",
                id.inner_name()
            )),
            FightChoice::Unarmed => self.log(format!(
                "Sans rien pour se défendre, {} perd 4 SAN face au professeur.",
                id.inner_name()
            )),
        }
        self.change_san(investigator, -outcome.san_loss);
        let fighter = &mut self.state.investigators[investigator];
        if outcome.loses_sign {
            fighter.lose_protection();
        }
        if let Some(gear) = outcome.spends {
            fighter.spend(gear);
        }
        if outcome.damage > 0 && self.wound_professor(investigator, outcome.damage) {
            return;
        }
        // An unarmed investigator is carried away and the professor stays; otherwise he leaves.
        if outcome.carries_investigator {
            self.offer_carry(investigator);
        } else if outcome.professor != ProfessorAfterFight::Stays {
            self.offer_relocation(outcome.professor == ProfessorAfterFight::PushedBack);
        }
    }

    /// Returns `true` when the professor dies.
    fn wound_professor(&mut self, investigator: usize, damage: i32) -> bool {
        self.state.investigators[investigator].damage_dealt += damage as u32;
        let professor = &mut self.state.professor;
        professor.hp = (professor.hp - damage).max(0);
        let hp = professor.hp;
        self.log(format!(
            "Il reste {hp} points de vie au professeur (sur {PROFESSOR_HP})."
        ));
        if hp == 0 {
            self.finish(Outcome::ProfessorKilled);
            return true;
        }
        self.rules().professor_wounded(self);
        false
    }

    /// Asks where the professor carries an unarmed investigator, two cities away; the engine
    /// then resumes where the fight started.
    fn offer_carry(&mut self, investigator: usize) {
        if !self.state.investigators[investigator].is_active() {
            return;
        }
        let from = self.state.investigators[investigator].location;
        let passable = |city: LocationId| !self.state.is_destroyed(city);
        let mut options = travel::within(from, 2, 2, Network::Roads, passable);
        if options.is_empty() {
            options = travel::within(from, 1, 1, Network::Roads, passable);
        }
        if options.is_empty() {
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::Carry {
                investigator,
                options,
            }),
            self.state.phase,
        );
    }

    pub(super) fn answer_carry(
        &mut self,
        investigator: usize,
        options: &[LocationId],
        picked: LocationId,
    ) -> Result<(), AnswerError> {
        let destination = pick(options, picked)?;
        self.state.prompt = None;
        let target = &mut self.state.investigators[investigator];
        target.go_to(destination);
        let name = target.id.inner_name();
        self.log(format!(
            "Le professeur emmène {name} {}.",
            destination.at_phrase()
        ));
        Ok(())
    }

    /// Asks where the professor retreats, or is pushed back, after a fight.
    fn offer_relocation(&mut self, pushed_back: bool) {
        let options = travel::neighbours(self.state.professor.location).to_vec();
        if options.is_empty() {
            return;
        }
        self.ask(
            Prompt::Professor(ProfessorPrompt::Relocate {
                options,
                pushed_back,
            }),
            self.state.phase,
        );
    }

    pub(super) fn answer_relocation(
        &mut self,
        options: &[LocationId],
        pushed_back: bool,
        picked: LocationId,
    ) -> Result<(), AnswerError> {
        let destination = pick(options, picked)?;
        self.state.prompt = None;
        let from = self.state.professor.location;
        self.state.professor.location = destination;
        self.clue(ClueKind::Retreated { from });
        let public = match pushed_back {
            true => "Le professeur est repoussé d'un cran.",
            false => "Le professeur bat en retraite d'un cran.",
        };
        let revealed = format!("{} Il se rend {}.", public, destination.at_phrase());
        self.log_revealing(public, revealed);
        Ok(())
    }
}
