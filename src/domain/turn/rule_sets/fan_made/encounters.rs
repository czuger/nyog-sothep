//! Fan-made encounters: the dynamite and the dreamer's amulet act before the shared encounter.

use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::encounter::{self, CounterFate, EncounterOutcome};
use crate::domain::investigator::Investigator;
use crate::domain::item::{Gear, Item};

/// Resolves the meeting of an investigator with a placed counter, single-use objects first.
pub(crate) fn resolve(
    kind: CounterKind,
    investigator: &Investigator,
    dice: &mut dyn Dice,
) -> EncounterOutcome {
    let holds = |gear: Gear| investigator.holds(Item::Gear(gear));
    match kind {
        CounterKind::Fanatics | CounterKind::Ghouls if holds(Gear::Dynamite) => {
            dynamite(kind, investigator)
        }
        CounterKind::Dreams if holds(Gear::Amulet) => amulet(investigator),
        _ => encounter::resolve(kind, investigator, dice),
    }
}

/// The dynamite destroys Fanatics or Ghouls, without a roll nor a SAN loss.
fn dynamite(kind: CounterKind, investigator: &Investigator) -> EncounterOutcome {
    let killed_fanatics = kind == CounterKind::Fanatics;
    let fate = match killed_fanatics {
        true => CounterFate::Removed,
        false => CounterFate::Reserve,
    };
    let mut outcome = EncounterOutcome::new(
        fate,
        format!(
            "{} fait sauter {} à la dynamite.",
            investigator.id.name(),
            kind.with_article()
        ),
    );
    outcome.kills_fanatics = killed_fanatics;
    outcome.spends = Some(Gear::Dynamite);
    outcome.event_allowed = true;
    outcome
}

/// The amulet wards off the Dreams sent by the professor.
fn amulet(investigator: &Investigator) -> EncounterOutcome {
    let mut outcome = EncounterOutcome::new(
        CounterFate::Reserve,
        format!(
            "L'amulette de rêveur {} chasse les Rêves envoyés par le professeur.",
            investigator.id.of_name()
        ),
    );
    outcome.spends = Some(Gear::Amulet);
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dice::ScriptedDice;
    use crate::domain::investigator_id::InvestigatorId;

    fn holding(gear: Gear) -> Investigator {
        let mut investigator = Investigator::new(InvestigatorId::Carter, 12);
        investigator.take(Item::Gear(gear));
        investigator
    }

    #[test]
    fn dynamite_kills_without_a_roll() {
        let mut dice = ScriptedDice::new(&[], 1);
        let fanatics = resolve(CounterKind::Fanatics, &holding(Gear::Dynamite), &mut dice);
        assert!(fanatics.kills_fanatics && fanatics.san_loss == 0);
        assert_eq!(fanatics.fate, CounterFate::Removed);
        assert_eq!(fanatics.spends, Some(Gear::Dynamite));
        let ghouls = resolve(CounterKind::Ghouls, &holding(Gear::Dynamite), &mut dice);
        assert_eq!((ghouls.san_loss, ghouls.fate), (0, CounterFate::Reserve));
        assert_eq!(dice.remaining(), 0);
    }

    #[test]
    fn the_amulet_wards_off_dreams_once() {
        let mut dice = ScriptedDice::new(&[], 1);
        let dreams = resolve(CounterKind::Dreams, &holding(Gear::Amulet), &mut dice);
        assert_eq!((dreams.san_loss, dreams.spends), (0, Some(Gear::Amulet)));
        let plain = resolve(CounterKind::Dreams, &holding(Gear::Pass), &mut dice);
        assert_eq!(plain.san_loss, 2);
    }
}
