-- Create Campaigns Table
CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    style TEXT NOT NULL DEFAULT 'IronHorse',
    -- 'IronHorse', 'ScenicRoute', 'NightTrain'
    current_station_id TEXT NOT NULL DEFAULT 'STATION_01_START',
    collective_coal INTEGER NOT NULL DEFAULT 100,
    collective_steam INTEGER NOT NULL DEFAULT 0,
    active_vote JSONB,
    -- Stores current vote state
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
-- Create Campaign Members Table (Join Table)
CREATE TABLE IF NOT EXISTS campaign_members (
    campaign_id UUID REFERENCES campaigns(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'Engineer',
    -- 'Engineer', 'Conductor', 'Stoker', 'Signalman'
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (campaign_id, user_id)
);