//! How safe a city is for a group of Fanatics.

use crate::domain::location_id::LocationId;
use crate::domain::professor_ai::professor_weights::ProfessorWeights;
use crate::domain::professor_ai::threats::Reach;
use crate::domain::travel;
use crate::domain::turn::GameState;

/// Steps beyond which an investigator no longer threatens the city.
const FAR_ENOUGH: u8 = 4;

/// How safe Fanatics left in `city` would be: the unarmed walk into counters and reveal them,
/// then the armed come back to kill them, so every investigator nearby counts, the armed more,
/// and so do the roads leading in, busy crossroads seeing more visitors.
pub fn fanatics_safety(
    state: &GameState,
    weights: &ProfessorWeights,
    reach: &Reach,
    city: LocationId,
) -> f64 {
    let nearest = |armed_only: bool| {
        let steps = reach
            .iter()
            .filter(|(index, _)| !armed_only || state.investigators[*index].weapon)
            .filter_map(|(_, steps)| steps[city.index()])
            .min()
            .unwrap_or(FAR_ENOUGH);
        f64::from(steps.min(FAR_ENOUGH))
    };
    let roads = travel::road_neighbours(city).len() as f64;
    nearest(false) + weights.safety_armed_weight * nearest(true)
        - weights.safety_road_weight * roads
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::professor_ai::test_state;
    use crate::domain::professor_ai::threats::investigator_reach;
    use LocationId::*;

    #[test]
    fn unarmed_investigators_nearby_make_a_city_unsafe() {
        let mut state = test_state();
        for investigator in &mut state.investigators {
            investigator.location = Boston;
        }
        let reach = investigator_reach(&state);
        let weights = ProfessorWeights::default();
        let safety = |city| fanatics_safety(&state, &weights, &reach, city);
        assert!(safety(Boston) < weights.safe_enough);
        assert!(safety(Pascoag) > safety(Lynn));
    }
}
