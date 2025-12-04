-- Add migration script here
CREATE TABLE IF NOT EXISTS knowledge_sources (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    source_type TEXT NOT NULL,
    -- 'pdf', 'txt', 'md', 'url'
    uploaded_by UUID,
    -- Optional link to user who uploaded it
    created_at TIMESTAMPTZ DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'::jsonb
);
CREATE TABLE IF NOT EXISTS telemetry_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp DOUBLE PRECISION NOT NULL,
    event_type TEXT NOT NULL,
    data TEXT NOT NULL,
    -- JSON string of the event data
    user_id TEXT,
    -- Optional user ID for tracking
    session_id TEXT,
    -- Optional session ID
    created_at TIMESTAMPTZ DEFAULT NOW()
);
-- Index for faster querying of logs
CREATE INDEX idx_telemetry_logs_event_type ON telemetry_logs(event_type);
CREATE INDEX idx_telemetry_logs_created_at ON telemetry_logs(created_at);