-- One row per game: the turn engine's cursor and the public flags, plus Nyog'Sothep's secret city.
CREATE TABLE games (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    version INTEGER NOT NULL DEFAULT 0,
    turn INTEGER NOT NULL,
    phase TEXT NOT NULL,
    prompt TEXT,
    outcome TEXT,
    nyog_city TEXT NOT NULL,
    nyog_location TEXT,
    nyog_joined INTEGER NOT NULL,
    nyog_target TEXT,
    sea_announcements INTEGER NOT NULL,
    move_in_the_open INTEGER NOT NULL
) STRICT;

CREATE TABLE professor (
    game_id INTEGER PRIMARY KEY REFERENCES games (id) ON DELETE CASCADE,
    hp INTEGER NOT NULL,
    location TEXT NOT NULL
) STRICT;

CREATE TABLE investigators (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    seat INTEGER NOT NULL,
    code TEXT NOT NULL,
    san INTEGER NOT NULL,
    weapon INTEGER NOT NULL,
    sign INTEGER NOT NULL,
    medallion INTEGER NOT NULL,
    spell INTEGER NOT NULL,
    location TEXT NOT NULL,
    previous TEXT NOT NULL,
    status TEXT NOT NULL,
    skip_turns INTEGER NOT NULL,
    skips_action INTEGER NOT NULL,
    car_broken INTEGER NOT NULL,
    forbidden_city TEXT,
    free_border INTEGER NOT NULL,
    blocked_crossing TEXT,
    seen_nyog INTEGER NOT NULL,
    fanatics_killed INTEGER NOT NULL,
    damage_dealt INTEGER NOT NULL,
    banished INTEGER NOT NULL,
    PRIMARY KEY (game_id, seat)
) STRICT;

-- Every counter of the box: in the reserve, in the professor's hand, placed in a city or removed.
CREATE TABLE counters (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    kind TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('reserve', 'hand', 'placed', 'removed')),
    city TEXT,
    discovered INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (game_id, number)
) STRICT;

-- The stacking rule: at most one placed counter per city.
CREATE UNIQUE INDEX counters_one_per_city ON counters (game_id, city) WHERE state = 'placed';

-- The professor's stage notes, hidden from the human until the end of the game.
CREATE TABLE route_sheet (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    turn INTEGER NOT NULL,
    location TEXT NOT NULL,
    counter TEXT,
    PRIMARY KEY (game_id, position)
) STRICT;

CREATE TABLE destroyed_cities (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    city TEXT NOT NULL,
    PRIMARY KEY (game_id, position)
) STRICT;

-- The investigators' notebook; `kind` is the clue as JSON.
CREATE TABLE clues (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    turn INTEGER NOT NULL,
    kind TEXT NOT NULL,
    PRIMARY KEY (game_id, position)
) STRICT;

CREATE TABLE event_log (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    turn INTEGER NOT NULL,
    text TEXT NOT NULL
) STRICT;

CREATE INDEX event_log_by_game ON event_log (game_id, id);
