# Nyog'Sothep — problèmes du jeu original

Ce document relève les problèmes du jeu tel qu'il est implémenté aujourd'hui : les règles parues dans
*Jeux & Stratégie* (1988), avec les lectures choisies pour les points obscurs (voir la fin de `RULES.md`). Il sert
de base à la proposition de version fan-made (`fan_mode_proposition.md`).

## Méthode

- 100 parties jouées entre les deux IA (`PetersenAi` contre `InvestigatorsAi`), graines 0 à 99 de
  `tests/simulation.rs`. Pour chacune, les 80 dernières lignes du journal ont été lues (`SIMULATION_TRACE=<graine>`).
- Le bilan sur 150 parties que le test de simulation imprime (`cargo test --test simulation -- --nocapture`).
- Une relecture de l'interface web (gabarits, `board.js`, `style.css`).

Les chiffres mesurent les règles telles que ces deux IA les jouent : un joueur humain ferait d'autres choix.
Ils montrent des tendances, pas un verdict absolu.

## Chiffres mesurés

Bilan sur 150 parties : 56,3 tours en moyenne.

| Fin de partie | Parties |
|---|---:|
| Nyog'Sothep renvoyé | 71 |
| Fanatiques exterminés (règle maison) | 44 |
| Nyog'Sothep règne | 35 |
| Professeur tué | 0 |
| Les quatre investigateurs perdus | 0 |
| Inachevées après 120 tours | 0 |

Le professeur finit avec **12,6 PV sur 14** en moyenne.

Sur les 100 parties lues :

