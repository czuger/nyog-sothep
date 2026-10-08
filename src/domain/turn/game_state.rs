use serde::{Deserialize, Serialize};

use crate::domain::counter_pool::CounterPool;
use crate::domain::investigator::Investigator;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::NyogSothep;
use crate::domain::turn::clue::Clue;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::professor::{Professor, RouteStage};
use crate::domain::turn::prompt::Prompt;

/// The whole state of a game: everything the turn engine needs to resume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// The round being played (the professor's turn opens it).
    pub turn: u32,
    /// The engine's cursor.
    pub phase: Phase,
    /// The decision the human must take, if any.
    pub prompt: Option<Prompt>,
    /// The result, once the game is over.
    pub outcome: Option<Outcome>,
    /// Professor Petersen.
    pub professor: Professor,
    /// The four investigators, in playing order.
    pub investigators: Vec<Investigator>,
    /// Every counter.
    pub counters: CounterPool,
    /// Nyog'Sothep.
    pub nyog: NyogSothep,
    /// Cities destroyed by Nyog'Sothep, in order.
    pub destroyed: Vec<LocationId>,
    /// The professor's route sheet.
    pub route_sheet: Vec<RouteStage>,
    /// The investigators' notebook.
    pub clues: Vec<Clue>,
    /// The professor must announce his movements at sea.
    pub sea_announcements: bool,
    /// The professor's next move is made in the open.
    pub move_in_the_open: bool,
}

impl GameState {
    /// Whether a city was destroyed by Nyog'Sothep.
    pub fn is_destroyed(&self, location: LocationId) -> bool {
        self.destroyed.contains(&location)
    }

    /// Indexes of the investigators still in the game.
    pub fn active_investigators(&self) -> Vec<usize> {
        (0..self.investigators.len())
            .filter(|index| self.investigators[*index].is_active())
            .collect()
    }

    /// Indexes of the active investigators standing in a location.
    pub fn investigators_at(&self, location: LocationId) -> Vec<usize> {
        self.active_investigators()
            .into_iter()
            .filter(|index| self.investigators[*index].location == location)
            .collect()
    }

    /// Whether the game is over.
    pub fn is_finished(&self) -> bool {
        self.outcome.is_some()
    }
}
