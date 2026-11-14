use serde::{Deserialize, Serialize};

use crate::domain::turn::rule_sets::fan_made::FanMadeRules;
use crate::domain::turn::rule_sets::original::OriginalRules;
use crate::domain::turn::rule_sets::rules::Rules;

/// The rules a game is played with, chosen when it is created.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleSet {
    /// The rules published in *Jeux & Stratégie*.
    #[default]
    Original,
    /// The fan-made version of `fan_mode_proposition.md`.
    FanMade,
}

impl RuleSet {
    /// Every rule set.
    pub const ALL: [RuleSet; 2] = [RuleSet::Original, RuleSet::FanMade];

    /// The implementation of these rules: the only place where the rule sets are told apart.
    pub(crate) fn rules(self) -> &'static dyn Rules {
        match self {
            RuleSet::Original => &OriginalRules,
            RuleSet::FanMade => &FanMadeRules,
        }
    }

    /// The stable code used in forms.
    pub fn code(self) -> &'static str {
        match self {
            RuleSet::Original => "original",
            RuleSet::FanMade => "fan_made",
        }
    }

    /// Reads a code written by [`RuleSet::code`].
    pub fn from_code(code: &str) -> Option<RuleSet> {
        RuleSet::ALL.into_iter().find(|rules| rules.code() == code)
    }

    /// French name.
    pub fn label(self) -> &'static str {
        match self {
            RuleSet::Original => "Règles originales",
            RuleSet::FanMade => "Version fan-made",
        }
    }

    /// Investigators knowing the spell needed to cast it together.
    pub fn banishers_needed(self) -> usize {
        self.rules().banishers_needed()
    }
}
