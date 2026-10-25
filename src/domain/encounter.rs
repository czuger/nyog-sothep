//! Encounters: what a counter placed in a city does to the investigator who meets it.

use crate::domain::counter_kind::CounterKind;
use crate::domain::dice::Dice;
use crate::domain::investigator::Investigator;

/// What becomes of the counter after the encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterFate {
    /// It stays in the city, now face up on the board.
    Stays,
    /// It goes back to the reserve, so Table 1 may give it again.
    Reserve,
    /// It leaves the game for good.
    Removed,
}

/// The result of an encounter, applied by the turn engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncounterOutcome {
    /// SAN lost (positive number).
    pub san_loss: i32,
    /// The protective sign is lost.
    pub loses_sign: bool,
    /// The medallion is lost.
    pub loses_medallion: bool,
    /// The Fanatics were killed: a medallion is gained and the kill is scored.
    pub kills_fanatics: bool,
    /// The investigator goes back to the location they left.
    pub pushed_back: bool,
    /// The city may not be entered on the next turn.
    pub forbids_city: bool,
    /// Coming turns lost.
    pub lost_turns: u8,
    /// What becomes of the counter.
    pub fate: CounterFate,
    /// The investigator may still draw an event.
    pub event_allowed: bool,
    /// French narration, starting with the investigator's name.
    pub text: String,
}

impl EncounterOutcome {
    fn new(fate: CounterFate, text: String) -> EncounterOutcome {
        EncounterOutcome {
            san_loss: 0,
            loses_sign: false,
            loses_medallion: false,
            kills_fanatics: false,
            pushed_back: false,
            forbids_city: false,
            lost_turns: 0,
            fate,
            event_allowed: false,
            text,
        }
    }
}

/// Resolves the meeting of an investigator with a placed counter.
pub fn resolve(
    kind: CounterKind,
    investigator: &Investigator,
    dice: &mut dyn Dice,
) -> EncounterOutcome {
    match kind {
        CounterKind::Ghouls => ghouls(investigator, dice),
        CounterKind::DeepOnes => deep_ones(investigator),
        CounterKind::Fanatics => fanatics(investigator, dice),
        CounterKind::ThingsOfTheMist => things_of_the_mist(investigator),
        CounterKind::Inhabitants => inhabitants(investigator),
        CounterKind::Dreams => dreams(investigator),
        CounterKind::Storm | CounterKind::FlyingHorror | CounterKind::Teleportation => {
            storm(investigator)
        }
    }
}

fn ghouls(investigator: &Investigator, dice: &mut dyn Dice) -> EncounterOutcome {
    let name = investigator.id.name();
    let sign_relief = i32::from(investigator.sign);
    if investigator.weapon {
        let loss = if dice.d6() <= 4 { 3 } else { 2 } - sign_relief;
        let mut outcome = EncounterOutcome::new(
            CounterFate::Reserve,
            format!(
                "{name} rencontre des Goules et les élimine avec son arme, mais perd {loss} SAN."
            ),
        );
        outcome.san_loss = loss;
        outcome.event_allowed = true;
        return outcome;
    }
    let loss = 4 - sign_relief;
    let pronoun = investigator.id.pronoun();
    let mut outcome = EncounterOutcome::new(
        CounterFate::Stays,
        format!("{name} rencontre des Goules : sans arme, {pronoun} s'enfuit et perd {loss} SAN."),
    );
    outcome.san_loss = loss;
    outcome
}

fn deep_ones(investigator: &Investigator) -> EncounterOutcome {
    let name = investigator.id.name();
    if investigator.sign {
        let mut outcome = EncounterOutcome::new(
            CounterFate::Reserve,
            format!(
                "{name} rencontre des Profonds et les repousse avec son signe protecteur, qu'{} perd avec 1 SAN.",
                investigator.id.pronoun()
            ),
        );
        outcome.san_loss = 1;
        outcome.loses_sign = true;
        outcome.event_allowed = true;
        return outcome;
    }
    let mut outcome = EncounterOutcome::new(
        CounterFate::Stays,
        format!("{name} rencontre des Profonds et perd 3 SAN."),
    );
    outcome.san_loss = 3;
    outcome
}

