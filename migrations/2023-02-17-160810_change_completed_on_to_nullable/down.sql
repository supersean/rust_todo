-- This file should undo anything in `up.sql`
ALTER TABLE todos ALTER COLUMN completed_on SET NOT NULL
