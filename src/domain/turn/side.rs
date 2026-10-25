use serde::{Deserialize, Serialize};

/// The two sides of the game: the human plays one, the computer the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    /// The four investigators.
    Investigators,
    /// Professor Petersen (and Nyog'Sothep).
    Professor,
}

impl Side {
    /// The stable code stored in the database and used in forms.
    pub fn code(self) -> &'static str {
        match self {
            Side::Investigators => "investigators",
            Side::Professor => "professor",
        }
    }

    /// Reads a code written by [`Side::code`].
    pub fn from_code(code: &str) -> Option<Side> {
        [Side::Investigators, Side::Professor]
            .into_iter()
            .find(|side| side.code() == code)
    }
}
