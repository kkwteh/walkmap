CREATE TABLE maps (
  id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT REFERENCES users(id),
  created_at timestamp NOT NULL DEFAULT (now() at time zone 'utc')
);