//! The rule sets a game may be played with, behind the [`rules::Rules`] hooks of the turn engine.

pub(crate) mod fan_made;
pub(crate) mod original;
mod rule_set;
pub(crate) mod rules;

pub use rule_set::RuleSet;
