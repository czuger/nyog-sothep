-- The slight rotation each token is drawn with; it is drawn again only when the token moves.
CREATE TABLE token_tilts (
    game_id INTEGER NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    token TEXT NOT NULL,
    location TEXT NOT NULL,
    tenths INTEGER NOT NULL,
    PRIMARY KEY (game_id, token)
) STRICT;
