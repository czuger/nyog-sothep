//! The professor's secret choices before the game.

use rand::Rng;

use crate::domain::investigator::STARTING_CITY;
use crate::domain::location_id::LocationId;
use crate::domain::map;
use crate::domain::professor_ai::near_best::pick_near_best;
use crate::domain::travel::{self, Network};

/// Picks Nyog'Sothep's city (central, with many roads, away from Nantucket), then a start city
/// a few steps from it and far from the investigators.
pub fn choose_setup<R: Rng>(rng: &mut R) -> (LocationId, LocationId) {
    let from_nantucket = travel::distances(STARTING_CITY, Network::All, |_| true);
    let scored: Vec<(LocationId, f64)> = map::cities()
        .map(|city| {
            let roads = travel::road_neighbours(city).len() as f64;
            let away = f64::from(from_nantucket[city.index()].unwrap_or(0)).min(8.0);
            (city, roads * 2.0 + away)
        })
        .collect();
    let nyog_city = pick_near_best(scored, 3.0, rng).unwrap_or(LocationId::Concord);
    let from_nyog = travel::distances(nyog_city, Network::All, |_| true);
    let scored: Vec<(LocationId, f64)> = map::cities()
        .filter(|city| travel::road_neighbours(*city).len() >= 2)
        .map(|city| {
            let away = f64::from(from_nantucket[city.index()].unwrap_or(0)).min(10.0);
            let from_goal = f64::from(from_nyog[city.index()].unwrap_or(9));
            let spread = -(from_goal - 3.0).abs();
            (city, away + 2.0 * spread)
        })
        .collect();
    let start = pick_near_best(scored, 2.0, rng).unwrap_or(LocationId::Worcester);
    (start, nyog_city)
}
