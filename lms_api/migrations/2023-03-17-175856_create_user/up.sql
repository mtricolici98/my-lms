-- Your SQL goes here
CREATE TABLE users (
  id uuid PRIMARY KEY,
  username VARCHAR(25) NOT NULL,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)