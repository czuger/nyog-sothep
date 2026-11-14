use crate::domain::counter::CounterPlace;
use crate::domain::investigator_status::InvestigatorStatus;
use crate::domain::turn::engine::Engine;

impl Engine<'_> {
    /// Changes an investigator's SAN; at 0 or less they go mad.
    pub(super) fn change_san(&mut self, investigator: usize, change: i32) {
        let target = &mut self.state.investigators[investigator];
        if !target.is_active() || change == 0 {
            return;
        }
        target.san += change;
        if target.san <= 0 {
            self.go_mad(investigator);
        }
    }

    /// An investigator regains SAN, up to the most the rules allow.
    ///
    /// # Returns
    /// The SAN actually regained.
    pub(super) fn gain_san(&mut self, investigator: usize, gain: i32) -> i32 {
        let cap = self
            .rules()
            .san_cap(&self.state.investigators[investigator]);
        let target = &mut self.state.investigators[investigator];
        if !target.is_active() {
            return 0;
        }
        let before = target.san;
        target.san = (before + gain).min(cap.max(before));
        target.san - before
    }

    /// The investigator becomes a Fanatics counter, face up, where they stand.
    fn go_mad(&mut self, investigator: usize) {
        let target = &mut self.state.investigators[investigator];
        target.san = 0;
        target.status = InvestigatorStatus::Mad;
        let (name, location) = (target.id.name(), target.location);
        self.log(format!(
            "{name} perd la raison et rejoint les Fanatiques du professeur {}.",
            location.at_phrase()
        ));
        if location.is_city() {
            self.state
                .counters
                .set_place_at(location, CounterPlace::Reserve);
            self.state.counters.raise_fanatics(location);
        }
        self.check_end();
    }

    /// Nyog'Sothep kills an investigator who has no protective sign.
    pub(super) fn kill(&mut self, investigator: usize) {
        let id = self.state.investigators[investigator].id;
        self.log(format!(
            "{} se trouve face à Nyog'Sothep sans signe protecteur : {} meurt sur-le-champ.",
            id.name(),
            id.pronoun()
        ));
        self.put_to_death(investigator);
    }

    /// The investigator dies, after the log has told how.
    pub(super) fn put_to_death(&mut self, investigator: usize) {
        self.state.investigators[investigator].status = InvestigatorStatus::Dead;
        self.check_end();
    }
}
