//! Pure game rules, with no I/O.

mod counter;
mod counter_kind;
mod counter_pool;
pub mod dice;
pub mod encounter;
mod event_effect;
mod event_table;
mod fight_choice;
mod investigator;
mod investigator_id;
mod investigator_status;
pub mod location_id;
mod location_kind;
pub mod map;
mod nyog_sothep;
pub mod professor_ai;
pub mod professor_fight;
mod professor_question;
pub mod scoring;
pub mod table1;
pub mod travel;
pub mod turn;

pub use counter::{Counter, CounterPlace};
pub use counter_kind::CounterKind;
pub use counter_pool::CounterPool;
pub use dice::{Dice, RandomDice, ScriptedDice};
pub use event_effect::EventEffect;
pub use event_table::{EVENT_ROWS, EventTable};
pub use fight_choice::FightChoice;
pub use investigator::{Investigator, STARTING_CITY, STARTING_SAN_BONUS, STARTING_SAN_DICE};
pub use investigator_id::InvestigatorId;
pub use investigator_status::InvestigatorStatus;
pub use location_id::{LOCATION_COUNT, LocationId};
pub use location_kind::LocationKind;
pub use nyog_sothep::{
    BANISHERS_NEEDED, BanishmentResult, NyogSothep, SUMMONING_FANATICS_CITIES, SUMMONING_RANGE,
    banishment, meeting_san_loss,
};
pub use professor_question::ProfessorQuestion;
pub use scoring::Score;
pub use travel::Network;
