//! The tunable weights of the professor's AI.

/// Every weight the professor's AI scores its choices with; `examples/tune_ai.rs` searches
/// better values by self-play.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfessorWeights {
    /// Risk of ending a turn where an armed investigator stands.
    pub danger_here: f64,
    /// Risk of ending a turn one move away from an armed investigator.
    pub danger_next: f64,
    /// Risk of ending a turn two moves away from an armed investigator.
    pub danger_far: f64,
    /// Cost of walking into a city holding investigators, which he must announce.
    pub meeting_cost: f64,
    /// Cost of putting to sea, where he loses time.
    pub sea_cost: f64,
    /// Cost of putting to sea once his sea moves are announced.
    pub announced_sea_cost: f64,
    /// Cost of staying in place.
    pub stay_cost: f64,
    /// Cost of staying in place while the investigators nearly know where he is.
    pub exposed_stay_cost: f64,
    /// Possible locations at or under which he counts as exposed.
    pub exposed_belief_size: f64,
    /// Bonus for spending the Flying Horror when exposed or threatened.
    pub flight_bonus: f64,
    /// Cost of spending the Flying Horror otherwise.
    pub idle_flight_cost: f64,
    /// Steps from Nyog'Sothep's city he drifts within freely before the summoning.
    pub drift_range: f64,
    /// Cost of each step beyond that range.
    pub drift_cost: f64,
    /// Cost of each step beyond summoning range once enough Fanatics are in place.
    pub summoning_approach_cost: f64,
    /// Worth of a new city where Fanatics can be left, before its safety.
    pub fanatics_place_worth: f64,
    /// Worth of a new city near the investigators where a harmful counter can be left.
    pub trap_place_worth: f64,
    /// Fanatics cities he aims for, a little above the five needed in case some die.
    pub fanatics_target: f64,
    /// Safety from which Fanatics are left in a city.
    pub safe_enough: f64,
    /// Weight of the nearest armed investigator in a city's safety.
    pub safety_armed_weight: f64,
    /// What each road into a city takes from its safety.
    pub safety_road_weight: f64,
    /// Moves scoring this close to the best are picked at random.
    pub move_tolerance: f64,
    /// What Nyog'Sothep meeting an investigator it kills or drives mad is worth.
    pub nyog_kill_worth: f64,
    /// What Nyog'Sothep meeting an investigator who only loses SAN is worth.
    pub nyog_harm_worth: f64,
    /// How much more a spell holder is worth to Nyog'Sothep.
    pub nyog_spell_factor: f64,
}

impl Default for ProfessorWeights {
    fn default() -> ProfessorWeights {
        ProfessorWeights {
            danger_here: 30.0,
            danger_next: 20.0,
            danger_far: 4.0,
            meeting_cost: 6.0,
            sea_cost: 3.0,
            announced_sea_cost: 6.0,
            stay_cost: 1.0,
            exposed_stay_cost: 4.0,
            exposed_belief_size: 3.0,
            flight_bonus: 4.0,
            idle_flight_cost: 8.0,
            drift_range: 3.0,
            drift_cost: 0.7,
            summoning_approach_cost: 4.0,
            fanatics_place_worth: 3.0,
            trap_place_worth: 2.0,
            fanatics_target: 7.0,
            safe_enough: 1.0,
            safety_armed_weight: 0.5,
            safety_road_weight: 0.5,
            move_tolerance: 1.5,
            nyog_kill_worth: 10.0,
            nyog_harm_worth: 4.0,
            nyog_spell_factor: 1.5,
        }
    }
}

impl ProfessorWeights {
    /// Every weight with its name, for the tuner.
    pub fn fields_mut(&mut self) -> Vec<(&'static str, &mut f64)> {
        vec![
            ("danger_here", &mut self.danger_here),
            ("danger_next", &mut self.danger_next),
            ("danger_far", &mut self.danger_far),
            ("meeting_cost", &mut self.meeting_cost),
            ("sea_cost", &mut self.sea_cost),
            ("announced_sea_cost", &mut self.announced_sea_cost),
            ("stay_cost", &mut self.stay_cost),
            ("exposed_stay_cost", &mut self.exposed_stay_cost),
            ("exposed_belief_size", &mut self.exposed_belief_size),
            ("flight_bonus", &mut self.flight_bonus),
            ("idle_flight_cost", &mut self.idle_flight_cost),
            ("drift_range", &mut self.drift_range),
            ("drift_cost", &mut self.drift_cost),
            ("summoning_approach_cost", &mut self.summoning_approach_cost),
            ("fanatics_place_worth", &mut self.fanatics_place_worth),
            ("trap_place_worth", &mut self.trap_place_worth),
            ("fanatics_target", &mut self.fanatics_target),
            ("safe_enough", &mut self.safe_enough),
            ("safety_armed_weight", &mut self.safety_armed_weight),
            ("safety_road_weight", &mut self.safety_road_weight),
            ("move_tolerance", &mut self.move_tolerance),
            ("nyog_kill_worth", &mut self.nyog_kill_worth),
            ("nyog_harm_worth", &mut self.nyog_harm_worth),
            ("nyog_spell_factor", &mut self.nyog_spell_factor),
        ]
    }
}
