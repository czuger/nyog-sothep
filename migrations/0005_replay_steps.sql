-- Every event of a game for its replay, secrets included: the dice rolled for it (JSON list) and
-- the whole board after it (JSON), NULL when the event left the board unchanged.
CREATE TABLE replay_steps (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    turn INTEGER NOT NULL,
    text TEXT,
    dice TEXT NOT NULL,
    board TEXT
) STRICT;

CREATE INDEX replay_steps_by_game ON replay_steps (game_id, id);
