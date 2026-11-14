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
        let target = &mut self.state.investigators[investigator];
        target.status = InvestigatorStatus::Dead;
        let (name, pronoun) = (target.id.name(), target.id.pronoun());
        self.log(format!(
            "{name} se trouve face à Nyog'Sothep sans signe protecteur : {pronoun} meurt sur-le-champ."
        ));
        self.check_end();
    }
}
