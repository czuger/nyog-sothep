use serde::{Deserialize, Serialize};

use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::professor_prompt::{ProfessorAnswer, ProfessorPrompt};

/// The professor's move for the turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessorMove {
    /// Where he goes (his own location to stay).
    pub destination: LocationId,
    /// He uses the Flying Horror to get there.
    pub flying_horror: bool,
}

/// The legal moves offered to the professor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfessorMoveOptions {
    /// Staying, or one ordinary step by road or by sea.
    pub steps: Vec<LocationId>,
    /// Destinations of the Flying Horror (empty when he does not hold it).
    pub flying: Vec<LocationId>,
}

/// A Teleportation the professor may cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Teleport {
    /// The investigator moved, by index.
    pub investigator: usize,
    /// Where they are sent.
    pub destination: LocationId,
}

/// The decisions of Professor Petersen when the computer plays him.
///
/// Every method answers one [`ProfessorPrompt`] and receives the whole state, which the professor
/// may read except for future dice. The engine checks each answer like a human's and falls back
/// to a legal one when it is refused.
pub trait ProfessorBrain {
    /// Answers a prompt by asking the matching method.
    fn answer(&mut self, state: &GameState, prompt: &ProfessorPrompt) -> ProfessorAnswer {
        use ProfessorAnswer as A;
        use ProfessorPrompt as P;
        match prompt {
            P::Setup => {
                let (start, nyog_city) = self.setup(state);
                A::Setup { start, nyog_city }
            }
            P::Teleport { options } => A::Teleport(self.teleport(state, options)),
            P::Movement { options } => A::Location(self.movement(state, options).destination),
            P::Attack { candidates, forced } => A::Attack(self.attack(state, candidates, *forced)),
            P::Carry {
                investigator,
                options,
            } => A::Location(self.carry(state, *investigator, options)),
            P::Relocate { options, .. } => A::Location(self.relocate(state, options)),
            P::Summon => A::Summon(self.summon(state)),
            P::Placement { options } => A::Placement(self.placement(state, options)),
            P::Table1 { options } => A::Counter(self.table1_choice(state, options)),
            P::Discard { hand } => A::Counter(self.discard(state, hand)),
            P::NyogStep { options } => A::Location(self.nyog_step(state, options)),
            P::NameCities { count } => A::Cities(self.name_cities(state, *count)),
            P::NameFanaticsCities => A::Cities(self.name_fanatics_cities(state)),
        }
    }

    /// Chooses the start city and the city where Nyog'Sothep will appear.
    fn setup(&mut self, state: &GameState) -> (LocationId, LocationId);

    /// Picks a Teleportation to cast before moving, if any.
    fn teleport(&mut self, state: &GameState, options: &[Teleport]) -> Option<Teleport>;

    /// Chooses the move of the turn.
    fn movement(&mut self, state: &GameState, options: &ProfessorMoveOptions) -> ProfessorMove;

    /// Picks the investigator to attack on arrival; `forced` when the die made him reveal himself.
    fn attack(&mut self, state: &GameState, candidates: &[usize], forced: bool) -> Option<usize>;

    /// Picks where he retreats or is pushed back after a fight.
    fn relocate(&mut self, state: &GameState, options: &[LocationId]) -> LocationId;

    /// Picks where he carries an investigator who faced him unarmed.
    fn carry(
        &mut self,
        state: &GameState,
        investigator: usize,
        options: &[LocationId],
    ) -> LocationId;

    /// Picks the counter to place where he stands, if any.
    fn placement(&mut self, state: &GameState, options: &[CounterKind]) -> Option<CounterKind>;

    /// Picks one of the two counters a Table 1 roll offers.
    fn table1_choice(&mut self, state: &GameState, options: &[CounterKind]) -> CounterKind;

    /// Picks the counter to discard when he holds six.
    fn discard(&mut self, state: &GameState, hand: &[CounterKind]) -> CounterKind;

    /// Whether to summon Nyog'Sothep now (only asked when he can).
    fn summon(&mut self, state: &GameState) -> bool;

    /// Picks Nyog'Sothep's next city.
    fn nyog_step(&mut self, state: &GameState, options: &[LocationId]) -> LocationId;

    /// Names `count` cities, one of which must be his position.
    fn name_cities(&mut self, state: &GameState, count: usize) -> Vec<LocationId>;

    /// Names two cities, one of which must hold Fanatics.
    fn name_fanatics_cities(&mut self, state: &GameState) -> Vec<LocationId>;
}
