# Nyog'Sothep ou les brumes de l'Immonde

A web version of *Nyog'Sothep ou les brumes de l'Immonde*, a board game published in *Jeux & Stratégie* (1988).
Choose your side when you start a game: lead the four Investigators against the computer's Professor
Petersen, hidden behind his screen, or play the professor against four investigators led by the computer. See
`RULES.md` for the translated rules and the readings this implementation chose.

Ticking « Version fan-made » when starting a game plays the variant of `fan_mode_proposition.md` instead: three
d20 event tables, new objects, draws decided by the professor's place, Fanatics arriving on their own, a
banishment spell cast alone that grows stronger with each failure, and more.

## Run

```sh
cargo run
```

Then open <http://127.0.0.1:3000> and start a game. Bookmark the page you land on: its URL is your private
link to the game.

Settings (environment variables):

| Variable | Default | Meaning |
|---|---|---|
| `DATABASE_PATH` | `data/nyog_sothep.polodb` | Folder of the PoloDB database, created if missing |
| `NYOG_SOTHEP_DATA_DIR` | | Folder holding `nyog_sothep.polodb`, used when `DATABASE_PATH` is not set |
| `BIND_ADDR` | `127.0.0.1:3000` | TCP address the server listens on |
| `NYOG_SOTHEP_SOCKET` | | Unix socket to listen on instead of TCP |
| `RUST_LOG` | `info` | Log filter |

`img/` and `static/` are compiled into the binary (read from disk in debug builds). Links are relative, so the
site works under any sub-path.

## Deployment

Deployed with `cargo deploy`, my deployment tool: it cross-compiles the binary, uploads it over SSH and runs it as
a systemd user service listening on `NYOG_SOTHEP_SOCKET`, served by the existing nginx under `/nyog-sothep/`. The
server settings come from the `DEPLOY_*` environment variables; how the app is started (no arguments, the
database in `data/db/`) is in `[package.metadata.deploy]` of `Cargo.toml`.

```sh
cargo deploy setup      # once: server layout, prerequisites, nginx snippet
cargo deploy            # build, upload, restart, health check on GET /
cargo deploy rollback   # back to the previous release
cargo deploy status     # versions, service state, socket and public URL
cargo deploy logs       # follow the journal
```

## Tests

```sh
cargo test
cargo test --test simulation -- --nocapture   # the professor's AI against the investigators' AI, with statistics for both rule sets
```
