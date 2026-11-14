# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A web implementation of *Nyog'Sothep ou les brumes de l'Immonde*, a board game published in *Jeux & Stratégie*
(1988). The human chooses a side: lead the **four Investigators** against the computer's **Professor Petersen**,
or play the professor against investigators led by the computer. `RULES.md` holds the
translated rules and, at the end, the readings this implementation chose for the unclear points. Scans of the
magazine pages and of the board are in `work/`; `work/board/` holds the straightened board, the counter slices and
the scripts that made them (see its README).

It started as an unfinished Rails 5.2 application; the `rust_conversion` branch replaced it with this Rust crate
(`nyog_sothep`), rewritten from the rules. `master` still holds the Rails code (`git show master:<path>`).

## Commands

```sh
cargo run                                     # serve on http://127.0.0.1:3000 (DATABASE_PATH, NYOG_SOTHEP_DATA_DIR, BIND_ADDR, NYOG_SOTHEP_SOCKET, RUST_LOG)
cargo test                                    # unit tests + tests/ (PoloDB in temporary folders)
cargo test --test game_flow <name>            # one integration test
cargo test --test simulation -- --nocapture   # the professor's AI against the investigators' AI; SIMULATION_DEBUG=1 describes stalled games, SIMULATION_TRACE=<seed> prints a game's log
cargo run --release --example tune_ai -- [rounds] [games]   # tunes both AIs' weights by self-play
cargo clippy --all-targets -- -D warnings
cargo fmt
cargo deploy render                           # resolved deploy settings, systemd unit and nginx snippet in target/cargo-deploy/ (local)
cargo deploy                                  # deploys to the server (also setup, rollback, status, logs): only when asked
```

Deployment: `cargo deploy` (see the README) runs the binary as a systemd user service on the Unix socket
`NYOG_SOTHEP_SOCKET`, behind nginx under `/nyog-sothep/` with the prefix stripped; `[package.metadata.deploy]` in
`Cargo.toml` describes it, the server values come from the `DEPLOY_*` variables.

## Architecture

