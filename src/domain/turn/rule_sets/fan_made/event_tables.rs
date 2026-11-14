//! The three d20 event tables of the fan-made version, picked by the number of Fanatics cities.

use crate::domain::event_effect::EventEffect;
use crate::domain::item::{Gear, Item, Protection, REVOLVER, SIGN, Weapon};
use crate::domain::professor_question::ProfessorQuestion;

/// One of the three fan-made event tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FanTable {
    /// « Premières pistes »: 0 or 1 Fanatics city.
    A,
    /// « La menace grandit »: 2 or 3 Fanatics cities.
    B,
    /// « Les brumes de l'Immonde »: 4 Fanatics cities or more.
    C,
}

/// Rows of each table (one d20).
pub(crate) const FAN_EVENT_ROWS: u8 = 20;

type Row = (EventEffect, &'static str);

impl FanTable {
    /// The table used while Fanatics are in place in this many cities.
    pub(crate) fn for_fanatics_cities(cities: usize) -> FanTable {
        match cities {
            0 | 1 => FanTable::A,
            2 | 3 => FanTable::B,
            _ => FanTable::C,
        }
    }

    /// The letter and title of the table.
    pub(crate) fn title(self) -> &'static str {
        match self {
            FanTable::A => "A — Premières pistes",
            FanTable::B => "B — La menace grandit",
            FanTable::C => "C — Les brumes de l'Immonde",
        }
    }

    /// The effect of a roll (1 to 20).
    pub(crate) fn effect(self, roll: u8) -> EventEffect {
        self.row(roll).0
    }

    /// The French text of a row, addressed to the investigator.
    pub(crate) fn text(self, roll: u8) -> &'static str {
        self.row(roll).1
    }

    fn row(self, roll: u8) -> Row {
        let index = usize::from(roll.clamp(1, FAN_EVENT_ROWS) - 1);
        match self {
            FanTable::A => TABLE_A[index],
            FanTable::B => TABLE_B[index],
            FanTable::C => TABLE_C[index],
        }
    }
}

const fn row(san: i32, gains: Option<Item>, question: Option<ProfessorQuestion>) -> EventEffect {
    EventEffect {
        san,
        gains,
        loses_weapon: false,
        goes_back: false,
        skips_next_turn: false,
        breaks_car: false,
        plays_again: false,
        question,
    }
}

const fn san(san: i32) -> EventEffect {
    row(san, None, None)
}

const fn find(san: i32, item: Item) -> EventEffect {
    row(san, Some(item), None)
}

const fn ask(question: ProfessorQuestion) -> EventEffect {
    row(0, None, Some(question))
}

const fn back() -> EventEffect {
    EventEffect {
        goes_back: true,
        ..san(0)
    }
}

const fn disarmed(loss: i32) -> EventEffect {
    EventEffect {
        loses_weapon: true,
        ..san(loss)
    }
}

const SHOTGUN: Item = Item::Weapon(Weapon::Shotgun);
const MNAR_STONE: Item = Item::Protection(Protection::MnarStone);
const DYNAMITE: Item = Item::Gear(Gear::Dynamite);
const PASS: Item = Item::Gear(Gear::Pass);
const LAUDANUM: Item = Item::Gear(Gear::Laudanum);
const AMULET: Item = Item::Gear(Gear::Amulet);
const POWDER: Item = Item::Gear(Gear::IbnGhaziPowder);
const NECRONOMICON: Item = Item::Gear(Gear::Necronomicon);
const NYOG_IN_TWO: ProfessorQuestion = ProfessorQuestion::NyogSothepInTwoCities { optional: false };

