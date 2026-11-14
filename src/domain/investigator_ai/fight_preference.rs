use crate::domain::fight_choice::FightChoice;

/// The best way to face the professor: shoot, then the sign, then the medallion.
pub(super) fn choose_fight(options: &[FightChoice]) -> FightChoice {
    [FightChoice::Weapon, FightChoice::Sign, FightChoice::Hide]
        .into_iter()
        .find(|choice| options.contains(choice))
        .unwrap_or(FightChoice::Unarmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_weapon_comes_first() {
        let options = [FightChoice::Sign, FightChoice::Weapon];
        assert_eq!(choose_fight(&options), FightChoice::Weapon);
        assert_eq!(choose_fight(&[FightChoice::Hide]), FightChoice::Hide);
        assert_eq!(choose_fight(&[FightChoice::Unarmed]), FightChoice::Unarmed);
    }
}
