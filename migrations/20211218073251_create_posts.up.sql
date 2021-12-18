-- Add up migration script here
CREATE TABLE posts (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  user_id UUID NOT NULL REFERENCES users(id)
);