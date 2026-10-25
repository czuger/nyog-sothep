//! Distances on the whole network, as the investigators travel.

use crate::domain::location_id::LocationId;
use crate::domain::travel::{self, Network};

/// The steps counted between two locations that cannot reach each other.
pub(super) const UNREACHABLE: u32 = 30;

/// Steps from `from` to `to`, by road and sea.
pub(super) fn steps(from: LocationId, to: LocationId) -> u32 {
    travel::distance(from, to, Network::All, |_| true).map_or(UNREACHABLE, u32::from)
}

/// Steps from `from` to every location (`UNREACHABLE` when it cannot be reached).
pub(super) fn steps_from(from: LocationId) -> Vec<u32> {
    travel::distances(from, Network::All, |_| true)
        .iter()
        .map(|found| found.map_or(UNREACHABLE, u32::from))
        .collect()
}

/// Steps from `from` to the nearest of `targets` (`UNREACHABLE` when there is none).
pub(super) fn steps_to_nearest(from: LocationId, targets: &[LocationId]) -> u32 {
    let found = travel::distances(from, Network::All, |_| true);
    targets
        .iter()
        .filter_map(|target| found[target.index()])
        .map(u32::from)
        .min()
        .unwrap_or(UNREACHABLE)
}
