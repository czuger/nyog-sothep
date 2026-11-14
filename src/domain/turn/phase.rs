use serde::{Deserialize, Serialize};

/// Where the turn engine stands: the cursor that lets a round pause on a prompt and resume.
///
/// While a professor's prompt waits, the phase is where the engine resumes once it is answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// The professor chooses his start and Nyog'Sothep's city.
    Setup,
    /// The professor starts his turn: Teleportation (or Nyog'Sothep's move).
    ProfessorMove,
    /// The professor moves, then may attack an investigator met on arrival.
    ProfessorMovement,
    /// A fight started by the professor on arrival waits for the investigator's choice.
    ProfessorFight {
        /// The investigator attacked.
        investigator: usize,
    },
    /// The professor may summon Nyog'Sothep.
    ProfessorSummon,
    /// The professor may place a counter.
    ProfessorPlace,
    /// The professor draws on Table 1 and discards down to five; after the starting draws
    /// (turn 0) the first round opens, otherwise the investigators move.
    ProfessorDraw {
        /// Draws still to make.
        remaining: u8,
    },
    /// The investigators move, all together.
    Moves,
    /// The action phase of one investigator.
    Action {
        /// The investigator, by index.
        investigator: usize,
        /// The step reached.
        step: ActionStep,
    },
    /// The banishment partly worked: the investigators place Nyog'Sothep.
    Banishment,
    /// The game is over.
    Finished,
}

/// Steps of an investigator's action phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStep {
    /// Encounter, then the table choice.
    Start,
    /// Waiting for the table choice.
    Event,
    /// Waiting for the cities named to the professor.
    Question {
        /// The event also lets the investigator play again.
        plays_again: bool,
    },
    /// The event is resolved: play again, or go on to the fight check.
    AfterEvent {
        /// The event lets the investigator play again.
        plays_again: bool,
    },
    /// Waiting for the « play again » move.
    Replay,
    /// The unavoidable fight if the professor is here.
    Fight,
    /// Waiting for the fight choice.
    FightChoice,
    /// The action phase is over.
    Done,
}
