#!bin/bash

mkdir scripts/data
cp pieces/postgres_etl/data/persons.csv scripts/data
docker compose -f scripts/docker-compose.yml up -d
