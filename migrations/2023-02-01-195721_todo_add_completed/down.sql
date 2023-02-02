-- This file should undo anything in `up.sql`
ALTER TABLE todos DROP COLUMN completed;
ALTER TABLE todos DROP COLUMN completed_on;