Stack: axum (HTTP), askama (server-rendered HTML in `templates/`), PoloDB (embedded document database on RocksDB). No front-end build:
`static/style.css`, `static/board.js` and `static/replay.js`, images in `img/`, both embedded in the binary with
rust-embed (`web/embedded_assets.rs`, read from disk in debug builds). `main.rs` reads `AppConfig` (`config.rs`) and
calls `server::serve` (TCP, or the Unix socket when `NYOG_SOTHEP_SOCKET` is set, until SIGTERM/Ctrl-C).

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
  - `turn/`: the turn engine, a pure state machine. `GameState` holds everything (the `Side` the human plays,
    turn, `Phase` cursor, pending `Prompt`, professor, investigators, counters, Nyog'Sothep, destroyed cities,
    route sheet, the investigators' notebook of `Clue`s). Every decision of either side is a `Prompt`: the
    investigators' (moves of all four, table choice, named cities, play-again move, fight choice, banishment
    city) and the professor's, `Prompt::Professor(ProfessorPrompt)` (setup, Teleportation, move, attack, carry,
    retreat, summoning, placement, Table 1 choice, discard, Nyog'Sothep's step, named cities), each holding its
    options and parking the engine on the `Phase` where it resumes. `Engine::new_game` sets up and plays until
    the human's first prompt; `Engine::answer` applies the human's `Answer` and `Engine::run` plays on until the
    human must decide again or the game ends, answering on the way the prompts of the computer's side:
    `ProfessorBrain::answer` dispatches to one method per decision, and `InvestigatorBrain` only receives
    `GameState::public_copy` (no hand, hidden counter, route sheet or secret position), so it cannot cheat. The
    computer's answers go through the same checks as the human's; a refused one is replaced by a legal
    fallback. Every refused answer leaves the state unchanged (`AnswerError`, French texts); the engine answers
    factual questions itself and checks the cities the professor names, so he cannot lie. Log lines (`LogLine`,
    French) never reveal his secrets. The engine also records the replay (`replay.rs`): every log line is a
    `ReplayStep` too, with the dice rolled since the previous step (`RecordedDice` wraps the dice) and the
    `ReplayBoard` (every token, secrets included) when it changed; `reveal` adds a step only the replay shows
    (the professor's setup, placements, Table 1 draws, discards) and `log_revealing` shows the replay the secret
    a log line hides (his secret move or retreat). Changes made after a step's text (an event's effects) are
    amended into that step between two iterations of `run`. `Engine::into_history` returns both as a `History`.
    The engine is split by topic (`professor_turn`, `professor_counters`,
    `professor_answers`, `computer_players`, `nyog_turn`, `moves`, `action_phase`, `fights`, `questions`,
    `sanity`, `setup`, `replay`); `engine_tests.rs` plays whole rounds with scripted dice and a scripted professor, from
    either side.
  - `ProfessorBelief` rebuilds from the notebook the set of locations where the investigators can still place
    the professor (one move per turn, three with the Flying Horror, narrowed by each clue), and `weights` how
    likely each one is from public facts (he avoids armed investigators and the sea, lingers near face-up
    Fanatics); both AIs read it.
  - `professor_ai/`: `PetersenAi`, the computer's professor. It keeps no memory: each decision is computed from
    the state, and from `ProfessorBelief` to know how well hidden he is. It picks a central
    appearance city with many roads, drops Fanatics in new cities that are safe (`fanatics_safety`: far from
    every investigator, the armed most, and off busy crossroads) while staying near that city, closes in once
    five Fanatics cities are in place, lays traps suited to the investigators near them, attacks only the
    defenceless, teleports armed investigators away, bluffs with decoy cities that are far from him but still
    believable, joins Nyog'Sothep at once after the summoning, and with it hunts the investigators on land, those
    it can kill or drive mad and the spell holders first, never stepping onto three spell holders unless forced.
    `pick_near_best` varies its moves. Its weights are a `ProfessorWeights` (`PetersenAi::with_weights`).
  - `investigator_ai/`: `InvestigatorsAi`, the computer's four investigators, stateless too, one file per
    decision. Armed investigators share out the professor's likely positions (`Hunt`: the weighted belief now
    and after his next move, each hunter covering what the previous ones leave) and walk into face-up Fanatics
    to kill them, the more as the Fanatics left near five; the unarmed drift towards the belief while drawing
    events away from known counters; everyone rests at the psychoanalyst before their SAN gets too low to face
    Nyog'Sothep; tables are chosen by the expected worth of their rows, questions by the bits they would teach
    about his position. Once Nyog'Sothep is out (`banishers`), the spell holders with a sign gather at the
    city where three of them meet at the least cost (a tired one's steps cost more), resting first when the
    Foul One would strike them all the way, the others keep away from it, everyone avoids being shut in by
    ruins, and after a partial banishment it is sent back close to fit banishers, far away otherwise. Paths
    (`paths`) avoid destroyed cities. Its weights are an `InvestigatorWeights`
    (`InvestigatorsAi::with_weights`). `tests/simulation.rs` plays 150 games of `PetersenAi` against it,
    checking every answer of the professor's AI and that almost every game ends, and checks that playing
    either side from outside the engine plays the same games, and that the replay follows each game to its
    final board. `examples/tune_ai.rs` hill-climbs both sides' weights by self-play on every core, keeping a
    change only when it wins more by three standard errors.
- **`src/db/`**: a document database, modelled on the game as one aggregate. `games` holds one `GameDocument`
  per game: `_id` (numbered by the `sequences` collection), `token`, `version` (1 when created, bumped by every
  `save`), the whole `GameState` embedded as it serializes (secrets included) and the tokens' `tilts`
  (`TiltDocument`s). The log and the replay grow apart, in batches (bucket pattern): every save appends at most
  one `LogBatch` (`log_batches`) and one `ReplayBatch` (`replay_batches`) tagged with the game id and the version,
  indexed on `game_id`, so a save never rewrites the history and the game page never reads the replay's boards.
  `Store` opens the database folder (created if missing, indexes created at open) and wraps it: `write` runs a
  closure in a transaction, alone among writers (PoloDB transactions do not lock what they read), committing it
  on success and rolling it back on error; `run` moves blocking work onto `spawn_blocking`. Reads take a
  `DocumentSource` (the `Database`, or a `Transaction` that sees its own writes). No migrations: a field added
  to a stored type must default when missing (`#[serde(default)]`) for the games already stored.
- **`src/service/`**: one use case per file. `create_game` takes the human's `Side`, draws four investigators and
  a private token, runs the game until the human's first prompt (the first moves, or the professor's setup) and
  stores everything. `answer_prompt` loads the game inside `Store::write`, runs the engine with real dice,
  `PetersenAi` and `InvestigatorsAi`, saves the state and the tilts and appends the log and the replay steps; a
  refused answer writes nothing. `GameView` is the human's view of a game. Leading the investigators, it never holds the professor's
  position (only the last sighting), hand, hidden counters, route sheet or Nyog'Sothep's city while they are
  secret; playing the professor, it holds all of them (`CounterView::hidden` marks the counters the
  investigators have not found). At the end it reveals the route sheet, every counter and the scores.
  `TokenTilts` gives every token a random tilt of up to 5° either way (stored in the game document), drawn again
  by `refresh_tilts` before each save only for the tokens that moved. `ReplayView` holds the replay steps of a
  finished game (`AppError::GameNotOver` before the end), from the first one with a board.
