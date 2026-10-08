use serde::{Deserialize, Serialize};

/// Whether a location is a city or a sea zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationKind {
    /// A city, reached by road (or by sea when it is a port).
    City,
    /// A sea zone, crossed by boat between ports.
    Sea,
}
