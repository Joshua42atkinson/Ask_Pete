-- Create Users Table (if not exists)
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'student',
    steam_balance DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    coal_balance DOUBLE PRECISION NOT NULL DEFAULT 100.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Create Story Graphs Table (if not exists)
CREATE TABLE IF NOT EXISTS story_graphs (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    author_id BIGINT REFERENCES users(id),
    graph_data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Create Quest Completions Table
CREATE TABLE IF NOT EXISTS quest_completions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    quest_id INTEGER NOT NULL REFERENCES story_graphs(id),
    steam_earned DOUBLE PRECISION NOT NULL,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);