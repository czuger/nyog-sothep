//! Movement on the board: neighbours by road and by sea, border crossings and distances.

use std::collections::VecDeque;
use std::sync::OnceLock;

use crate::domain::location_id::{LOCATION_COUNT, LocationId};
use crate::domain::map::{BORDER_CROSSINGS, ROADS, SEA_LINKS};

/// The links a distance is measured along.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Roads and sea links: one step is one turn of movement.
    All,
    /// Roads only, between cities (Nyog'Sothep, the professor's push of an investigator).
    Roads,
}

/// Neighbour lists of every location, built once from the map tables.
struct Adjacency {
    roads: Vec<Vec<LocationId>>,
    all: Vec<Vec<LocationId>>,
}

fn adjacency() -> &'static Adjacency {
    static ADJACENCY: OnceLock<Adjacency> = OnceLock::new();
    ADJACENCY.get_or_init(|| Adjacency {
        roads: LocationId::ALL
            .into_iter()
            .map(|from| linked(&ROADS, from))
            .collect(),
        all: LocationId::ALL
            .into_iter()
            .map(|from| {
                let mut found = linked(&ROADS, from);
                found.extend(linked(&SEA_LINKS, from));
                found
            })
            .collect(),
    })
}

/// Cities linked to `from` by a road.
pub fn road_neighbours(from: LocationId) -> &'static [LocationId] {
    &adjacency().roads[from.index()]
}

/// Locations one move away from `from`: road neighbours, plus the sea zones of a port, or
/// the neighbouring zones and the ports of a sea zone.
pub fn neighbours(from: LocationId) -> &'static [LocationId] {
    &adjacency().all[from.index()]
}

/// Neighbours along the given network.
pub fn network_neighbours(from: LocationId, network: Network) -> &'static [LocationId] {
    match network {
        Network::All => neighbours(from),
        Network::Roads => road_neighbours(from),
    }
}

/// Whether going from `from` to `to` crosses a state border.
pub fn crosses_border(from: LocationId, to: LocationId) -> bool {
    BORDER_CROSSINGS.contains(&(from, to)) || BORDER_CROSSINGS.contains(&(to, from))
}

/// Number of steps from `from` to every location, or `None` when it cannot be reached.
///
/// # Arguments
/// * `passable` - whether a location may be entered; `from` itself is always allowed.
pub fn distances(
    from: LocationId,
    network: Network,
    passable: impl Fn(LocationId) -> bool,
) -> [Option<u8>; LOCATION_COUNT] {
    let mut found = [None; LOCATION_COUNT];
    found[from.index()] = Some(0);
    let mut queue = VecDeque::from([from]);
    while let Some(current) = queue.pop_front() {
        let next_distance = found[current.index()].unwrap_or(0) + 1;
        for &next in network_neighbours(current, network) {
            if found[next.index()].is_none() && passable(next) {
                found[next.index()] = Some(next_distance);
                queue.push_back(next);
            }
        }
    }
    found
}

/// Number of steps from `from` to `to`, entering only passable locations.
pub fn distance(
    from: LocationId,
    to: LocationId,
    network: Network,
    passable: impl Fn(LocationId) -> bool,
) -> Option<u8> {
    distances(from, network, passable)[to.index()]
}

/// Locations at `min..=max` steps from `from`, in board order.
pub fn within(
    from: LocationId,
    min: u8,
    max: u8,
    network: Network,
    passable: impl Fn(LocationId) -> bool,
) -> Vec<LocationId> {
    let found = distances(from, network, passable);
    LocationId::ALL
        .into_iter()
        .filter(|id| found[id.index()].is_some_and(|steps| (min..=max).contains(&steps)))
        .collect()
}

/// The first steps of every shortest path from `from` to `to`.
///
/// # Returns
/// An empty list when `to` cannot be reached or equals `from`.
pub fn first_steps_towards(
    from: LocationId,
    to: LocationId,
    network: Network,
    passable: impl Fn(LocationId) -> bool + Copy,
) -> Vec<LocationId> {
    let to_target = distances(to, network, passable);
    let Some(steps) = to_target[from.index()].filter(|steps| *steps > 0) else {
        return Vec::new();
    };
    network_neighbours(from, network)
        .iter()
        .copied()
        .filter(|next| passable(*next) && to_target[next.index()] == Some(steps - 1))
        .collect()
}

fn linked(links: &[(LocationId, LocationId)], from: LocationId) -> Vec<LocationId> {
    links
        .iter()
        .filter_map(|(a, b)| match from {
            _ if *a == from => Some(*b),
            _ if *b == from => Some(*a),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::map::cities;
    use LocationId::*;

    #[test]
    fn adjacency_is_symmetric() {
        for a in LocationId::ALL {
            for &b in neighbours(a) {
                assert!(neighbours(b).contains(&a), "{a:?}-{b:?}");
            }
        }
    }

    #[test]
    fn sea_moves_go_port_zone_port() {
        assert_eq!(neighbours(Nantucket), [NantucketSound]);
        let from_sound = neighbours(NantucketSound);
        for expected in [
            Chatham,
            Edgartown,
            Nantucket,
            GulfOfMaine,
            BuzzardsBay,
            AtlanticOcean,
        ] {
            assert!(from_sound.contains(&expected), "{expected:?}");
        }
    }

    #[test]
    fn borders_are_listed_both_ways() {
        assert!(crosses_border(Nashua, Lowell) && crosses_border(Lowell, Nashua));
        assert!(!crosses_border(Boston, Lynn));
    }

    #[test]
    fn every_location_is_reachable_from_nantucket() {
        let found = distances(Nantucket, Network::All, |_| true);
        assert!(found.iter().all(Option::is_some));
        let by_road = distances(Boston, Network::Roads, |_| true);
        for city in cities() {
            let island = [Nantucket, Edgartown, Gloucester].contains(&city);
            assert_eq!(by_road[city.index()].is_some(), !island, "{city:?}");
        }
    }

    #[test]
    fn distances_skip_impassable_locations() {
        assert_eq!(
            distance(Nashua, Lawrence, Network::Roads, |_| true),
            Some(2)
        );
        assert_eq!(
            distance(Nashua, Lawrence, Network::Roads, |id| id != Lowell),
            Some(6)
        );
        let steps = first_steps_towards(Nashua, Lawrence, Network::Roads, |_| true);
        assert_eq!(steps, vec![Lowell]);
        assert!(first_steps_towards(Boston, Boston, Network::Roads, |_| true).is_empty());
        assert_eq!(within(Nantucket, 1, 2, Network::All, |_| true).len(), 6);
    }
}