- **`src/web/`**: `router.rs` lists the routes (`/`, `POST /games` with the `side` field, `/rules` (the scanned French
  rules, `img/rules-*.jpg`), `/games/{id}/{token}`, `/games/{id}/{token}/replay` (`replay_handlers.rs`)
  and one `POST` per kind of answer: the investigators' `moves`, `table`, `cities`, `play-again`, `fight`,
  `banishment` in `answer_handlers.rs`, the professor's `setup`, `teleport`, `move`, `attack`, `location`
  (carry, retreat, Nyog'Sothep's step), `summon`, `place`, `counter` (Table 1, discard), `name` in
  `professor_handlers.rs`). Handlers read the form as name/value pairs (`form_fields.rs`), call
  `answer_prompt`, then redirect (303) to the game page, or show it again with the French error (400).
  Every link, form action and redirect is relative, the site being served under a sub-path: pages get a
  `RelativeRoot` (`relative_root.rs`, an extractor: `./`, `../../`…) as `root` and write `{{ root }}static/…`,
  `{{ root }}games/…`; `AppError` responses are turned into the error page by the `render_error_pages` middleware,
  which knows the request's path, and `replay.js` finds the images next to its own URL.
  `GamePage` (`templates/game.html`) draws the full-window map (`BoardView`: every location as a click target, tokens stacked side by side on their location, investigators at sea on a boat, the professor's last
  sighting, or his position and hidden counters for the professor) with floating panels: on the left the banner
  (professor's hit points and position; for the professor, also his hand and summoning progress; the
  investigators with SAN, items, spell and lasting effects) and the screen of the pending prompt (`Screen::of`,
  `templates/screens/*.html`, the professor's in `professor_*.html` with their choices in
  `professor_screens.rs`), with below it the « Nouvelle partie » and « Règles » buttons, on the right the notebook, the professor's route sheet and the log. Moves are made in
  selects (`move_{index}` for the investigators, `destination` for the professor), which `board.js` also fills
  when the player clicks a token and then a highlighted destination (it draws an arrow per move). Locations are
  picked by ticking inputs drawn on the map (`name="city"`, joined to `#screen-form` with the `form` attribute),
  only on the locations the prompt allows. `board.js` scales the board to the width left between the
  panels and remembers which panels are open. `ReplayPage` (`templates/replay.html`, linked from the end screen)
  draws the bare map (`BoardView::bare`) and embeds the steps as JSON with the French names; `replay.js` plays
  them every half second (play/pause, step, scrub, journal), laying out the tokens as `BoardView` does and
  sliding them to their new location.

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
