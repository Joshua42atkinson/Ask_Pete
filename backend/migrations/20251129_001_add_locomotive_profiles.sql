-- Migration: Add Locomotive Profile columns to Archetypes and Seed Data
-- Date: 2025-11-28

-- 1. Add Locomotive Stats Columns
ALTER TABLE archetypes 
ADD COLUMN IF NOT EXISTS fuel_efficiency FLOAT DEFAULT 1.0,
ADD COLUMN IF NOT EXISTS cargo_capacity FLOAT DEFAULT 10.0,
ADD COLUMN IF NOT EXISTS durability FLOAT DEFAULT 1.0,
ADD COLUMN IF NOT EXISTS locomotive_type VARCHAR(255) DEFAULT 'Standard';

-- 2. Clear existing archetypes to ensure clean seed (optional, but safer for dev)
DELETE FROM archetypes;

-- 3. Seed the 12 Jungian Archetypes (Locomotive Profiles)

-- Group 1: The Ego Types (Preparation)
INSERT INTO archetypes (name, description, locomotive_type, fuel_efficiency, cargo_capacity, durability) VALUES
('The Innocent', 'Desires safety and happiness. Fears punishment.', 'Light Commuter Rail', 1.5, 5.0, 0.8),
('The Orphan', 'Desires belonging. Fears being left out.', 'Utility Shunter', 1.2, 8.0, 1.0),
('The Hero', 'Desires to prove worth. Fears weakness.', 'Interceptor Express', 0.8, 6.0, 0.9), -- High speed (low efficiency), brittle
('The Caregiver', 'Desires to protect. Fears selfishness.', 'Armored Supply Train', 1.0, 12.0, 1.5);

-- Group 2: The Soul Types (Journey)
INSERT INTO archetypes (name, description, locomotive_type, fuel_efficiency, cargo_capacity, durability) VALUES
('The Explorer', 'Desires freedom. Fears entrapment.', 'All-Terrain Switcher', 1.3, 7.0, 1.2),
('The Rebel', 'Desires revolution. Fears powerlessness.', 'Ice-Breaker Engine', 0.9, 8.0, 1.4),
('The Lover', 'Desires intimacy. Fears aloneness.', 'Scenic Cruiser', 1.1, 6.0, 1.0),
('The Creator', 'Desires to create value. Fears mediocrity.', 'Maintenance-of-Way Vehicle', 1.0, 10.0, 1.1);

-- Group 3: The Self Types (Integration)
INSERT INTO archetypes (name, description, locomotive_type, fuel_efficiency, cargo_capacity, durability) VALUES
('The Jester', 'Desires joy. Fears boredom.', 'Steam Whistle', 1.4, 5.0, 1.0),
('The Sage', 'Desires truth. Fears ignorance.', 'Diesel-Electric Heavy Hauler', 0.7, 20.0, 1.5), -- Massive capacity, low efficiency (slow)
('The Magician', 'Desires knowledge of laws. Fears unintended consequences.', 'Maglev Prototype', 1.8, 4.0, 0.7), -- High efficiency, low durability
('The Ruler', 'Desires control. Fears chaos.', 'Station Master', 1.0, 15.0, 1.3);
