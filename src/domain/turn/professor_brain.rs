use crate::domain::counter_kind::CounterKind;
use crate::domain::location_id::LocationId;
use crate::domain::turn::game_state::GameState;

/// The professor's move for the turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessorMove {
    /// Where he goes (his own location to stay).
    pub destination: LocationId,
    /// He uses the Flying Horror to get there.
    pub flying_horror: bool,
}

/// The legal moves offered to the professor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessorMoveOptions {
    /// Staying, or one ordinary step by road or by sea.
    pub steps: Vec<LocationId>,
    /// Destinations of the Flying Horror (empty when he does not hold it).
    pub flying: Vec<LocationId>,
}

/// A Teleportation the professor may cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Teleport {
    /// The investigator moved, by index.
    pub investigator: usize,
    /// Where they are sent.
    pub destination: LocationId,
}

/// The decisions of Professor Petersen, taken by the computer.
///
/// Every method receives the whole state, which the professor may read except for future dice.
/// The engine checks each answer and falls back to a legal one when it is not.
pub trait ProfessorBrain {
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
