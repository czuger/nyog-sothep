use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::professor_ai::bluff::{choose_fanatics_names, choose_names};
use crate::domain::professor_ai::counter_values::{choose_discard, choose_table1};
use crate::domain::professor_ai::fight_choices::{
    choose_attack, choose_carry, choose_relocation, choose_teleport,
};
use crate::domain::professor_ai::movement_choice::choose_movement;
use crate::domain::professor_ai::nyog_choice::choose_nyog_step;
use crate::domain::professor_ai::placement_choice::choose_placement;
use crate::domain::professor_ai::setup_choice::choose_setup;
use crate::domain::turn::{
    GameState, ProfessorBrain, ProfessorMove, ProfessorMoveOptions, Teleport,
};

/// Professor Petersen played by the computer.
///
/// It keeps no memory between calls: every decision is computed from the state, so a game can
/// be saved and reloaded at any time. Its random generator only varies the choice among
/// near-best options.
pub struct PetersenAi {
    rng: StdRng,
}

impl PetersenAi {
    /// An AI seeded from the operating system.
    pub fn new() -> PetersenAi {
        PetersenAi {
            rng: rand::make_rng(),
        }
    }

    /// A reproducible AI, for simulations.
    pub fn seeded(seed: u64) -> PetersenAi {
        PetersenAi {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Default for PetersenAi {
    fn default() -> PetersenAi {
        PetersenAi::new()
    }
}

impl ProfessorBrain for PetersenAi {
    fn setup(&mut self, _: &GameState) -> (LocationId, LocationId) {
        choose_setup(&mut self.rng)
    }

    fn teleport(&mut self, state: &GameState, options: &[Teleport]) -> Option<Teleport> {
        choose_teleport(state, options)
    }

    fn movement(&mut self, state: &GameState, options: &ProfessorMoveOptions) -> ProfessorMove {
        choose_movement(state, options, &mut self.rng)
    }

    fn attack(&mut self, state: &GameState, candidates: &[usize], forced: bool) -> Option<usize> {
        choose_attack(state, candidates, forced)
    }

    fn relocate(&mut self, state: &GameState, options: &[LocationId]) -> LocationId {
        choose_relocation(state, options)
    }

    fn carry(&mut self, state: &GameState, _: usize, options: &[LocationId]) -> LocationId {
        choose_carry(state, options)
    }

    fn placement(&mut self, state: &GameState, options: &[CounterKind]) -> Option<CounterKind> {
        choose_placement(state, options)
    }

    fn table1_choice(&mut self, state: &GameState, options: &[CounterKind]) -> CounterKind {
        choose_table1(state, options)
    }

    fn discard(&mut self, state: &GameState, hand: &[CounterKind]) -> CounterKind {
        choose_discard(state, hand)
    }

    fn summon(&mut self, _: &GameState) -> bool {
        true
    }

    fn nyog_step(&mut self, state: &GameState, options: &[LocationId]) -> LocationId {
        choose_nyog_step(state, options)
    }

    fn name_cities(&mut self, state: &GameState, count: usize) -> Vec<LocationId> {
        choose_names(state, count, &mut self.rng)
    }

    fn name_fanatics_cities(&mut self, state: &GameState) -> Vec<LocationId> {
        choose_fanatics_names(state, &mut self.rng)
    }
}
