SELECT 'CREATE DATABASE etl'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'etl')\gexec