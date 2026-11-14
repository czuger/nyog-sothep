use crate::domain::event_effect::EventEffect;
use crate::domain::event_table::{EVENT_ROWS, EventTable};
use crate::domain::investigator::Investigator;
use crate::domain::investigator_ai::investigator_weights::InvestigatorWeights;
use crate::domain::item::Item;
use crate::domain::professor_belief::ProfessorBelief;
use crate::domain::professor_question::ProfessorQuestion;
use crate::domain::turn::GameState;

/// What the other questions are worth: the Fanatics are where the hunters strike them, and
/// knowing the encounters spares traps.
const FANATICS_QUESTION_WORTH: f64 = 1.5;
const SEA_QUESTION_WORTH: f64 = 1.0;
const NYOG_QUESTION_WORTH: f64 = 1.0;
const ENCOUNTER_QUESTION_WORTH: f64 = 0.5;

/// The table whose events are worth most to the investigator, on average over their dice.
pub(super) fn choose_table(
    state: &GameState,
    weights: &InvestigatorWeights,
    investigator: usize,
) -> EventTable {
    let me = &state.investigators[investigator];
    let odds = roll_odds(me.event_dice());
    let bits = ProfessorBelief::of(state).size().max(1) as f64;
    let bits = bits.log2();
    let worth = |table: EventTable| -> f64 {
        (1..=EVENT_ROWS)
            .map(|roll| {
                odds[usize::from(roll)] * value(state, weights, me, bits, &table.effect(roll))
            })
            .sum()
    };
    match worth(EventTable::Three) > worth(EventTable::Two) {
        true => EventTable::Three,
        false => EventTable::Two,
    }
}

/// The chance of each total (index 0 to 18) with `dice` six-sided dice.
fn roll_odds(dice: u8) -> Vec<f64> {
    let mut odds = vec![0.0; usize::from(EVENT_ROWS) + 1];
    odds[0] = 1.0;
    for _ in 0..dice {
        let mut next = vec![0.0; odds.len()];
        for (total, chance) in odds.iter().enumerate() {
            for face in 1..=6 {
                if let Some(slot) = next.get_mut(total + face) {
                    *slot += chance / 6.0;
                }
            }
        }
        odds = next;
    }
    odds
}

/// What an event is worth to an investigator, in rough SAN points; `bits` measures how little
/// the investigators know of the professor's position.
fn value(
    state: &GameState,
    weights: &InvestigatorWeights,
    me: &Investigator,
    bits: f64,
    effect: &EventEffect,
) -> f64 {
    // Under that SAN every point counts double.
    let san_weight = if f64::from(me.san) <= weights.low_san {
        2.0
    } else {
        1.0
    };
    // Once Nyog'Sothep is out, the spell and the sign it takes to survive it matter most.
    let rare = match state.nyog.is_summoned() {
        true => weights.sign_or_spell_worth_summoned,
        false => weights.sign_or_spell_worth,
    };
    let gains = [
        (
            matches!(effect.gains, Some(Item::Weapon(_))) && !me.weapon,
            weights.weapon_worth,
        ),
        (effect.loses_weapon && me.weapon, -weights.weapon_worth),
        (
            matches!(effect.gains, Some(Item::Protection(_))) && !me.sign,
            rare,
        ),
        (
            effect.gains == Some(Item::Medallion) && !me.medallion,
            weights.medallion_worth,
        ),
        (effect.gains == Some(Item::Spell) && !me.spell, rare),
        (effect.goes_back, -1.0),
        (effect.skips_next_turn, -2.0),
        (effect.breaks_car, -1.0),
        (effect.plays_again, 1.0),
    ];
    let items: f64 = gains
        .iter()
        .filter(|(applies, _)| *applies)
        .map(|(_, worth)| worth)
        .sum();
    let question = effect.question.map_or(0.0, |question| {
        question_worth(state, weights, me, bits, question)
    });
    f64::from(effect.san) * san_weight + items + question
}

/// What making the professor answer is worth: the bits a position answer removes from what the
/// investigators do not know, the other answers by kind. Once Nyog'Sothep is out he stands in
/// plain sight and nothing is worth asking.
fn question_worth(
    state: &GameState,
    weights: &InvestigatorWeights,
    me: &Investigator,
    bits: f64,
    question: ProfessorQuestion,
) -> f64 {
    if state.nyog.is_summoned() {
        return 0.0;
    }
    let hunter = if me.weapon {
        weights.hunter_info_factor
    } else {
        1.0
    };
    let position = |named: f64| hunter * weights.bit_worth * (bits - named.log2()).max(0.0);
    match question {
        ProfessorQuestion::RevealPosition
        | ProfessorQuestion::RevealPositionAndFanatics
        | ProfessorQuestion::RevealAndMoveInTheOpen => position(1.0),
        ProfessorQuestion::NameCities(count) => position(f64::from(count)),
        ProfessorQuestion::NameFanaticsCities => hunter * FANATICS_QUESTION_WORTH,
        ProfessorQuestion::AnnounceSeaMoves if !state.sea_announcements => SEA_QUESTION_WORTH,
        ProfessorQuestion::AnnounceSeaMoves => 0.0,
        ProfessorQuestion::NyogSothepInTwoCities { .. } => NYOG_QUESTION_WORTH,
        ProfessorQuestion::EncountersInTwoCities | ProfessorQuestion::EncounterInOneCity => {
            ENCOUNTER_QUESTION_WORTH
        }
        // Only on the fan-made tables, which are not chosen.
        ProfessorQuestion::WithinThreeCities | ProfessorQuestion::CountFanaticsCities => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_odds_of_two_dice() {
        let odds = roll_odds(2);
        assert!((odds[7] - 6.0 / 36.0).abs() < 1e-9);
        assert!((odds.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }
}
