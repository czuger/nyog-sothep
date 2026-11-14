use serde::{Deserialize, Serialize};

use crate::domain::event_effect::EventEffect;
use crate::domain::item::{Item, REVOLVER, SIGN};
use crate::domain::professor_question::ProfessorQuestion;

/// The two investigator event tables of the rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventTable {
    /// Table 2.
    Two,
    /// Table 3.
    Three,
}

/// Number of rows of each event table (the most three dice can roll).
pub const EVENT_ROWS: u8 = 18;

impl EventTable {
    /// The rules' number of the table.
    pub fn number(self) -> u8 {
        match self {
            EventTable::Two => 2,
            EventTable::Three => 3,
        }
    }

    /// Reads a table number typed on a form.
    pub fn from_number(number: &str) -> Option<EventTable> {
        match number {
            "2" => Some(EventTable::Two),
            "3" => Some(EventTable::Three),
            _ => None,
        }
    }

    /// The effect of a roll (1 to 18).
    pub fn effect(self, roll: u8) -> EventEffect {
        match self {
            EventTable::Two => table_two(roll),
            EventTable::Three => table_three(roll),
        }
    }

    /// The French text of a row, addressed to the investigator as in the rules.
    pub fn text(self, roll: u8) -> &'static str {
        let index = usize::from(roll.clamp(1, EVENT_ROWS) - 1);
        match self {
            EventTable::Two => TABLE_TWO_TEXTS[index],
            EventTable::Three => TABLE_THREE_TEXTS[index],
        }
    }
}

fn effect(san: i32) -> EventEffect {
    EventEffect {
        san,
        ..EventEffect::default()
    }
}

fn ask(question: ProfessorQuestion) -> EventEffect {
    EventEffect {
        question: Some(question),
        ..EventEffect::default()
    }
}

fn table_two(roll: u8) -> EventEffect {
    match roll {
        1 => EventEffect {
            goes_back: true,
            ..effect(0)
        },
        2 => EventEffect {
            gains: Some(REVOLVER),
            ..effect(-2)
        },
        3 => EventEffect {
            gains: Some(SIGN),
            ..effect(-2)
        },
        4 => EventEffect {
            skips_next_turn: true,
            ..effect(5)
        },
        5 => ask(ProfessorQuestion::NameCities(3)),
        6 => EventEffect {
            gains: Some(Item::Spell),
            ..effect(-2)
        },
        7 => EventEffect {
            loses_weapon: true,
            ..effect(-2)
        },
        8 => EventEffect {
            breaks_car: true,
            ..effect(0)
        },
        9 => EventEffect {
            gains: Some(Item::Medallion),
            plays_again: true,
            ..effect(0)
        },
        10 => ask(ProfessorQuestion::NameCities(2)),
        11 => EventEffect {
            gains: Some(REVOLVER),
            ..effect(-2)
        },
        12 => EventEffect {
            gains: Some(Item::Spell),
            ..effect(-3)
        },
        13 => ask(ProfessorQuestion::RevealPosition),
        14 => EventEffect {
            plays_again: true,
            ..effect(0)
        },
        15 => ask(ProfessorQuestion::AnnounceSeaMoves),
        16 => ask(ProfessorQuestion::EncountersInTwoCities),
        17 => ask(ProfessorQuestion::NyogSothepInTwoCities { optional: false }),
        _ => ask(ProfessorQuestion::RevealPositionAndFanatics),
    }
}

fn table_three(roll: u8) -> EventEffect {
    match roll {
        1 => EventEffect {
            skips_next_turn: true,
            ..effect(5)
        },
        2 => EventEffect {
            gains: Some(REVOLVER),
            ..effect(-3)
        },
        3 => EventEffect {
            gains: Some(Item::Spell),
            ..effect(-3)
        },
        4 => EventEffect {
            plays_again: true,
            ..effect(0)
        },
        5 => EventEffect {
            gains: Some(SIGN),
            ..effect(-2)
        },
        6 => ask(ProfessorQuestion::NameCities(3)),
        7 => EventEffect {
            goes_back: true,
            ..effect(0)
        },
        8 => EventEffect {
            gains: Some(Item::Medallion),
            ..effect(-3)
        },
        9 => ask(ProfessorQuestion::NameCities(2)),
        10 => EventEffect {
            gains: Some(Item::Spell),
            plays_again: true,
            ..effect(-2)
        },
        11 => EventEffect {
            gains: Some(REVOLVER),
            plays_again: true,
            ..effect(0)
        },
        12 => effect(-2),
        13 => ask(ProfessorQuestion::RevealAndMoveInTheOpen),
        14 => ask(ProfessorQuestion::NameFanaticsCities),
        15 => EventEffect {
            question: Some(ProfessorQuestion::NyogSothepInTwoCities { optional: true }),
            ..effect(-2)
        },
        16 => ask(ProfessorQuestion::EncounterInOneCity),
        17 => EventEffect {
            plays_again: true,
            ..effect(0)
        },
        _ => EventEffect {
            loses_weapon: true,
            ..effect(-2)
        },
    }
}

