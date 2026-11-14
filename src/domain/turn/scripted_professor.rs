//! A predictable professor for the engine tests.

use std::collections::VecDeque;

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::professor_brain::{
    ProfessorBrain, ProfessorMove, ProfessorMoveOptions, Teleport,
};

/// Plays the scripted moves, then stays; never attacks unless told; places what it is told.
pub(super) struct ScriptedProfessor {
    pub start: LocationId,
    pub nyog_city: LocationId,
    pub moves: VecDeque<ProfessorMove>,
    pub attack: bool,
    pub placements: VecDeque<CounterKind>,
    pub summons: bool,
    pub names: Vec<LocationId>,
    pub teleport: bool,
}

impl ScriptedProfessor {
    pub fn new(start: LocationId, nyog_city: LocationId) -> ScriptedProfessor {
        ScriptedProfessor {
            start,
            nyog_city,
            moves: VecDeque::new(),
            attack: false,
            placements: VecDeque::new(),
            summons: false,
            names: Vec::new(),
            teleport: false,
        }
    }

    pub fn goes(mut self, destination: LocationId) -> ScriptedProfessor {
        self.moves.push_back(ProfessorMove {
            destination,
            flying_horror: false,
        });
        self
    }
}

impl ProfessorBrain for ScriptedProfessor {
    fn setup(&mut self, _: &GameState) -> (LocationId, LocationId) {
        (self.start, self.nyog_city)
    }

    fn teleport(&mut self, _: &GameState, options: &[Teleport]) -> Option<Teleport> {
        self.teleport.then(|| options[0])
    }

    fn movement(&mut self, state: &GameState, _: &ProfessorMoveOptions) -> ProfessorMove {
        self.moves.pop_front().unwrap_or(ProfessorMove {
            destination: state.professor.location,
            flying_horror: false,
        })
    }

    fn attack(&mut self, _: &GameState, candidates: &[usize], _: bool) -> Option<usize> {
        self.attack.then(|| candidates[0])
    }

    fn relocate(&mut self, _: &GameState, options: &[LocationId]) -> LocationId {
        options[0]
    }

    fn carry(&mut self, _: &GameState, _: usize, options: &[LocationId]) -> LocationId {
        options[0]
    }

    fn placement(&mut self, _: &GameState, options: &[CounterKind]) -> Option<CounterKind> {
        let wanted = self.placements.pop_front()?;
        options.contains(&wanted).then_some(wanted)
    }

    fn table1_choice(&mut self, _: &GameState, options: &[CounterKind]) -> CounterKind {
        options[0]
    }

    fn discard(&mut self, _: &GameState, hand: &[CounterKind]) -> CounterKind {
        hand[0]
    }

    fn summon(&mut self, _: &GameState) -> bool {
        self.summons
    }

    fn nyog_step(&mut self, _: &GameState, options: &[LocationId]) -> LocationId {
        options[0]
    }

    fn name_cities(&mut self, _: &GameState, _: usize) -> Vec<LocationId> {
        self.names.clone()
    }

    fn name_fanatics_cities(&mut self, _: &GameState) -> Vec<LocationId> {
        self.names.clone()
    }
}
