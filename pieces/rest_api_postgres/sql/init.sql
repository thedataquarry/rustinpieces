SELECT 'CREATE DATABASE api'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'api')\gexec

\connect api

CREATE TABLE IF NOT EXISTS users(
  id serial PRIMARY KEY,
  user_name text UNIQUE NOT NULL,
  hashed_password text NOT NULL,
  is_admin boolean NOT NULL DEFAULT false
)\gexec
