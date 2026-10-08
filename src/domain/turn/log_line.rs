/// A line of the game log, in French, never revealing the professor's secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    /// The turn it happened.
    pub turn: u32,
    /// The text.
    pub text: String,
}
