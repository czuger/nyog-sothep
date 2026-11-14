//! The tunable weights of the investigators' AI.

/// Every weight the investigators' AI scores its choices with; `examples/tune_ai.rs` searches
/// better values by self-play.
#[derive(Debug, Clone, PartialEq)]
pub struct InvestigatorWeights {
    /// SAN at which an investigator rests at the psychoanalyst, keeping enough to face
    /// Nyog'Sothep (4 SAN at the first meeting, and up to 4 more if the spell fails).
    pub low_san: f64,
    /// Once Nyog'Sothep is out, SAN at which a banisher rests: time counts.
    pub banisher_low_san: f64,
    /// SAN a banisher keeps to cast again: a meeting, then the worst backlash of the spell.
    pub casting_san: f64,
    /// SAN with which a banisher travels freely under Nyog'Sothep's blows.
    pub fit_san: f64,
    /// What each step costs a tired banisher, compared with a fit one.
    pub tired_step_cost: f64,
    /// Possible locations at or under which the professor is hunted city by city.
    pub narrow_belief: f64,
    /// Moves scoring this close to the best are picked at random.
    pub move_tolerance: f64,
    /// Worth of ending a move on land, where an event is drawn.
    pub land_worth: f64,
    /// Worth of each step away from the other investigators, up to three.
    pub spread_worth: f64,
    /// Share of that worth for the hunters, who also share out the belief.
    pub hunter_spread_factor: f64,
    /// Weight of the distance to the professor for the unarmed.
    pub unarmed_hunting_factor: f64,
    /// Cost of walking into a known counter for the unarmed.
    pub known_counter_cost: f64,
    /// Cost of walking where the professor may be, for the unarmed without a medallion.
    pub near_professor_cost: f64,
    /// Worth of the chance of ending a move in the professor's city.
    pub hunt_now_worth: f64,
    /// Worth of the chance of threatening him after his next move.
    pub hunt_next_worth: f64,
    /// Worth of killing a group of Fanatics when it is the last one needed to win.
    pub fanatics_kill_worth: f64,
    /// Cost of ending in a city with a single way out once Nyog'Sothep roams.
    pub dead_end_cost: f64,
    /// Cost of ending in a city with no way out.
    pub shut_in_cost: f64,
    /// Worth of one bit learnt about the professor's position, in SAN points.
    pub bit_worth: f64,
    /// How much more an armed investigator, who can act on it, values that knowledge.
    pub hunter_info_factor: f64,
    /// Worth of a weapon, in SAN points.
    pub weapon_worth: f64,
    /// Worth of a protective sign or the spell before the summoning.
    pub sign_or_spell_worth: f64,
    /// Worth of a protective sign or the spell once Nyog'Sothep is out.
    pub sign_or_spell_worth_summoned: f64,
    /// Worth of a medallion.
    pub medallion_worth: f64,
}

impl Default for InvestigatorWeights {
    fn default() -> InvestigatorWeights {
        InvestigatorWeights {
            low_san: 8.0,
            banisher_low_san: 5.0,
            casting_san: 7.0,
            fit_san: 15.0,
            tired_step_cost: 3.0,
            narrow_belief: 8.0,
            move_tolerance: 0.5,
            land_worth: 0.5,
            spread_worth: 0.5,
            hunter_spread_factor: 0.6,
            unarmed_hunting_factor: 0.7,
            known_counter_cost: 4.0,
            near_professor_cost: 3.0,
            hunt_now_worth: 20.0,
            hunt_next_worth: 10.0,
            fanatics_kill_worth: 24.0,
            dead_end_cost: 2.0,
            shut_in_cost: 6.0,
            bit_worth: 1.0,
            hunter_info_factor: 1.5,
            weapon_worth: 6.0,
            sign_or_spell_worth: 5.0,
            sign_or_spell_worth_summoned: 10.0,
            medallion_worth: 3.0,
        }
    }
}

impl InvestigatorWeights {
    /// Every weight with its name, for the tuner.
    pub fn fields_mut(&mut self) -> Vec<(&'static str, &mut f64)> {
        vec![
            ("low_san", &mut self.low_san),
            ("banisher_low_san", &mut self.banisher_low_san),
            ("casting_san", &mut self.casting_san),
            ("fit_san", &mut self.fit_san),
            ("tired_step_cost", &mut self.tired_step_cost),
            ("narrow_belief", &mut self.narrow_belief),
            ("move_tolerance", &mut self.move_tolerance),
            ("land_worth", &mut self.land_worth),
            ("spread_worth", &mut self.spread_worth),
            ("hunter_spread_factor", &mut self.hunter_spread_factor),
            ("unarmed_hunting_factor", &mut self.unarmed_hunting_factor),
            ("known_counter_cost", &mut self.known_counter_cost),
            ("near_professor_cost", &mut self.near_professor_cost),
            ("hunt_now_worth", &mut self.hunt_now_worth),
            ("hunt_next_worth", &mut self.hunt_next_worth),
            ("fanatics_kill_worth", &mut self.fanatics_kill_worth),
            ("dead_end_cost", &mut self.dead_end_cost),
            ("shut_in_cost", &mut self.shut_in_cost),
            ("bit_worth", &mut self.bit_worth),
            ("hunter_info_factor", &mut self.hunter_info_factor),
            ("weapon_worth", &mut self.weapon_worth),
            ("sign_or_spell_worth", &mut self.sign_or_spell_worth),
            (
                "sign_or_spell_worth_summoned",
                &mut self.sign_or_spell_worth_summoned,
            ),
            ("medallion_worth", &mut self.medallion_worth),
        ]
    }
}
