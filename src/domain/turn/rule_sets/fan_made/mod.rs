//! The fan-made version of `fan_mode_proposition.md`: three d20 event tables, new items, no
//! respite after the summoning, draws decided by the place, Fanatics arriving on their own, a
//! wounded professor given away, a progressive banishment spell cast alone, a SAN cap and a
//! hurrying Foul One.

mod draw_sites;
mod encounters;
mod end_of_game;
mod equipment;
mod event_phase;
mod event_tables;
mod fan_made_rules;
mod fanatics_supply;
mod location_draws;
mod progressive_banishment;

pub(crate) use fan_made_rules::FanMadeRules;

#[cfg(test)]
mod fan_made_tests;