const TABLE_TWO_TEXTS: [&str; 18] = [
    "Absorbé par d'inquiétantes réflexions, vous vous perdez sur les petites routes : revenez à la ville que vous venez de quitter.",
    "Vous découvrez un repaire de Profonds : vous perdez 2 SAN, mais vous trouvez une arme.",
    "Vous rencontrez un vieillard. Il vous donne un signe protecteur et vous parle longuement des créatures de l'au-delà : vous perdez 2 SAN.",
    "Un psychanalyste de renom s'intéresse à vos troubles : vous passez votre prochain tour, mais gagnez 5 SAN.",
    "Des voyageurs vous parlent du Professeur : il doit citer trois villes dont l'une est sa position actuelle (s'il est en mer, il le dit).",
    "Dans le grenier d'une vieille maison, un grimoire en mauvais état explique comment renvoyer Nyog'Sothep : vous apprenez le sort de renvoi, qui vous coûte 2 SAN.",
    "Vous errez la nuit dans un cimetière infesté de Goules : vous perdez 2 SAN et votre arme (si vous en avez une).",
    "Votre voiture refuse d'avancer : vous passez le tour suivant, sauf si vous prenez la mer.",
    "Vous trouvez des traces du Professeur et un médaillon. Stimulé par ces découvertes, vous rejouez immédiatement.",
    "Le Professeur fait parler de lui : il doit citer deux villes dont l'une est sa position actuelle (s'il est en mer, il le dit).",
    "Vous êtes agressé par des adorateurs déments : vous perdez 2 SAN, mais gagnez une arme.",
    "Vous trouvez un vieux manuscrit qui parle à demi-mot de Nyog'Sothep : vous apprenez le sort de renvoi, mais perdez 3 SAN.",
    "Le Professeur doit révéler sa position actuelle.",
    "Vous gagnez du temps sur le Professeur : rejouez.",
    "Le Professeur Petersen doit désormais signaler ses mouvements en mer (sans préciser la zone).",
    "Citez deux villes au Professeur : il doit vous dire s'il y a des rencontres dans celles-ci, sans les préciser.",
    "Citez deux villes au Professeur : il doit vous dire si Nyog'Sothep doit apparaître dans l'une d'elles.",
    "Le Professeur doit révéler sa position et le nombre de Fanatiques déjà en place.",
];

const TABLE_THREE_TEXTS: [&str; 18] = [
    "Vous séjournez chez un psychanalyste : vous passez votre prochain tour, mais gagnez 5 SAN.",
    "Vous observez des Goules près d'une vieille crypte : vous perdez 3 SAN, mais trouvez une arme.",
    "Dans une malle, de vieux manuscrits expliquent comment renvoyer l'Immonde : ce sort de renvoi vous coûte 3 SAN.",
    "Grâce à de bonnes informations, vous voilà sur une piste prometteuse : rejouez.",
    "Un ermite à l'esprit un peu dérangé vous donne un signe protecteur et vous parle de créatures insensées : vous perdez 2 SAN.",
    "Vous entendez parler de Petersen : il doit citer trois villes dont l'une est sa position actuelle (s'il est en mer, il le dit).",
    "Impossible de vous y retrouver dans ce labyrinthe de petites routes : retournez à la ville d'où vous venez pour demander votre chemin.",
    "Près d'une sortie d'égout, un cadavre de Profond : à côté, vous trouvez un médaillon de Fanatique, mais perdez 3 SAN.",
    "Des nouvelles du Professeur : il doit citer deux villes dont l'une est sa position actuelle (s'il est en mer, il le dit).",
    "La lecture d'un gros livre d'occultisme vous apprend le sort de renvoi : vous perdez 2 SAN, mais rejouez immédiatement.",
    "En discutant avec la police locale, vous passez pour un ami : vous gagnez une arme et rejouez.",
    "Votre sommeil est peuplé de créatures étranges et de visions incroyables : vous perdez 2 SAN.",
    "Des voyageurs ont vu Petersen récemment : il doit se montrer et faire son prochain déplacement à découvert.",
    "Un prêtre vous parle des Fanatiques qui infestent la région : le Professeur doit citer deux villes dont l'une contient des Fanatiques.",
    "À la bibliothèque, un vieux livre de légendes locales parle de créatures de la brume : vous perdez 2 SAN, mais pouvez citer deux villes au Professeur, qui vous dit si Nyog'Sothep doit apparaître dans l'une d'elles.",
    "Les plans du Professeur s'éclaircissent : citez-lui une ville. S'il y a une rencontre, il doit vous la préciser.",
    "Vous sentez que vous approchez du but : rejouez.",
    "Perdu dans vos réflexions en pleine campagne, vous êtes surpris par la nuit et une brume anormale : vous perdez votre arme (si vous en avez une) et 2 SAN.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_row_has_an_effect_and_a_text() {
        for table in [EventTable::Two, EventTable::Three] {
            for roll in 1..=EVENT_ROWS {
                let _ = table.effect(roll);
                assert!(!table.text(roll).is_empty());
            }
        }
    }

    #[test]
    fn rows_match_the_rules() {
        let row = EventTable::Two.effect(9);
        assert!(row.gains == Some(Item::Medallion) && row.plays_again);
        let row = EventTable::Three.effect(10);
        assert!(row.gains == Some(Item::Spell) && row.plays_again && row.san == -2);
        assert_eq!(
            EventTable::Three.effect(15).question,
            Some(ProfessorQuestion::NyogSothepInTwoCities { optional: true })
        );
        assert_eq!(EventTable::Two.effect(4).san, 5);
        assert!(EventTable::Two.effect(8).breaks_car);
    }
}
