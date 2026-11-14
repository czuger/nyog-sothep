use crate::domain::turn::game_state::GameState;
use crate::domain::turn::outcome::Outcome;

/// Everyone lost, or Nyog'Sothep is out and nobody left in the game knows the spell. The
/// Fanatics never run out, so they can no longer be wiped out.
pub(crate) fn outcome(state: &GameState) -> Option<Outcome> {
    let active = state.active_investigators();
    let spell_known = active.iter().any(|index| state.investigators[*index].spell);
    if active.is_empty() {
        Some(Outcome::InvestigatorsLost)
    } else if state.nyog.is_summoned() && !spell_known {
        Some(Outcome::NyogSothepReigns)
    } else {
        None
    }
}
