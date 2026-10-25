-- The side the human plays; the computer plays the other.
ALTER TABLE games ADD COLUMN human_side TEXT NOT NULL DEFAULT 'investigators'
    CHECK (human_side IN ('investigators', 'professor'));
