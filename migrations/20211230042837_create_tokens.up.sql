-- Add up migration script here
CREATE TABLE tokens (
  id UUID PRIMARY KEY,
  value TEXT NOT NULL UNIQUE,
  user_id UUID NOT NULL REFERENCES users(id)
);