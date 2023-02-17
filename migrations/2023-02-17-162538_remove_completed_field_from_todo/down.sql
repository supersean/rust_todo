-- This file should undo anything in `up.sql`
ALTER TABLE todos ADD COLUMN completed boolean NOT NULL DEFAULT false
