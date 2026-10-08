# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A web implementation of *Nyog'Sothep ou les brumes de l'Immonde*, a board game published in *Jeux & Stratégie*
(1988). The human leads the **four Investigators**; the computer plays **Professor Petersen**. `RULES.md` holds the
translated rules and, at the end, the readings this implementation chose for the unclear points. Scans of the
magazine pages and of the board are in `work/`; `work/board/` holds the straightened board, the counter slices and
the scripts that made them (see its README).

It started as an unfinished Rails 5.2 application; the `rust_conversion` branch replaced it with this Rust crate
(`nyog_sothep`), rewritten from the rules. `master` still holds the Rails code (`git show master:<path>`).

## Commands

```sh
cargo run                                     # serve on http://127.0.0.1:3000 (DATABASE_URL, BIND_ADDR, ASSET_ROOT, RUST_LOG)
cargo test                                    # unit tests + tests/ (in-memory SQLite)
cargo test --test game_flow <name>            # one integration test
cargo test --test simulation -- --nocapture   # the professor's AI against a bot; SIMULATION_DEBUG=1 describes stalled games
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## Architecture

Stack: axum (HTTP), askama (server-rendered HTML in `templates/`), sqlx on SQLite. No front-end build:
`static/style.css` and `static/board.js`, images served from `img/`.

Layers, from the inside out:

- **`src/domain/`**: pure rules, no I/O, unit-tested.
  - The map: `LocationId` (47 cities and 6 sea zones, stable snake_case codes), `map` (static table transcribed
    from `img/board.jpg`: French name, port flag, pixel position, `ROADS`, `SEA_LINKS`, `BORDER_CROSSINGS`) and
    `travel` (cached neighbour lists, BFS `distances`/`within`/`first_steps_towards` over `Network::All` (roads and
    sea, one step per turn) or `Network::Roads`).
  - Rules primitives: `Dice` (`RandomDice`, and `ScriptedDice` for tests, which counts 1 to 6 after its script so
    that Table 1 rerolls end), `CounterKind`/`Counter`/`CounterPlace`/`CounterPool` (the 44 counters: reserve, hand,
    placed hidden or face up, removed), `table1` (2d6 roll with the "his choice" rows, `HAND_LIMIT`),
    `Investigator`/`InvestigatorId` (French name forms and gender agreement)/`InvestigatorStatus`, `encounter`
    (one outcome per counter), `EventTable`/`EventEffect`/`ProfessorQuestion` (Tables 2 and 3 as data, with the
    French row texts), `FightChoice` and `professor_fight`, `NyogSothep` and the banishment table, `Score`.
  - `turn/`: the turn engine, a pure state machine. `GameState` holds everything (turn, `Phase` cursor, pending
    `Prompt`, professor, investigators, counters, Nyog'Sothep, destroyed cities, route sheet, the investigators'
    notebook of `Clue`s). `Engine::new_game` sets up and plays until the first prompt; `Engine::answer` applies the
    human's `Answer` (moves of all four, table choice, named cities, play-again move, fight choice, banishment
    city) and `Engine::run` plays on until the human must decide again or the game ends. Every refused answer
    leaves the state unchanged (`AnswerError`, French texts). The professor's decisions go through the
    `ProfessorBrain` trait, synchronously, so the human never waits; the engine checks every answer of the brain
    and falls back to a legal one, and answers factual questions itself, so the professor cannot lie. Log lines
    (`LogLine`, French) never reveal his secrets. The engine is split by topic (`professor_turn`, `nyog_turn`,
    `moves`, `action_phase`, `fights`, `questions`, `sanity`, `setup`); `engine_tests.rs` plays whole rounds with
    scripted dice and a scripted professor.
  - `professor_ai/`: `PetersenAi`, the computer's professor. It keeps no memory: each decision is computed from
    the state. `ProfessorBelief` rebuilds from the notebook the set of locations where the investigators can
    still place him (one move per turn, three with the Flying Horror, narrowed by each clue). It picks a central
    appearance city with many roads, drops Fanatics in new cities away from armed investigators while staying
    near that city, closes in once five Fanatics cities are in place, lays traps suited to the investigators near
    them, attacks only the defenceless, teleports armed investigators away, bluffs with decoy cities that are far
    from him but still believable, and hunts the investigators without a protective sign with Nyog'Sothep.
    `pick_near_best` varies its moves. `tests/simulation.rs` plays 150 games against a simple bot and checks
    every answer of the AI.
- **`src/db/`**: one record type per table (`GameRecord`, `ProfessorRecord`, `InvestigatorRecord`,
  `CounterRecord`, `RouteStageRecord`, `DestroyedCityRecord`, `ClueRecord`, `EventRecord`), with async functions
  taking `&mut SqliteConnection`; `game_store::load_state`/`save_state` rebuild and write a whole `GameState`
  (`games.version` is bumped on every save; the phase, prompt and clues are stored as JSON). Queries use runtime
  `sqlx::query`/`query_as` (no `DATABASE_URL` at build time). `db::connect` runs `migrations/` (embedded with
  `sqlx::migrate!`); schema changes go in a new migration file, never by editing an applied one. A partial unique
  index enforces one placed counter per city.
- **`src/service/`**: one use case per file. `create_game` draws four investigators and a private token, lets the
  AI set up and play its first turn, and stores everything. `answer_prompt` opens `begin_write`
  (`BEGIN IMMEDIATE`), loads the state, runs the engine with real dice and `PetersenAi`, saves the state and
  appends the log. `GameView` is the investigators' view of a game: it never holds the professor's position
  (only the last sighting), hand, hidden counters, route sheet or Nyog'Sothep's city while they are secret; at
  the end it reveals the route sheet, every counter and the scores.
- **`src/web/`**: `router.rs` lists the routes (`/`, `POST /games`, `/games/{id}/{token}` and one `POST` per
  prompt: `moves`, `table`, `cities`, `play-again`, `fight`, `banishment`). Handlers read the form as name/value
  pairs, call `answer_prompt`, then redirect (303) to the game page, or show it again with the French error
  (400). `GamePage` (`templates/game.html`) draws the full-window map (`BoardView`: every location as a click
  target, tokens stacked side by side on their location, investigators at sea on a boat, the professor's last
  sighting) with floating panels: on the left the banner (professor's hit points and known position, the
  investigators with SAN, items, spell and lasting effects) and the screen of the pending prompt (`Screen::of`,
  `templates/screens/*.html`), on the right the notebook and the log. Investigator moves are made in selects
  named `move_{index}`, which `board.js` also fills when the player clicks an investigator and then a highlighted
  destination (it draws an arrow per move). Cities are named by ticking inputs drawn on the map (`name="city"`,
  joined to `#screen-form` with the `form` attribute). `board.js` scales the board to the width left between the
  panels and remembers which panels are open.

`img/board.jpg` (2242×2732) is `work/board/map.png`; the counters are `img/token-<kind>.png` (counter codes),
`img/token-investigator-<code>.png` and a few others, all cut from the same photo (see `work/board/README.md`).

## Development guidelines

### Imports

- Prefer `crate::`-rooted paths for imports across modules; treat this as the Rust equivalent of "absolute imports."
- `super::` is acceptable within tightly nested module hierarchies (e.g. a tests submodule referencing its parent, or a submodule referencing its direct parent's types).
- Avoid deep `super::super::` chains — restructure the module tree instead.
- Avoid glob imports (`use foo::*;`) except for prelude modules explicitly designed for it, or in test modules (`use super::*;`).

### Type Annotations

- Annotate all `pub` function signatures explicitly (parameters and return type), even where inference would work.
- Let type inference handle local variable bindings inside function bodies.
- Prefer concrete types over generics unless genericity provides real, used value (called with more than one concrete type, or part of a public API meant to be generic).
- Avoid `Box<dyn Any>` and other type-erasure patterns unless strictly necessary; justify with a short comment.
- Avoid `Box<dyn Error>` for library code — prefer a concrete, well-typed error enum. `Box<dyn Error>` is acceptable at binary/application boundaries (e.g. `main`).

### Docstrings

- All `pub` items (functions, methods, structs, enums, traits) require a rustdoc comment (`///`), following standard rustdoc sections (`# Arguments`, `# Returns`, `# Errors`, `# Panics` as applicable).
- Private (non-`pub`) functions get a docstring only when the logic isn't obvious from the signature and name.
- Keep docstrings proportionate: short for simple functions, more detailed only when it adds real clarity.
- Do not restate information already conveyed by the signature (parameter types, obvious return values).
- Enforce at the crate level with `#![warn(missing_docs)]` (or `#![deny(missing_docs)]` for libraries).

### Comments

- Do not add comments to explain straightforward or self-evident code.
- Add comments only when the logic is genuinely complex, non-obvious, or could be misread.
- Keep comments short and synthetic — a brief note on intent or reasoning, not a narration of what the code does.
- Avoid verbose or redundant comments; never caption the obvious.

### Error Handling

- Use `Result<T, E>` for recoverable errors and `Option<T>` for absence. Do not use panics as control flow.
- Propagate errors with `?` rather than manual `match`/`if let` chains, unless the error needs to be transformed or contextualized at that point.
- Do not use `unwrap()` or `expect()` in library or production code paths. Reserve for tests, examples, and prototypes.
- If used outside those contexts because a failure is provably impossible, justify with a short comment (e.g. `// safe: regex is a compile-time constant`).
- Define proper error types (plain enums, or via `thiserror`) rather than stringly-typed errors (`Err("something failed".to_string())`) or blanket `Box<dyn Error>` in library code.
- Do not silently discard errors with `.ok()`, `let _ =`, or similar — handle, log, or propagate explicitly.
- `panic!` (and `unreachable!`, `todo!`, `unimplemented!` left in shipped code) is reserved for genuine invariant violations (bugs), not expected failure modes.

### Function Size & Decomposition

- Soft limit of 50 lines per function.
- If a function exceeds this limit, break it into smaller, well-named sub-functions, each with a single clear responsibility and its own concise docstring.
- Prefer composition of small functions over deeply nested logic or long procedural blocks.
- Prefer early returns / `?`-based flattening over deeply nested `match`/`if let` pyramids.

### Types & Modules

- One primary public type (struct, enum, or trait) per file, with its associated `impl` block(s).
- File name (snake_case) matches the type name (e.g. `struct UserAccount` → `user_account.rs`).
- Exceptions: small, tightly-coupled types may share the file with the primary type — e.g. an error enum returned by its methods, a config/builder struct feeding it, or a result type it produces.
- A closely-related family of small types with no independent identity (e.g. AST node variants, a state-machine's states) may live together in one file organized around the concept they jointly express, rather than one type per file. Split it out once it grows large enough to be its own topic.
- Traits implemented by multiple unrelated types get their own file; a trait implemented by only one type may be co-located with that type.

### File & Module Organization

- Avoid monolithic files. Split code by topic/responsibility into modules (`mod foo;`).
- Group related modules into a directory-based module tree (subpackages) when a topic grows large enough.
- Keep `mod.rs` / inline module-declaration files thin — `pub use` re-exports only, no implementation logic.
- Keep `lib.rs` / `main.rs` as a thin entry point: module declarations, top-level wiring, and re-exports — no business logic.

### General Principles

- Favor readability and small, testable units over cleverness.
- Prefer explicit over implicit in naming, typing, and structure.
- Split files as soon as they start doing "too much" rather than waiting for a refactor pass.
- Prefer composition over inheritance-like patterns; use traits for shared behavior, not deep hierarchies.
- Align with idiomatic Rust and clippy/rustfmt defaults rather than fighting them; deviate only when this style guide explicitly says so.

### Language

- Code is English, whatever language the prompt is in. Identifiers, function names, comments, docstrings, commit messages and log lines are written in English even when the request or the discussion is in French.
- User-facing text is French, and stays French. Error messages surfaced to users, CLI output, labels, templates — anything a reader sees is French, UTF-8, accents kept. Do not translate it, and write new user-facing text in French.
