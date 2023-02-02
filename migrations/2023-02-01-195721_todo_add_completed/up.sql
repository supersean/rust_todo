-- Your SQL goes here
ALTER TABLE todos ADD COLUMN completed BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE todos ADD COLUMN completed_on timestamp NOT NULL DEFAULT now();

