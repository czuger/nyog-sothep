# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A digital implementation of *Nyog'Sothep ou les brumes de l'Immonde*, a 1988 board game from *Jeux & Stratégie*.
The full game rules (translated from the original French) are in `RULES.md`; scans of the original magazine pages
and of the board are in `work/*.jpg`.

The app is currently a Rails 5.2 / Ruby 2.6.3 / PostgreSQL application. The current branch, `rust_conversion`, is
meant to port it to Rust; no Rust code exists yet.

Game model: **the human plays Professor Petersen; all six Investigators are driven by an AI.** UI text and
translations are in French (`config/locales/fr/`).

## Commands

```sh
bundle install
bin/rails db:create db:migrate      # needs a local "nyog_sothep" Postgres role, see postgresql.txt
bin/rails server                    # root page is the games list (g_game_boards#index)

bin/rails test                                       # all tests (SimpleCov coverage is on)
bin/rails test test/models/encounters_goules_test.rb # one file
bin/rails test test/models/encounters_goules_test.rb:15   # one test, by line number

bundle exec rubocop
bin/rails print_investigator_graph  # regenerate graphs/*.gviz from the AASM state machines (needs graphviz)
```

Tests use FactoryGirl factories (`test/factories/`) and Mocha stubs. Randomness is made deterministic by stubbing
`Kernel.rand` (which `GameCore::Dices` uses), or by stubbing the roll helpers directly, for example
`IInvestigator.any_instance.stubs(:event_dices)` / `:choose_table` or `@gb.stubs(:dice)`.

## Architecture

All game logic lives in `app/models/game_core/`, split into modules that get mixed into three ActiveRecord models:

- `GGameBoard` (game state): includes `Encounters`, `GGameBoardInvestigatorsActions` and `NyogSothep`.
- `IInvestigator`: includes `Events` (which pulls in `EventGroundA` and `EventGroundB`), `Ia::Investigator`,
  `IInvestigatorFight` and `IInvestigatorTurn`.
- `PProfessor`: includes `ProfessorActions` and `ProfessorFight`.

**Turn flow is driven by two AASM state machines** (diagrams in `graphs/board.png` and `graphs/inv.png`):

- Board: `prof_move → inv_move → inv_events → prof_move`, plus `prof_asked_for_fake_cities`, `game_won` and
  `game_lost`.
- Investigator: `move → events → turn_finished`, plus waiting states for multi-turn effects (`at_a_psy`,
  `going_to_great_psy`/`at_a_great_psy`, `to_misty_things`/`at_misty_things`) and `dead`.

A full round runs synchronously within one HTTP request:

1. `ProfessorActionsController#move` / `#dont_move` calls `PProfessor#prof_play`. That moves the Professor (and
   Nyog'Sothep if they are linked), checks for fights, and draws a monster.
2. It then calls `GGameBoard#investigators_ia_play`, which runs the movement AI and then each investigator's
   `ia_actions` (encounters, then events).
3. Finally `finalize_turn` runs.

**Pausing for player input.** Some events need the human player to name cities (for example "name two cities, one
of which is your position"). These events switch the board to `prof_asked_for_fake_cities` and stop the
investigator loop (`:break`). `ProfFakePosController#create` records the answer and resumes
`investigators_ia_play` from the `inv_events` state.

**The map is static YAML, not stored in the database.**

- `GameCore::Map::Location` (with the `City` and `WaterArea` subclasses) loads data from
  `app/models/game_core/map/data/`: `locations.yml` (coordinates, port flag), `destinations.yml` (adjacency
  between cities and sea zones) and `borders_crossings.yml`.
- Positions are stored everywhere as `*_code_name` strings or symbols (`:nantucket`, `:oxford`…).
- The `process_data` rake tasks regenerate the YAML files from `work/*.txt` and `work/destinations.json`.
- The `load_data` rake tasks and the city, road and water-area migrations date from an older database-backed map
  (`CCity`, `WWaterArea`) and no longer work.

**Investigator AI** (`app/models/game_core/ia/`):

- `ProfPositionFinder` keeps what the investigators know about the Professor's position: spotted positions and the
  "fake city" answers, for the last 3 turns. It is serialized to `IaProfPosition#gb_data`.
- `InvestigatorMovementTarget` makes investigators chase the Professor if they have a weapon and flee otherwise.
  It uses `BfsAlgo` for distances on the map graph.

**Other models**

- `MMonster` is the Professor's draw pile, `PMonster` is his hand, and `PMonsterPosition` is a monster placed in a
  city.
- `GDestroyedCity` stores the cities Nyog'Sothep has destroyed.
- `LLog` is the game log: entries hold i18n keys plus data and are translated at display time.
- Events are dispatched by name: `send("table#{n}_e#{roll}")`. Events that are not implemented fall through to
  `method_missing` and are only logged.

## Divergences from RULES.md

These matter for the Rust port; decide whether to keep or fix each one.

- The code's event tables `table1_*` and `table2_*` correspond to the rules' **Table 2** and **Table 3**.
- `MONSTERS_ROLL_MAP` (`professor_actions.rb`) simplifies Table 1. It has no "his choice" options, so Flying
  Horror, Teleportation and Storm are never drawn.
- Starting SAN is `3d6`; the rules say `3d6 + 3`. The Professor draws 5 starting monsters; the rules say 4.
