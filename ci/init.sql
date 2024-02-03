SELECT 'CREATE DATABASE testing'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'testing')\gexec

\connect testing

CREATE TABLE IF NOT EXISTS persons(
    id integer PRIMARY KEY,
    name text,
    age smallint,
    isMarried boolean,
    city text,
    state text,
    country text
)\gexec

TRUNCATE TABLE persons\gexec

COPY persons(id, name, age, isMarried, city, state, country)
FROM '/data/persons.csv'
DELIMITER ','
CSV HEADER\gexec

CREATE TYPE bookstatus AS ENUM('read', 'currently_reading', 'want_to_read')\gexec

CREATE TABLE IF NOT EXISTS books(
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  author_first_name TEXT NOT NULL,
  author_last_name TEXT NOT NULL,
  book_status bookstatus NOT NULL,
  date_added TIMESTAMP with time zone NOT NULL,
  date_read TIMESTAMP with time zone,
  rating SMALLINT,
  UNIQUE (title, author_first_name, author_last_name)
)\gexec

TRUNCATE TABLE books\gexec
