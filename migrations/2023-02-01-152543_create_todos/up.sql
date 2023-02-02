-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  text VARCHAR NOT NULL
  /* date_created TIMESTAMP NOT NULL DEFAULT now() */
)
