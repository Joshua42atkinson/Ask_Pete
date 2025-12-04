CREATE TABLE IF NOT EXISTS vocabulary_words (
    word TEXT PRIMARY KEY,
    definition TEXT NOT NULL,
    grade_level INTEGER NOT NULL,
    tier INTEGER NOT NULL,
    weight INTEGER NOT NULL,
    tags TEXT [] NOT NULL
);