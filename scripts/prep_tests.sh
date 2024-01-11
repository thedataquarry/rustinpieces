#!bin/bash

cp pieces/postgres_etl/data/persons.csv scripts/data
docker compose -f scripts/docker-compose.yml up
