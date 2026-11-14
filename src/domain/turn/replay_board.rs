use serde::{Deserialize, Serialize};

use crate::domain::counter::CounterPlace;
use crate::domain::counter_kind::CounterKind;
use crate::domain::investigator_id::InvestigatorId;
use crate::domain::investigator_status::InvestigatorStatus;
use crate::domain::location_id::LocationId;
use crate::domain::turn::game_state::GameState;
use crate::domain::turn::phase::Phase;

/// The whole board at one moment of a replay, secrets included.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBoard {
    /// Where the professor really stands; none before he picks his start.
    pub professor: Option<LocationId>,
    /// His hit points.
    pub professor_hp: i32,
    /// The investigators, in playing order.
    pub investigators: Vec<ReplayInvestigator>,
    /// Every placed counter.
    pub counters: Vec<ReplayCounter>,
    /// The professor's hand.
    pub hand: Vec<CounterKind>,
    /// The city written down for Nyog'Sothep's appearance; none before the professor picks it.
    pub nyog_city: Option<LocationId>,
    /// Where Nyog'Sothep stands, once summoned.
    pub nyog: Option<LocationId>,
    /// Cities destroyed by Nyog'Sothep.
    pub destroyed: Vec<LocationId>,
}

/// An investigator on a replayed board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayInvestigator {
    /// Which counter.
    pub id: InvestigatorId,
    /// Where they are.
    pub location: LocationId,
    /// Sanity points.
    pub san: i32,
    /// In the game, mad or dead.
    pub status: InvestigatorStatus,
    /// Holds a weapon.
    pub weapon: bool,
    /// Holds a protective sign.
    pub sign: bool,
    /// Holds a medallion.
    pub medallion: bool,
    /// Knows the banishment spell.
    pub spell: bool,
}

/// A counter placed on a replayed board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCounter {
    /// Its city.
    pub city: LocationId,
    /// Its kind.
    pub kind: CounterKind,
    /// Not found by the investigators yet.
    pub hidden: bool,
}

impl ReplayBoard {
    /// The board of a game as it stands.
    pub fn of(state: &GameState) -> ReplayBoard {
        let set_up = state.phase != Phase::Setup;
        ReplayBoard {
            professor: set_up.then_some(state.professor.location),
            professor_hp: state.professor.hp,
            investigators: state
                .investigators
                .iter()
                .map(|investigator| ReplayInvestigator {
                    id: investigator.id,
                    location: investigator.location,
                    san: investigator.san,
                    status: investigator.status,
                    weapon: investigator.weapon,
                    sign: investigator.sign,
                    medallion: investigator.medallion,
                    spell: investigator.spell,
                })
                .collect(),
            counters: state
                .counters
                .placed()
                .map(|(city, counter)| ReplayCounter {
                    city,
                    kind: counter.kind,
                    hidden: !matches!(
                        counter.place,
                        CounterPlace::Placed {
                            discovered: true,
                            ..
                        }
                    ),
                })
                .collect(),
            hand: state.counters.hand(),
            nyog_city: set_up.then_some(state.nyog.city),
            nyog: state.nyog.location,
            destroyed: state.destroyed.clone(),
        }
    }
}
