# Migration plan: Rails → Rust

Goal: replace the unfinished Rails 5.2 / PostgreSQL application with a Rust web application. The stack and
code organisation copy `../the-wizard-war`. The rules in `RULES.md` are the specification. The Rails code is
used as a reference only (map data, French texts, a few ideas); it is not ported line by line.

**The roles are reversed compared with the Rails app.** It is a two-player game: the human always plays the
**four Investigators**, and the computer plays **Professor Petersen**.

This file is the plan only. Nothing below has been started.

---

## 1. Decisions taken

| Topic | Decision | Why |
|---|---|---|
| Approach | Rewrite from scratch on the rules; read the Rails code only for data and ideas | About a third of the events are missing (`method_missing`), Table 1 is simplified, the professor's combat choices are automatic, and the roles are reversed anyway (see §7) |
| Players | The human plays the 4 Investigators; an AI plays Professor Petersen | Project decision |
| Stack | Same as the-wizard-war: Rust edition 2024, `axum` 0.8, `askama`, `sqlx` (SQLite, runtime queries, embedded migrations), `tokio`, `tower-http` (fs, set-header), `serde`, `rand`, `thiserror`, `tracing` / `tracing-subscriber` | Known to work; one human per game, so SQLite is enough |
| Front end | Server-rendered HTML, no front-end build: `static/style.css` plus small vanilla JS (`static/board.js`) | Same as the-wizard-war |
| Layers | `domain/` (pure rules, no I/O) → `db/` (one record per table) → `service/` (one use case per file) → `web/` (router, handlers, pages) | Same as the-wizard-war |
| Code style | Copy the-wizard-war's "Development guidelines" (imports, docstrings, errors, 50-line functions, one type per file, English code and French UI) into the new `CLAUDE.md` | Consistency between the two projects |
| Rails history | Nothing is lost when Rails files are deleted: `master` still holds the full Rails app (`git show master:<path>`) | Allows an aggressive cleanup |

### Rule readings decided

These go into `RULES.md` "Notes on unclear points" (phase 7) and are enforced by the domain.

