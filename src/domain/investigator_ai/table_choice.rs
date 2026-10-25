use crate::domain::event_effect::EventEffect;
use crate::domain::event_table::{EVENT_ROWS, EventTable};
use crate::domain::investigator::Investigator;
use crate::domain::turn::GameState;

/// SAN under which every point counts double.
const LOW_SAN: i32 = 8;

/// The table whose events are worth most to the investigator, on average over their dice.
pub(super) fn choose_table(state: &GameState, investigator: usize) -> EventTable {
    let me = &state.investigators[investigator];
    let odds = roll_odds(me.event_dice());
    let worth = |table: EventTable| -> f64 {
        (1..=EVENT_ROWS)
            .map(|roll| odds[usize::from(roll)] * value(state, me, &table.effect(roll)))
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

/// What an event is worth to an investigator, in rough SAN points.
fn value(state: &GameState, me: &Investigator, effect: &EventEffect) -> f64 {
    let san_weight = if me.san <= LOW_SAN { 2.0 } else { 1.0 };
    // Once Nyog'Sothep is out, the spell and the sign it takes to survive it matter most.
    let (spell_worth, sign_worth) = match state.nyog.is_summoned() {
        true => (10.0, 10.0),
        false => (5.0, 5.0),
    };
    let gains = [
        (effect.gains_weapon && !me.weapon, 6.0),
        (effect.loses_weapon && me.weapon, -6.0),
        (effect.gains_sign && !me.sign, sign_worth),
        (effect.gains_medallion && !me.medallion, 3.0),
        (effect.learns_spell && !me.spell, spell_worth),
        (effect.goes_back, -1.0),
        (effect.skips_next_turn, -2.0),
        (effect.breaks_car, -1.0),
        (effect.plays_again, 1.0),
        (effect.question.is_some(), 2.0),
    ];
    let items: f64 = gains
        .iter()
        .filter(|(applies, _)| *applies)
        .map(|(_, worth)| worth)
        .sum();
    f64::from(effect.san) * san_weight + items
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
