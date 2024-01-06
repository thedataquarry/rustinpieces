SELECT 'CREATE DATABASE etl'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'etl')\gexec

\connect etl

CREATE TABLE IF NOT EXISTS persons(
    id integer PRIMARY KEY,
    name text,
    age smallint,
    isMarried boolean,
    city text,
    state text,
    country text
)\gexec
