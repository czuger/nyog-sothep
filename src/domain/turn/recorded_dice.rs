use crate::domain::dice::Dice;

/// Dice that remember each die they roll, for the replay.
pub(super) struct RecordedDice<'a> {
    inner: &'a mut dyn Dice,
    rolls: Vec<u8>,
}

impl<'a> RecordedDice<'a> {
    pub(super) fn new(inner: &'a mut dyn Dice) -> RecordedDice<'a> {
        RecordedDice {
            inner,
            rolls: Vec::new(),
        }
    }

    /// The dice rolled since the last [`RecordedDice::take_rolls`].
    pub(super) fn rolls(&self) -> &[u8] {
        &self.rolls
    }

    /// The dice rolled since the last call, forgotten.
    pub(super) fn take_rolls(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.rolls)
    }
}

impl Dice for RecordedDice<'_> {
    fn d6(&mut self) -> u8 {
        let roll = self.inner.d6();
        self.rolls.push(roll);
        roll
    }

    fn d20(&mut self) -> u8 {
        let roll = self.inner.d20();
        self.rolls.push(roll);
        roll
    }
}