fn fanatics(investigator: &Investigator, dice: &mut dyn Dice) -> EncounterOutcome {
    let name = investigator.id.name();
    let pronoun = investigator.id.pronoun();
    if !investigator.weapon {
        let mut outcome = EncounterOutcome::new(
            CounterFate::Stays,
            format!(
                "{name} rencontre des Fanatiques : sans arme, {pronoun} doit rebrousser chemin."
            ),
        );
        outcome.pushed_back = true;
        return outcome;
    }
    let roll = dice.d6() + u8::from(investigator.medallion);
    let mut outcome = match roll {
        7 => EncounterOutcome::new(
            CounterFate::Removed,
            format!("{name} rencontre des Fanatiques et les tue sans y laisser sa raison."),
        ),
        6 => {
            let mut killed = EncounterOutcome::new(
                CounterFate::Removed,
                format!("{name} rencontre des Fanatiques et les tue, mais perd 2 SAN."),
            );
            killed.san_loss = 2;
            killed
        }
        _ => {
            let mut repelled = EncounterOutcome::new(
                CounterFate::Stays,
                format!(
                    "{name} menace des Fanatiques de son arme, mais leur détermination est la plus forte : {pronoun} rebrousse chemin."
                ),
            );
            repelled.pushed_back = true;
            return repelled;
        }
    };
    outcome.kills_fanatics = true;
    outcome.event_allowed = true;
    outcome
}

fn things_of_the_mist(investigator: &Investigator) -> EncounterOutcome {
    let mut outcome = EncounterOutcome::new(
        CounterFate::Reserve,
        format!(
            "{} rencontre des Choses de la Brume : {} perd 1 SAN et errera dans la brume au prochain tour.",
            investigator.id.name(),
            investigator.id.pronoun()
        ),
    );
    outcome.san_loss = 1;
    outcome.lost_turns = 1;
    outcome
}

fn inhabitants(investigator: &Investigator) -> EncounterOutcome {
    let name = investigator.id.name();
    if !investigator.medallion {
        let mut outcome = EncounterOutcome::new(
            CounterFate::Stays,
            format!(
                "{name} croise des Habitants exaspérés par les Fanatiques, sans être inquiété{}.",
                investigator.id.agreement()
            ),
        );
        outcome.event_allowed = true;
        return outcome;
    }
    let mut outcome = EncounterOutcome::new(
        CounterFate::Reserve,
        format!(
            "Les Habitants prennent {} pour un Fanatique à cause de son médaillon : {} le perd, est chassé{} de la ville et ne pourra pas y revenir au prochain tour.",
            investigator.id.inner_name(),
            investigator.id.pronoun(),
            investigator.id.agreement()
        ),
    );
    outcome.loses_medallion = true;
    outcome.pushed_back = true;
    outcome.forbids_city = true;
    outcome
}

fn dreams(investigator: &Investigator) -> EncounterOutcome {
    let mut outcome = EncounterOutcome::new(
        CounterFate::Reserve,
        format!(
            "Les Rêves envoyés par le professeur entraînent {} dans les Contrées du Rêve : {} perd 2 SAN.",
            investigator.id.inner_name(),
            investigator.id.pronoun()
        ),
    );
    outcome.san_loss = 2;
    outcome
}

