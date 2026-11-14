use crate::domain::dice::Dice;
use crate::domain::location_id::LocationId;
use crate::domain::nyog_sothep::{self, BANISHERS_NEEDED, BanishmentResult};
use crate::domain::turn::engine::Engine;
use crate::domain::turn::outcome::Outcome;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::prompt::Prompt;
use crate::domain::turn::rule_sets::rules::spell_holders_at;

impl Engine<'_> {
    /// Three spell holders or more in the city cast the spell together: one die minus their
    /// number.
    pub(super) fn group_banishment(&mut self, city: LocationId) {
        let casters = spell_holders_at(self.state, city);
        if casters.len() < BANISHERS_NEEDED {
            return;
        }
        let roll = self.dice.d6();
        let result = nyog_sothep::banishment(roll, casters.len());
        self.log(format!(
            "{} investigateurs lancent le sort de renvoi : {roll} au dé, moins {} participants.",
            casters.len(),
            casters.len()
        ));
        match result {
            BanishmentResult::Banished => {
                for caster in casters {
                    self.state.investigators[caster].banished = true;
                }
                self.finish(Outcome::Banished);
            }
            BanishmentResult::Displaced => {
                self.log("Le sort agit en partie : les investigateurs choisissent la ville où envoyer Nyog'Sothep.");
                self.ask(Prompt::BanishmentCity, Phase::Banishment);
            }
            BanishmentResult::Fails => self.log("Le sort échoue : Nyog'Sothep reste libre."),
            BanishmentResult::Disaster | BanishmentResult::Backlash => {
                let loss = result.san_loss();
                self.log(format!(
                    "Le sort se retourne contre eux : chaque participant perd {loss} SAN, et Nyog'Sothep reste libre."
                ));
                for caster in casters {
                    self.change_san(caster, -loss);
                }
            }
        }
    }

    /// Three spell holders gathered in a city become Nyog'Sothep's only goal.
    pub(super) fn update_group_target(&mut self) {
        if !self.state.nyog.is_summoned() {
            return;
        }
        let target = self
            .state
            .active_investigators()
            .into_iter()
            .map(|index| self.state.investigators[index].location)
            .find(|location| {
                location.is_city()
                    && spell_holders_at(self.state, *location).len() >= BANISHERS_NEEDED
            });
        if target.is_some()
            && target != self.state.nyog.target
            && let Some(city) = target
        {
            self.log(format!(
                "Trois investigateurs connaissant le sort de renvoi sont réunis à {city} : Nyog'Sothep et le professeur fondent sur eux."
            ));
        }
        self.state.nyog.target = target;
    }
}