- 52 renvois, 25 victoires par extermination des Fanatiques, 23 règnes de Nyog'Sothep, 0 professeur tué ;
- 119 lancers du sort de renvoi pour 52 victoires, soit **2,3 tentatives par renvoi réussi** ;
- 36 folies dans les 23 défaites, chacune juste après une perte de SAN (événement, Rêves, rencontre ou
  Nyog'Sothep) ;
- environ **4 questions par partie qui n'apprennent plus rien** après l'invocation ;
- environ **19 consultations de psychanalyste par partie** dans la fin des parties gagnées par renvoi ;
- des « rejouez » en chaîne : dans la partie 0, le docteur Armitage tire quatre fois de suite l'événement de la
  police, qui lui donne chaque fois une arme qu'il a déjà.

## Problèmes de règles

### 1. Le professeur est presque impossible à tuer

Tuer le professeur est l'une des deux victoires des investigateurs. Elle n'est jamais arrivée en 150 parties :

- il a 14 PV et n'en perd que 2 ou 3 par combat gagné. Il faut donc 5 à 7 combats victorieux ;
- après chaque blessure, il recule d'une ville et redevient invisible ;
- sa position est secrète, et il n'est forcé de combattre qu'avec 1 ou 2 au dé quand il entre dans une ville
  occupée ;
- seul un investigateur armé le blesse.

Un professeur prudent évite les armes, et cette victoire disparaît en pratique.

### 2. Le sort de renvoi est une loterie

Le sort réussit si 1d6 moins le nombre de participants donne −2 ou moins :

| Participants | Victoire | Renvoi partiel | Échec |
|---:|---:|---:|---:|
| 3 | 1/6 | 1/6 | 4/6 (dont 2/6 avec perte de SAN) |
| 4 | 2/6 | 1/6 | 3/6 (dont 1/6 avec perte de SAN) |

Chaque tentative coûte 2 à 4 SAN pour avoir affronté l'Immonde, et une ville détruite. Les investigateurs
choisissent seulement le nombre de participants, le dé décide du reste.

### 3. Une partie sans fin quand les Fanatiques sont exterminés

S'il reste moins de cinq pions Fanatiques, Nyog'Sothep ne peut plus être invoqué. Comme le professeur n'est
presque jamais tué, la partie ne peut plus finir, et les règles ne disent rien de ce cas. Une règle maison donne
la victoire aux investigateurs. Elle termine 29 % des parties, mais elle n'est pas dans le jeu original.

### 4. Des événements devenus inutiles

Après l'invocation, le professeur se montre sur la carte, les Fanatiques sont révélés et la ville d'apparition
est connue. Pourtant ces lignes posent encore leurs questions :

- **Table 2** : 5 et 10 (citer des villes dont sa position), 13 (révéler sa position), 15 (annoncer ses
  déplacements en mer), 17 (ville d'apparition), 18 (position et nombre de Fanatiques) ;
- **Table 3** : 6 et 9 (citer des villes dont sa position), 13 (se révéler), 14 (une ville de Fanatiques),
  15 (ville d'apparition, qui coûte en plus 2 SAN).

De même, un objet gagné quand l'investigateur en a déjà un ne sert à rien (une arme, un signe et un médaillon au
plus).

### 5. Les « rejouez » en chaîne

Six lignes font rejouer : Table 2 n° 9 et 14, Table 3 n° 4, 10, 11 et 17. L'investigateur peut rester sur place
et tirer un nouvel événement, puis rejouer encore. Ces chaînes allongent le tour et permettent de « farmer » les
tables.

### 6. Le repos sans limite

La SAN n'a pas de plafond, et le psychanalyste rend 1d6 SAN à chaque tour. En attendant Nyog'Sothep (qui reste
sur place sur 1 ou 2), les investigateurs se reposent tour après tour. La fin de partie devient une attente.

### 7. La folie par usure

Les défaites viennent surtout des pertes de SAN des événements et des Rêves, que le dé décide. Les choix du
professeur y comptent moins.

### 8. Des parties longues

56 tours en moyenne. Face à un humain qui mène quatre investigateurs, cela fait des centaines de décisions par
partie.

### 9. L'équilibre

Contre ces IA, les investigateurs gagnent environ 77 % des parties. Plus d'un tiers de ces victoires (44 sur
115) vient de la règle maison des Fanatiques exterminés.

## Problèmes d'interface

- **Un rechargement par décision.** Le choix de table se fait investigateur par investigateur, sans valeur par
  défaut (`src/domain/turn/action_phase.rs:56-63`). Un tour coûte environ 5 rechargements, et jusqu'à 9.
- **Une table choisie à l'aveugle.** `templates/screens/table.html` n'affiche que « Table 2 / Table 3 » et le
  nombre de dés. Les lignes des tables (`TABLE_TWO_TEXTS` / `TABLE_THREE_TEXTS` dans
  `src/domain/event_table.rs`), leurs probabilités et les lignes inutiles ne sont pas montrées.
- **Le tour de l'ordinateur se lit seulement dans le journal.** Les nouvelles lignes ne sont pas mises en évidence,
  rien n'est animé hors de la relecture, et il n'y a pas d'`aria-live`.
- **Les positions possibles du professeur ne sont pas sur la carte.** `ProfessorBelief` n'est pas utilisé par le
  web. Le carnet n'est qu'une liste de texte.
- **Des chiffres clés absents.** Les chances du sort de renvoi selon le nombre de participants et le seuil de
  folie (0 SAN) ne sont affichés nulle part.
- **Pas de confirmation.** Valider les déplacements sans rien toucher laisse tout le monde sur place sans
  avertissement. « Consulter un psychanalyste » n'est pas proposé par défaut.
- **Les villes à citer des investigateurs.** `templates/screens/cities.html` n'a pas de `data-pick-limit`,
  contrairement à `professor_names.html` : cocher trop de villes n'est refusé que par le serveur. Il utilise des
  cases à cocher même pour une seule ville.
- **Une réponse refusée** est réaffichée avec une erreur 400 à l'adresse du POST, si bien qu'un rafraîchissement
  renvoie le formulaire.
- **Sur téléphone**, sous 760 px, les panneaux couvrent la carte. Le plateau est réduit à environ 0,17×, les cibles
  font environ 9 px, et les noms ne s'affichent qu'au survol.
- **Au clavier**, les cases des villes sont invisibles (`opacity: 0`) et aucun style `:focus` n'existe.
