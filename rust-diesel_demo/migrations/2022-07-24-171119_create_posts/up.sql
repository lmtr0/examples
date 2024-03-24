-- Your SQL goes here
CREATE TABLE posts (
  title VARCHAR NOT NULL PRIMARY KEY,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);