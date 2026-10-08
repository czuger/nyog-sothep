# Nyog'Sothep ou les brumes de l'Immonde

A web version of *Nyog'Sothep ou les brumes de l'Immonde*, a board game published in *Jeux & Stratégie* (1988).
You lead the four Investigators; the computer plays Professor Petersen, hidden behind his screen. See
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
cargo test --test simulation -- --nocapture   # the professor's AI against a simple bot, with statistics
```
