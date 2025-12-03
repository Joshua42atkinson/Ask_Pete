-- Create Characters Table
CREATE TABLE IF NOT EXISTS characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    -- 'Engineer', 'Conductor', 'Stoker', 'Signalman'
    archetype TEXT NOT NULL,
    -- 'The Builder', 'The Explorer', etc.
    level INTEGER NOT NULL DEFAULT 1,
    stats JSONB DEFAULT '{}'::jsonb,
    -- Flexible stats storage
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
-- Index for faster lookups by user
CREATE INDEX idx_characters_user_id ON characters(user_id);