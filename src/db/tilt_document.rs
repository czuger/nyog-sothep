use serde::{Deserialize, Serialize};

use crate::domain::LocationId;

/// A sub-document of a game: the rotation a token is drawn with, and where it stood when it was
/// drawn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TiltDocument {
    /// The token's key.
    pub token: String,
    /// Where the token stood when its tilt was drawn.
    pub location: LocationId,
    /// Rotation, in tenths of a degree.
    pub tenths: i32,
}