const TABLE_A: [Row; 20] = [
    (
        san(-1),
        "Vous passez une nuit blanche à ressasser d'inquiétantes lectures : vous perdez 1 SAN.",
    ),
    (
        back(),
        "Vous vous perdez sur les petites routes : retournez à la ville que vous venez de quitter.",
    ),
    (
        find(0, REVOLVER),
        "Un armurier vous vend sans poser de questions un revolver.",
    ),
    (
        find(-1, SIGN),
        "Un vieil homme vous remet un signe protecteur et vous parle des créatures de l'Au-delà : vous perdez 1 SAN.",
    ),
    (
        ask(ProfessorQuestion::NameCities(3)),
        "Des voyageurs parlent du Professeur : il doit citer trois villes dont l'une est sa position (s'il est en mer, il le dit).",
    ),
    (
        find(0, PASS),
        "Un douanier bavard vous prend en sympathie : vous recevez un laissez-passer.",
    ),
    (
        find(0, LAUDANUM),
        "Un pharmacien vous confie une fiole de laudanum.",
    ),
    (
        ask(ProfessorQuestion::WithinThreeCities),
        "Le journal local évoque un étrange universitaire : le Professeur dit s'il se trouve à trois villes ou moins de vous.",
    ),
    (
        ask(ProfessorQuestion::EncountersInTwoCities),
        "À la bibliothèque, citez deux villes au Professeur : il dit s'il s'y trouve des rencontres, sans les préciser.",
    ),
    (
        find(-2, Item::Medallion),
        "Un rôdeur vous agresse : vous perdez 2 SAN, mais lui arrachez un médaillon de Fanatique.",
    ),
    (
        find(0, SHOTGUN),
        "Un fermier vous prête son fusil de chasse.",
    ),
    (
        san(2),
        "Un médecin de campagne vous remet d'aplomb : vous regagnez 2 SAN.",
    ),
    (
        ask(ProfessorQuestion::NameCities(2)),
        "Le Professeur fait parler de lui : il doit citer deux villes dont l'une est sa position (s'il est en mer, il le dit).",
    ),
    (
        disarmed(-2),
        "Vous errez de nuit dans un cimetière infesté de Goules : vous perdez 2 SAN et votre arme (si vous en avez une).",
    ),
    (
        find(0, AMULET),
        "Chez un brocanteur, une amulette de rêveur attire votre regard.",
    ),
    (
        ask(ProfessorQuestion::EncounterInOneCity),
        "Une lettre d'un collègue : citez une ville au Professeur. S'il s'y trouve une rencontre, il doit la préciser.",
    ),
    (
        ask(NYOG_IN_TWO),
        "Dans le bureau de Petersen à l'université, citez deux villes : il dit si Nyog'Sothep doit apparaître dans l'une d'elles.",
    ),
    (
        find(-2, Item::Spell),
        "Dans le grenier d'une vieille maison, un grimoire en lambeaux : vous apprenez le sort de renvoi et perdez 2 SAN.",
    ),
    (
        ask(ProfessorQuestion::AnnounceSeaMoves),
        "Des pêcheurs le connaissent : le Professeur doit désormais annoncer ses déplacements en mer (sans préciser la zone).",
    ),
    (
        ask(ProfessorQuestion::RevealPosition),
        "Un coup de chance : le Professeur doit révéler sa position.",
    ),
];

const TABLE_B: [Row; 20] = [
    (
        san(-2),
        "Votre sommeil est peuplé de visions incroyables : vous perdez 2 SAN.",
    ),
    (
        back(),
        "Des Fanatiques vous ont repéré et vous filent : retournez à la ville que vous venez de quitter.",
    ),
    (
        find(-2, REVOLVER),
        "Vous découvrez un repaire de Profonds : vous perdez 2 SAN, mais trouvez un revolver.",
    ),
    (
        find(-2, SIGN),
        "Un ermite un peu dérangé vous donne un signe protecteur : vous perdez 2 SAN.",
    ),
    (
        ask(ProfessorQuestion::NameFanaticsCities),
        "Un prêtre vous parle des Fanatiques : le Professeur doit citer deux villes dont l'une contient des Fanatiques.",
    ),
    (
        row(
            0,
            Some(Item::Medallion),
            Some(ProfessorQuestion::CountFanaticsCities),
        ),
        "Un Fanatique repenti vous donne son médaillon : le Professeur dit dans combien de villes des Fanatiques sont en place.",
    ),
    (
        find(0, DYNAMITE),
        "Des contrebandiers vous cèdent un bâton de dynamite.",
    ),
    (
        find(-1, MNAR_STONE),
        "Chez un antiquaire d'Arkham, une pierre étoilée de Mnar : vous perdez 1 SAN à son contact.",
    ),
    (
        ask(ProfessorQuestion::NameCities(2)),
        "Des nouvelles du Professeur : il doit citer deux villes dont l'une est sa position (s'il est en mer, il le dit).",
    ),
    (
        find(-2, REVOLVER),
        "Des adorateurs déments vous attaquent : vous perdez 2 SAN, mais gagnez un revolver.",
    ),
    (
        find(-1, POWDER),
        "Dans un laboratoire abandonné, un flacon de poudre d'Ibn-Ghazi : vous perdez 1 SAN.",
    ),
    (
        find(-3, Item::Spell),
        "Dans une malle, de vieux manuscrits : vous apprenez le sort de renvoi et perdez 3 SAN.",
    ),
    (
        find(-2, Item::Spell),
        "Un gros livre d'occultisme : vous apprenez le sort de renvoi et perdez 2 SAN.",
    ),
    (
        san(3),
        "Un psychanalyste renommé s'intéresse à vous : vous regagnez 3 SAN.",
    ),
    (
        disarmed(-2),
        "Une brume surnaturelle vous surprend dans la campagne : vous perdez 2 SAN et votre arme (si vous en avez une).",
    ),
    (
        ask(ProfessorQuestion::EncounterInOneCity),
        "Les plans du Professeur s'éclaircissent : citez-lui une ville. S'il s'y trouve une rencontre, il doit la préciser.",
    ),
    (
        row(-1, None, Some(NYOG_IN_TWO)),
        "Un livre de légendes locales : vous perdez 1 SAN, puis citez deux villes. Il dit si Nyog'Sothep doit apparaître dans l'une d'elles.",
    ),
    (
        find(0, SHOTGUN),
        "La police locale vous prend pour un ami : vous recevez un fusil de chasse.",
    ),
    (
        ask(ProfessorQuestion::RevealAndMoveInTheOpen),
        "Des voyageurs viennent de voir Petersen : il doit se révéler et faire son prochain déplacement à découvert.",
    ),
    (
        ask(ProfessorQuestion::RevealPositionAndFanatics),
        "Le Professeur doit révéler sa position et le nombre de villes où des Fanatiques sont en place.",
    ),
];