| Point | Reading |
|---|---|
| Number of investigators | Always **4** (the rules' 2-player setup). At creation, 4 of the 6 investigator tokens are drawn at random; the other 2 are not used |
| Turn order | The rules' 2-player case: the professor plays; then the human **moves all four investigators**; then the **action phase** is resolved for each investigator in turn (encounter, else event, then any unavoidable fight). A "play again" event gives that investigator another move and action phase at once |
| Stacking | **At most one counter per city.** The professor cannot place a counter in a city that already holds one, hidden or discovered |
| Placing without moving | Allowed: the professor may place a counter in the city he stays in, provided it holds no counter |
| Investigator going mad | They become a Fanatics counter (discovered) where they stand. If that city already holds a counter, that counter goes back to the reserve and the Fanatics take its place, so the stacking rule has no exception |
| At sea | **No events and no fights at sea.** An investigator who ends their move in a sea zone skips the action phase; the professor and an investigator in the same sea zone do not meet |
| "Less than three cities away" | A distance of **2 at most** (Nyog'Sothep summoning, Teleportation) |
| Destroyed cities | Investigators can no longer enter or cross them. **The professor and Nyog'Sothep can** |
| Banishment result −1 | The human (the investigators) chooses the city where Nyog'Sothep is placed |
| Hand limit | As written: the professor always draws, and discards down to 5 if he holds 6 |
| Killed by Nyog'Sothep | An investigator without a protective sign who meets Nyog'Sothep is **dead**: no Fanatics counter and no −5 "became a Fanatic" score. They count as lost for the "fewer than three Investigators" check |
| Fewer than three investigators before the summoning | The game goes on; Nyog'Sothep can no longer be banished, so its summoning is an immediate professor win. The game is lost at once only if all four investigators are out (gone mad or dead) |
| Visible professor information | The human sees the professor's **HP** (the investigators dealt the damage) but **not the size of his hand** |

## 2. Target layout

```
Cargo.toml            # crate `nyog_sothep`
migrations/           # 0001_init.sql, …  (never edit an applied migration)
src/
  main.rs lib.rs config.rs error.rs
  domain/             # pure rules and the professor AI, unit-tested
  db/                 # records + connect/migrate
  service/            # use cases: create_game, move_investigators, choose_table, answer_prompt, …
  web/                # router, handlers, GamePage, Screen
templates/            # askama: base, lobby, game, partials/, screens/
static/               # style.css, board.js
img/                  # board.jpg + token-*.png (cut from work/board/)
tests/                # integration tests on in-memory SQLite
work/                 # magazine scans, board extraction scripts and sources (kept)
RULES.md README.md CLAUDE.md LICENSE
```

Environment variables, as in the-wizard-war: `DATABASE_URL` (default `sqlite://data/nyog_sothep.db`),
`BIND_ADDR` (`127.0.0.1:3000`), `ASSET_ROOT` (`.`) and `RUST_LOG`.

## 3. Domain design (`src/domain/`)

### 3.1 Map

- `LocationId`: an enum of the 47 cities and 6 sea zones (`gulf_of_maine`, `massachusetts_bay`,
  `atlantic_ocean`, `buzzards_bay`, `nantucket_sound`, `cape_cod_bay`).
- `map.rs`: a static table, as in the-wizard-war's `board.rs`. It holds the French display name, the kind
  (city or sea zone), the port flag (16 ports), the pixel position on `img/board.jpg`, the adjacency list and
  the border crossings.
- Source data: `app/models/game_core/map/data/{locations,destinations,borders_crossings}.yml`. It must be
  **checked against the board scan**, because the YAML was generated from the hand-typed `work/*.txt` files.
- The pixel coordinates in the YAML match the old `app/assets/images/map.jpg` (1205×1437). The straightened
  `work/board/map.png` (2242×2732) is not a uniform scale of it, so every position has to be measured again.
  This is the same job as the-wizard-war's "remap the map coordinates" commit.
- `travel.rs`: legal moves by road and by sea (port → adjacent zone → zone or port). Destroyed cities block
  investigators only. It also provides BFS distances and shortest paths, used by the summoning radius,
  Teleportation, Flying Horror, the banishment chase and the professor AI.

### 3.2 Rules and pieces

- `CounterKind`: Ghouls, Deep Ones, Fanatics, Things of the Mist, Inhabitants, Dreams, Storm, Flying Horror and
  Teleportation, with their quantities. Each counter is in one place: the `Reserve`, the professor's `Hand`, a
  `Placed { city, discovered }` slot, or `Removed`. A city holds at most one placed counter.
- `table1.rs`: the full Table 1, including the "his choice" rolls (2, 5, 9, 10). It rerolls when no counter
  is available, applies the draw-then-discard hand limit, and deals 4 starting counters.
- `investigator.rs`: identity (one of the six names and genders of the Rails app, with French name forms for
  log agreement; 4 are drawn at random per game), SAN (`3d6 + 3`), weapon, protective sign, medallion, spell,
  position, previous position, a forbidden city (Inhabitants, border result 6), lost turns (psychoanalyst, car,
  Storm, Things of the Mist), "seen Nyog'Sothep", and status (active, gone mad, dead).
- `encounter.rs`: one function per counter that returns its outcome (SAN change, item change, push back, counter
  fate). Deep Ones may only be placed in ports.
- `event_table.rs`: the rules' **Table 2 and Table 3**, all 18 rows each. The dice are 1d6, 2d6 or 3d6
  depending on sign and medallion. Each row maps to an `EventEffect` value, so effects are data rather than
  methods dispatched by name.
- `professor_fight.rs` (investigator against professor) and `nyog_sothep.rs` (summoning conditions, joint
  movement, the 1–2 "no move" roll, destroyed cities, encounter SAN loss, banishment table).
- `scoring.rs`: the investigator scoring table, shown at the end.
- `Dice` trait, injected everywhere: `rand` in production and a scripted roller in tests. It replaces the
  Rails `Kernel.rand` / Mocha stubbing.

### 3.3 Turn engine (the key piece)

A pure state machine:

```
Engine::run(state, &mut dice, &mut professor_ai) -> Vec<LogEntry>
```

It advances until the game ends or the **human** has to decide. The pending decision is a `Prompt` value stored
in the game state. The answer arrives as `Engine::answer(state, Answer, …)`, which resumes the same loop. The
professor's decisions never stop the loop: the engine calls the professor AI (§3.4) synchronously, so the human
never waits.

Human prompts (the investigators):

| Prompt | Rule |
|---|---|
| Movement of the four investigators, given together: stay, consult a psychoanalyst (+1d6 SAN, not when forced to stay), road move, or sea move. The engine then rolls border crossings in order | Movement |
| For each investigator in the action phase, with no encounter or an encounter that allows it: Table 2 or Table 3, chosen before the roll | Action phase |
| A "play again" investigator: a new move, then its action phase | Tables 2 and 3 |
| Fight choices: weapon or protective sign; with a medallion, hide or attack | Fighting the Professor |
| Name cities: two cities, "encounters there?" (Table 2 #16); two cities, "Nyog'Sothep to appear there?" (Table 2 #17, Table 3 #15 optional); one city, "which encounter?" (Table 3 #16) | Tables 2 and 3 |
| Banishment result −1: the city where Nyog'Sothep goes | Banishment |

Professor decisions, taken by the AI inside the loop:
- Start city and Nyog'Sothep city, at creation.
- Each turn: Flying Horror or Teleportation before moving, the move, which counter to place, "his choice"
  Table 1 rolls, and the discard.
- Whether to reveal himself and attack on meeting an investigator.
- Retreat and push-back cities, and the two cities an unarmed investigator is moved.
- Bluff answers: name 2 or 3 cities, one of which is his position; name two cities, one of which holds
  Fanatics.
- When to summon Nyog'Sothep, and where to move with it. The move is forced along the shortest path once
  three spell holders gather.

**Honesty is enforced by the engine, not trusted to the AI.**
- Factual questions are answered by the engine itself: revealing his position, encounters in the named cities,
  "is Nyog'Sothep to appear there", the number of Fanatics, and announcing sea movements.
- AI bluff answers are validated: one named city must be his real position, or must hold Fanatics. When he is
  at sea, the answer is "at sea" automatically.

The state, including a cursor over the investigators and a sub-step, can be fully serialised, so a pause in the
middle of the action phase survives across HTTP requests.

### 3.4 Professor AI (`domain/professor_ai/`)

The Rails app has no professor AI; this part is new.

- **Input:** a `ProfessorView`. It holds the full board the professor knows: his own secrets and the public
  state of the investigators (positions, SAN, items, spells). It also holds the investigators' **notebook**,
  the clues the investigators have gathered (§4). Future dice are never visible to it.
- **Exposure.** `ProfessorBelief` is the set of locations the investigators can still place him in. Each turn
  the set grows by one move (or by Flying Horror's radius, or by sea moves). It is then narrowed by every clue:
  spotted positions, named cities, sea announcements, discovered counters and fights. The AI uses it to measure
  how well hidden it is and to choose moves and bluffs that keep the set large. The Rails `ProfPositionFinder`
  is the ancestor of this idea.
- **Plan:**
  - Choose a central Nyog'Sothep city with many roads and a start city away from Nantucket.
  - Plan a route that drops Fanatics in five different cities and ends within 2 cities of the Nyog'Sothep city.
  - Avoid armed investigators and attack unarmed ones when the 1d6 or the odds favour it.
  - Place harmful counters on the investigators' likely paths: Deep Ones in ports, Inhabitants where medallion
    holders go.
  - Use Teleportation to push threats away and Flying Horror to escape after being spotted.
  - After summoning, hunt the investigators who have no protective sign.
- **Variety:** pick among the near-best options, as in the-wizard-war's `pick_near_best`, so that the
  professor is not predictable.
- One level at first. Levels and temperaments like the-wizard-war's are a later step.

## 4. Persistence (`migrations/`, `src/db/`)

SQLite, `STRICT` tables, with the-wizard-war's patterns: load a record, change it, `save`; `games.version` bumped on
each save; `BEGIN IMMEDIATE` write transactions. Planned tables:

- `games`: token, phase/status, turn, current prompt (JSON) and engine cursor, Nyog'Sothep appearance city
  (secret), its current location, the banishment meeting city, the sea-announcement flag, and version.
- `professor`: HP, location, previous location, and the AI's plan state (target route, Fanatics cities).
- `investigators`: four rows per game (the drawn tokens), with the fields of §3.2.
- `counters`: kind and state (reserve/hand/placed/removed), city, discovered flag. A partial unique index on
  `(game_id, city)` for placed counters enforces the stacking rule.
- `route_sheet`: the professor's stage notes (turn, city, counter placed). They stay hidden from the human until
  the summoning (which reveals the Fanatics) and the end of the game (full proof).
- `destroyed_cities`.
- `clues`: the investigators' notebook (turn, kind, cities, answer). It is shown to the human and rebuilt into
  `ProfessorBelief` for the AI.
- `event_log`: French text already rendered, plus turn and actor, newest first. The Rails `LLog` stored i18n
  keys; we store the final text, as the-wizard-war does. Log lines must never reveal the professor's secrets.

## 5. Web (`src/web/`, `templates/`, `static/`)

- **Lobby** (`/`): a "Nouvelle partie" form. It redirects to the private game URL `/games/{id}/{token}`, as
  the-wizard-war does.
- **Hidden information.** `GameView` holds only what the investigators may see. The professor's position,
  hand, undiscovered counters, route sheet and Nyog'Sothep city must never reach the HTML: no hidden fields and
  no JS data. Page tests check this.
- **Game page:**
  - One screen per `Prompt` (`Screen::of`, `templates/screens/*.html`).
  - Full-window map with tokens at their real size, scaled by `board.js`.
  - The four investigators' moves are staged on the map and sent together: click an investigator, then a
    highlighted destination, as `board.js` already does for pawns in the-wizard-war. There are also buttons
    for "stay" and "psychoanalyst".
  - City choices for the naming events and for the banishment −1 result are made by clicking on the map.
  - On the map: discovered counters, destroyed cities, Nyog'Sothep once it appears, and a "last seen" professor
    marker with its turn.
  - Floating panels:
    - a banner with the turn, the professor's HP (never his hand size), and each investigator's SAN, items
      and spell;
    - the notebook of clues;
    - the log.
  - End screen: the result, the professor's revealed route sheet and Nyog'Sothep city, and the investigators'
    scoring.
- The handler flow is the-wizard-war's: parse the form, call a service, then redirect with 303, or re-render
  with a French error (400). `/games/{id}/version` polling is not needed: the professor plays inside the
  request.

## 6. Assets

- `work/board/map.png` (2242×2732, 9.9 MB) → `img/board.jpg` (high-quality JPEG, same resolution).
- `work/board/tokens/*.png` (82 slices) → `img/token-<kind>.png`, one per counter kind plus each investigator,
  the professor, Nyog'Sothep, destroyed city and the items. Duplicate slices (`goules_1…8`) are only kept if
  they really differ.
- Keep `boat.png` from `app/assets/images/tokens/` as the marker for an investigator at sea. Everything else in
  `app/assets/images` (stock photo, old car, `.xcf`, old `map.jpg`) is dropped.
- Keep `work/board/scripts/` and `work/board/tokens_sheet.png` as the extraction sources, and add a short
  `work/board/README.md` like the-wizard-war's `work/extraction`.

## 7. What to take from the Rails app (then delete it)

Take:
- The map data (§3.1).
- The French texts in `config/locales/fr/`, used as a base for log and UI wording (events, encounters, fights,
  Nyog'Sothep, the investigators' declined names, m/f agreement). They need proofreading: there are typos such
  as "perds" and "gagnerea".
- `ProfPositionFinder` (clue tracking) and the BFS helpers, as ideas for `ProfessorBelief` and `travel`.
- The test scenarios in `test/models/*_test.rb`, reused as a checklist for domain unit tests (encounters, BFS,
  Nyog'Sothep, fights).
- The investigator movement AI (`InvestigatorMovement`, `InvestigatorMovementTarget`). It is not needed in the
  game any more, but its ideas can drive the scripted investigator bot used in simulation tests (§8, phase 4).

Rails behaviour that is **not** carried over, because it diverges from `RULES.md` or from the decisions above:
- The roles: the human played the professor, and the investigators were the AI.
- Six investigators instead of four.
- Table 1 simplified (`MONSTERS_ROLL_MAP` has no choices: Flying Horror, Teleportation and Storm are never
  drawn).
- Starting SAN `3d6` instead of `3d6 + 3`; 5 starting counters instead of 4; draws only when holding fewer than
  5 instead of a draw then discard.
- Table 2 #8, #9, #14–17 and Table 3 #4, #10, #11, #13–17 not implemented.
- Professor combat decisions automatic (he always attacks unarmed investigators, with no 1d6 roll and no
  retreat or push-back).
- Border result 6 not distinguished from 5.
- Summoning radius `≤ 3` instead of "less than three cities" (≤ 2).
- Banishment −1 result not implemented.

## 8. Phases

Each phase ends with `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` passing, then a
commit.

0. **Cleanup and scaffold**
   - Delete the Rails application: `app/`, `bin/`, `config/`, `config.ru`, `db/`, `lib/`, `log/`, `public/`,
     `test/`, `vendor/`, `Gemfile`, `Gemfile.lock`, `Rakefile`, `.rubocop.yml`, `.ruby-version`, `.travis.yml`,
     `postgresql.txt`, `push_coverage.sh` and `graphs/` (AASM diagrams).
   - Copy what §7 takes into the new code first, or read it later from `master`.
   - Create `Cargo.toml`, `.gitignore` (`/target`, `*.db*`, `/data/`, `.DS_Store`), an empty layered crate,
     and a `main.rs` serving a placeholder page.
   - Update `CLAUDE.md`'s "Game model" line to the reversed roles.
1. **Map**: `LocationId`, the static map table checked against the scan, re-measured coordinates on the new
   board image, `travel` and BFS, with tests (adjacency is symmetric, every port touches a sea zone, borders
   are listed both ways).
2. **Rules primitives**: `Dice`, counters and Table 1, investigator state, encounters, Tables 2 and 3, fights,
   Nyog'Sothep, scoring. All of it is pure and unit-tested with scripted dice.
3. **Turn engine**: `GameState`, `Prompt`/`Answer`, `Engine::run`/`answer`, win and lose detection. Tests play
   whole rounds with scripted dice, scripted investigator answers and a scripted professor.
4. **Professor AI**: `ProfessorView`, `ProfessorBelief`, and the plan and decision functions.
   - Unit tests: the AI's choices are always legal and its bluffs always pass the engine's checks.
   - Simulation test: the AI plays many full games against a simple scripted investigator bot. They must finish
     with no error, and the win rate and game length are printed so that balance can be tuned.
5. **Persistence**: migration `0001_init.sql`, the records, and loading and saving `GameState`, with round-trip
   tests on in-memory SQLite.
6. **Services and web**: lobby, game creation, the movement, table-choice, fight and city-naming handlers, the
   game page with screens, map, panels and log, the end screen, and `tests/` integration tests (as in the
   wizard-war's `game_flow.rs` and `game_page.rs`). The page tests include a check that no hidden professor
   information appears in the HTML.
7. **Assets and polish**: cut `img/`, styling, `README.md` (run instructions and environment table) and a new
   `CLAUDE.md` describing the Rust architecture. `RULES.md` gets a "Notes on unclear points" section like the
   one in the-wizard-war (§1, §9). This file is then removed.

Possible later steps, out of scope for now: professor AI levels and temperaments; an optional on-map overlay
of the cities where the professor can still be.

## 9. Open points

None at the moment. Every rule ambiguity raised so far is settled in "Rule readings decided" (§1). New ones
found during implementation are added here first, with a proposal.
