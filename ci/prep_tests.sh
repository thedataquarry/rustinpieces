#!bin/bash

mkdir ci/data
cp pieces/postgres_etl/data/persons.csv ci/data
docker compose -f ci/docker-compose.yml up -d
