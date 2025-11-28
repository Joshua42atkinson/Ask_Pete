CREATE TABLE IF NOT EXISTS scenarios (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    difficulty TEXT NOT NULL, -- 'Easy', 'Medium', 'Hard'
    status TEXT NOT NULL DEFAULT 'Available', -- 'Locked', 'Available', 'Completed'
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed data
INSERT INTO scenarios (title, description, difficulty, status, tags) VALUES
('The Cognitive Load Express', 'Learn the basics of cognitive load theory while managing a busy train station.', 'Easy', 'Available', ARRAY['Tutorial', 'Basics']),
('The Dissonance Derailment', 'Identify and fix projective dissonance in a complex learning scenario.', 'Medium', 'Locked', ARRAY['Advanced', 'Troubleshooting']),
('The Steam Engine of Mastery', 'Master the art of generating steam through Socratic dialogue.', 'Hard', 'Locked', ARRAY['Mastery', 'Socratic']);
