use crate::domain::fight_choice::FightChoice;
use crate::domain::location_id::LocationId;
use crate::domain::professor_fight::{self, PROFESSOR_HP, ProfessorAfterFight};
use crate::domain::travel::{self, Network};
use crate::domain::turn::answer_error::AnswerError;
use crate::domain::turn::clue::ClueKind;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::{ActionStep, Phase};
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
        let outcome = professor_fight::resolve(choice, self.dice);
        let id = self.state.investigators[investigator].id;
        let name = id.name();
        match choice {
            FightChoice::Weapon if outcome.damage == 0 => {
                self.log(format!("{name} tire sur le professeur et le manque."))
            }
            FightChoice::Weapon => self.log(format!(
                "{name} tire sur le professeur et lui fait perdre {} points de vie.",
                outcome.damage
            )),
            FightChoice::Sign => self.log(format!(
                "{name} brandit son signe protecteur : {} perd 2 SAN et son signe, mais repousse le professeur.",
                id.pronoun()
            )),
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
        if outcome.loses_sign {
            self.state.investigators[investigator].sign = false;
        }
        if outcome.damage > 0 && self.wound_professor(investigator, outcome.damage) {
            return;
        }
        if outcome.carries_investigator {
            self.carry(investigator);
        }
        if outcome.professor != ProfessorAfterFight::Stays {
            self.relocate_professor(outcome.professor);
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
        false
    }

    /// The professor moves an unarmed investigator two cities.
    fn carry(&mut self, investigator: usize) {
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
        let picked = self.brain.carry(self.state, investigator, &options);
        let destination = if options.contains(&picked) {
            picked
        } else {
            options[0]
        };
        let target = &mut self.state.investigators[investigator];
        target.go_to(destination);
        let name = target.id.inner_name();
        self.log(format!(
            "Le professeur emmène {name} {}.",
            destination.at_phrase()
        ));
    }

    fn relocate_professor(&mut self, after: ProfessorAfterFight) {
        let from = self.state.professor.location;
        let options = travel::neighbours(from);
        if options.is_empty() {
            return;
        }
        let picked = self.brain.relocate(self.state, &options);
        self.state.professor.location = if options.contains(&picked) {
            picked
        } else {
            options[0]
        };
        self.clue(ClueKind::Retreated { from });
        self.log(match after {
            ProfessorAfterFight::PushedBack => "Le professeur est repoussé d'un cran.",
            _ => "Le professeur bat en retraite d'un cran.",
        });
    }
}
