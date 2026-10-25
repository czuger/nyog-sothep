# Nyog'Sothep ou les brumes de l'Immonde

A web version of *Nyog'Sothep ou les brumes de l'Immonde*, a board game published in *Jeux & Stratégie* (1988).
Choose your side when you start a game: lead the four Investigators against the computer's Professor
Petersen, hidden behind his screen, or play the professor against four investigators led by the computer. See
`RULES.md` for the translated rules and the readings this implementation chose.

## Run

```sh
cargo run
```

Then open <http://127.0.0.1:3000> and start a game. Bookmark the page you land on: its URL is your private
link to the game.

Settings (environment variables):

| Variable | Default | Meaning |
|---|---|---|
| `DATABASE_URL` | `sqlite://data/nyog_sothep.db` | SQLite database, created with its folder if missing |
| `BIND_ADDR` | `127.0.0.1:3000` | Address the server listens on |
| `ASSET_ROOT` | `.` | Folder holding `img/` and `static/` |
| `RUST_LOG` | `info` | Log filter |

## Tests

```sh
cargo test
cargo test --test simulation -- --nocapture   # the professor's AI against the investigators' AI, with statistics
```