const TABLE_C: [Row; 20] = [
    (
        san(-3),
        "Des visions de l'Immonde vous assaillent : vous perdez 3 SAN.",
    ),
    (
        back(),
        "Un brouillard épais vous égare : retournez à la ville que vous venez de quitter.",
    ),
    (
        disarmed(-2),
        "Des formes bougent dans la brume : vous perdez 2 SAN et votre arme (si vous en avez une).",
    ),
    (
        find(0, SHOTGUN),
        "Un vétéran décidé à se battre vous donne son fusil de chasse.",
    ),
    (
        find(0, SIGN),
        "Un prêtre bénit votre route et vous remet un signe protecteur.",
    ),
    (
        find(0, DYNAMITE),
        "Sur un chantier désert, une caisse de dynamite.",
    ),
    (
        find(-2, MNAR_STONE),
        "Dans une crypte, une pierre étoilée de Mnar : vous perdez 2 SAN.",
    ),
    (
        find(-3, NECRONOMICON),
        "Un exemplaire du Necronomicon : sa lecture vous coûte 3 SAN.",
    ),
    (
        find(-2, Item::Spell),
        "Des pages très pénibles à lire expliquent comment renvoyer l'Immonde : vous apprenez le sort de renvoi et perdez 2 SAN.",
    ),
    (
        find(-3, Item::Spell),
        "Un vieux manuscrit parle de Nyog'Sothep : vous apprenez le sort de renvoi et perdez 3 SAN.",
    ),
    (
        find(-1, Item::Spell),
        "Un occultiste vous enseigne le sort de renvoi : vous perdez 1 SAN.",
    ),
    (
        ask(ProfessorQuestion::NameCities(2)),
        "Des nouvelles du Professeur : il doit citer deux villes dont l'une est sa position (s'il est en mer, il le dit).",
    ),
    (
        ask(ProfessorQuestion::RevealPositionAndFanatics),
        "Le Professeur doit révéler sa position et le nombre de villes où des Fanatiques sont en place.",
    ),
    (
        ask(ProfessorQuestion::NameFanaticsCities),
        "Un prêtre vous parle des Fanatiques : le Professeur doit citer deux villes dont l'une contient des Fanatiques.",
    ),
    (
        ask(NYOG_IN_TWO),
        "Citez deux villes au Professeur : il dit si Nyog'Sothep doit apparaître dans l'une d'elles.",
    ),
    (
        ask(ProfessorQuestion::EncounterInOneCity),
        "Citez une ville au Professeur. S'il s'y trouve une rencontre, il doit la préciser.",
    ),
    (
        find(-2, Item::Medallion),
        "Près d'une sortie d'égout, un cadavre de Profond : vous trouvez un médaillon de Fanatique, mais perdez 2 SAN.",
    ),
    (
        find(0, LAUDANUM),
        "Un médecin vous laisse une fiole de laudanum.",
    ),
    (
        san(3),
        "Les habitants se rassemblent à l'église et vous redonnent courage : vous regagnez 3 SAN.",
    ),
    (
        find(0, POWDER),
        "Dans les affaires d'un Fanatique, un flacon de poudre d'Ibn-Ghazi.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_follows_the_fanatics_cities() {
        let tables: Vec<FanTable> = (0..=6).map(FanTable::for_fanatics_cities).collect();
        use FanTable::*;
        assert_eq!(tables, vec![A, A, B, B, C, C, C]);
    }

    #[test]
    fn no_row_replays_or_loses_a_turn() {
        for table in [FanTable::A, FanTable::B, FanTable::C] {
            for roll in 1..=FAN_EVENT_ROWS {
                let effect = table.effect(roll);
                assert!(!effect.plays_again && !effect.skips_next_turn && !effect.breaks_car);
                assert!(!table.text(roll).is_empty());
            }
        }
    }

    #[test]
    fn rows_match_the_proposal() {
        assert_eq!(FanTable::A.effect(11).gains, Some(SHOTGUN));
        assert_eq!(
            FanTable::A.effect(8).question,
            Some(ProfessorQuestion::WithinThreeCities)
        );
        let repentant = FanTable::B.effect(6);
        assert_eq!(repentant.gains, Some(Item::Medallion));
        assert_eq!(
            repentant.question,
            Some(ProfessorQuestion::CountFanaticsCities)
        );
        assert_eq!(FanTable::C.effect(8).gains, Some(NECRONOMICON));
        assert_eq!(FanTable::C.effect(8).san, -3);
        assert!(FanTable::C.effect(3).loses_weapon);
    }
}
