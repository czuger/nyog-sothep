use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::turn::engine::Engine;
use crate::domain::turn::phase::Phase;
use crate::domain::turn::rule_sets::fan_made::draw_sites::DrawSite;

impl Engine<'_> {
    /// One roll on the table of the professor's place, then the next; once none remains, the
    /// Fanatics may join him and the round goes on.
    pub(super) fn location_draw(&mut self, remaining: u8) {
        let Some(left) = remaining.checked_sub(1) else {
            self.fanatics_arrive();
            self.finish_draws();
            return;
        };
        self.state.phase = Phase::ProfessorDraw { remaining: left };
        let at = self.state.professor.location;
        let site = DrawSite::of(at);
        let roll = self.dice.d6();
        let Some(kind) = site.counter(roll, at) else {
            self.reveal(format!(
                "Le professeur lance 1d6 sur la table {} : {roll}, rien.",
                site.name()
            ));
            return;
        };
        self.state.counters.supply(kind);
        self.reveal(format!(
            "Le professeur lance 1d6 sur la table {} : {roll}, {}.",
            site.name(),
            kind.name()
        ));
        let limited = self.state.counters.hand_without(CounterKind::Fanatics);
        self.discard_over_limit(limited);
    }
}
