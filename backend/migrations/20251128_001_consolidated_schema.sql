-- 1. Users Table (Updated for Google Auth)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    google_id VARCHAR(255) UNIQUE, -- [NEW] For OAuth
    password_hash VARCHAR(255), -- Nullable for OAuth users
    full_name VARCHAR(255),
    avatar_url TEXT, -- [NEW] From Google Profile
    creator_level INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. Projects & Nodes (Core Graph)
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    risk_tier INTEGER NOT NULL DEFAULT 0,
    learning_objectives TEXT,
    starting_node_id INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    position_x INTEGER NOT NULL DEFAULT 0,
    position_y INTEGER NOT NULL DEFAULT 0,
    tags TEXT
);

CREATE TABLE choices (
    id SERIAL PRIMARY KEY,
    source_node_id INTEGER NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    target_node_id INTEGER NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    choice_text VARCHAR(255) NOT NULL
);

-- 3. Persona Engine (Archetypes)
CREATE TABLE archetypes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE stats (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE archetype_stat_buffs (
    archetype_id INTEGER NOT NULL REFERENCES archetypes(id),
    stat_id INTEGER NOT NULL REFERENCES stats(id),
    buff_value INTEGER NOT NULL,
    PRIMARY KEY (archetype_id, stat_id)
);

-- Link Users to Archetypes
ALTER TABLE users ADD COLUMN primary_archetype_id INTEGER REFERENCES archetypes(id);
ALTER TABLE choices ADD COLUMN required_archetype_id INTEGER REFERENCES archetypes(id);

-- 4. Vocabulary & VaaM
CREATE TABLE vocabulary_words (
    word VARCHAR(255) PRIMARY KEY,
    definition TEXT NOT NULL,
    grade_level INTEGER,
    tier INTEGER,
    weight INTEGER,
    tags TEXT[]
);

CREATE TABLE word_usage_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    word VARCHAR(255) REFERENCES vocabulary_words(word),
    context_used TEXT,
    used_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5. Scenarios (Train Yard)
CREATE TABLE scenarios (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    difficulty TEXT NOT NULL, -- 'Easy', 'Medium', 'Hard'
    status TEXT NOT NULL DEFAULT 'Available', -- 'Locked', 'Available', 'Completed'
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed Data: Scenarios
INSERT INTO scenarios (title, description, difficulty, status, tags) VALUES
('The Cognitive Load Express', 'Learn the basics of cognitive load theory while managing a busy train station.', 'Easy', 'Available', ARRAY['Tutorial', 'Basics']),
('The Dissonance Derailment', 'Identify and fix projective dissonance in a complex learning scenario.', 'Medium', 'Locked', ARRAY['Advanced', 'Troubleshooting']),
('The Steam Engine of Mastery', 'Master the art of generating steam through Socratic dialogue.', 'Hard', 'Locked', ARRAY['Mastery', 'Socratic']);

-- Seed Data: Archetypes (Jungian Locomotive Profiles)
INSERT INTO archetypes (name, description) VALUES
('The Innocent', 'The Light Commuter Rail. High efficiency, low friction tolerance.'),
('The Hero', 'The Interceptor Express. High velocity, high combustion.'),
('The Sage', 'The Heavy Hauler. High cargo capacity, slow acceleration.');