fn storm(investigator: &Investigator) -> EncounterOutcome {
    let mut outcome = EncounterOutcome::new(
        CounterFate::Removed,
        format!(
            "Une Tempête bloque {} dans la ville pendant deux tours.",
            investigator.id.inner_name()
        ),
    );
    outcome.lost_turns = 2;
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dice::ScriptedDice;
    use crate::domain::investigator_id::InvestigatorId;

    fn investigator(weapon: bool, sign: bool, medallion: bool) -> Investigator {
        let mut investigator = Investigator::new(InvestigatorId::Barnes, 12);
        investigator.weapon = weapon;
        investigator.sign = sign;
        investigator.medallion = medallion;
        investigator
    }

    #[test]
    fn ghouls_cost_less_with_a_sign_and_die_to_a_weapon() {
        let mut dice = ScriptedDice::new(&[], 1);
        let unarmed = resolve(
            CounterKind::Ghouls,
            &investigator(false, false, false),
            &mut dice,
        );
        assert_eq!((unarmed.san_loss, unarmed.fate), (4, CounterFate::Stays));
        let signed = resolve(
            CounterKind::Ghouls,
            &investigator(false, true, false),
            &mut dice,
        );
        assert_eq!(signed.san_loss, 3);
        let mut dice = ScriptedDice::new(&[5, 2], 1);
        let armed = resolve(
            CounterKind::Ghouls,
            &investigator(true, true, false),
            &mut dice,
        );
        assert_eq!((armed.san_loss, armed.fate), (1, CounterFate::Reserve));
        assert!(armed.event_allowed);
        let armed = resolve(
            CounterKind::Ghouls,
            &investigator(true, false, false),
            &mut dice,
        );
        assert_eq!(armed.san_loss, 3);
    }

    #[test]
    fn deep_ones_take_the_sign() {
        let mut dice = ScriptedDice::new(&[], 1);
        let signed = resolve(
            CounterKind::DeepOnes,
            &investigator(false, true, false),
            &mut dice,
        );
        assert!(signed.loses_sign && signed.san_loss == 1);
        assert_eq!(signed.fate, CounterFate::Reserve);
        let plain = resolve(
            CounterKind::DeepOnes,
            &investigator(true, false, false),
            &mut dice,
        );
        assert_eq!((plain.san_loss, plain.fate), (3, CounterFate::Stays));
    }

    #[test]
    fn fanatics_push_back_or_die_with_the_medallion_bonus() {
        let mut dice = ScriptedDice::new(&[6, 5, 5], 1);
        let unarmed = resolve(
            CounterKind::Fanatics,
            &investigator(false, false, true),
            &mut dice,
        );
        assert!(unarmed.pushed_back && unarmed.fate == CounterFate::Stays);
        let clean = resolve(
            CounterKind::Fanatics,
            &investigator(true, false, true),
            &mut dice,
        );
        assert!(clean.kills_fanatics && clean.san_loss == 0);
        assert_eq!(clean.fate, CounterFate::Removed);
        let costly = resolve(
            CounterKind::Fanatics,
            &investigator(true, false, false),
            &mut dice,
        );
        assert!(costly.pushed_back);
        let medallion = resolve(
            CounterKind::Fanatics,
            &investigator(true, false, true),
            &mut dice,
        );
        assert!(medallion.kills_fanatics && medallion.san_loss == 2);
    }

    #[test]
    fn inhabitants_only_chase_medallion_holders() {
        let mut dice = ScriptedDice::new(&[], 1);
        let chased = resolve(
            CounterKind::Inhabitants,
            &investigator(false, false, true),
            &mut dice,
        );
        assert!(chased.loses_medallion && chased.pushed_back && chased.forbids_city);
        let ignored = resolve(
            CounterKind::Inhabitants,
            &investigator(false, false, false),
            &mut dice,
        );
        assert!(ignored.event_allowed && !ignored.pushed_back);
    }

    #[test]
    fn mist_dreams_and_storm() {
        let mut dice = ScriptedDice::new(&[], 1);
        let someone = investigator(false, false, false);
        let mist = resolve(CounterKind::ThingsOfTheMist, &someone, &mut dice);
        assert_eq!((mist.san_loss, mist.lost_turns), (1, 1));
        let dreams = resolve(CounterKind::Dreams, &someone, &mut dice);
        assert_eq!((dreams.san_loss, dreams.fate), (2, CounterFate::Reserve));
        let storm = resolve(CounterKind::Storm, &someone, &mut dice);
        assert_eq!((storm.lost_turns, storm.fate), (2, CounterFate::Removed));
    }
}
