#!/bin/bash

# Build the rust binary in release mode
cd rust
cargo build -r
cd ..

rust_start_time=$(date +%s%N)  # Get the start time in nanoseconds

# Because this is in a workspace the binary gets built in the ../../target/release/ directory
../../target/release/meilisearch-cli search "red"
../../target/release/meilisearch-cli search "Desiccated blackberry, leather, charred wood and mint aromas" -l 5
../../target/release/meilisearch-cli search "Tempranillo Blend" -s "country:asc"
../../target/release/meilisearch-cli search "Pinot Noir"
../../target/release/meilisearch-cli search "Zumaya"
../../target/release/meilisearch-cli search "Brut Sparkling" -l 10 -s "title:asc" -s "country:desc"
../../target/release/meilisearch-cli search "Catalonia" -l 50
../../target/release/meilisearch-cli search "On the palate, it's malic and citric, with lime and white grapefruit flavors" -s "country:asc"
../../target/release/meilisearch-cli search "Cabernet Sauvignon"
../../target/release/meilisearch-cli search "Chianti Classico" -s "title:desc"

rust_end_time=$(date +%s%N)  # Get the end time in nanoseconds
rust_duration=$(echo "scale=3; ($rust_end_time - $rust_start_time) / 1000000000" | bc)

cd python
if [ ! -d "venv" ]; then
  python -m venv venv
fi
. venv/bin/activate
python -m pip install -r requirements.txt
python_start_time=$(date +%s%N)  # Get the start time in nanoseconds

meilisearch-cli search "red"
meilisearch-cli search "Desiccated blackberry, leather, charred wood and mint aromas" -l 5
meilisearch-cli search "Tempranillo Blend" -s "country:asc"
meilisearch-cli search "Pinot Noir"
meilisearch-cli search "Zumaya"
meilisearch-cli search "Brut Sparkling" -l 10 -s "title:asc" -s "country:desc"
meilisearch-cli search "Catalonia" -l 50
meilisearch-cli search "On the palate, it's malic and citric, with lime and white grapefruit flavors" -s "country:asc"
meilisearch-cli search "Cabernet Sauvignon"
meilisearch-cli search "Chianti Classico" -s "title:desc"

python_end_time=$(date +%s%N)  # Get the end time in nanoseconds
python_duration=$(echo "scale=3; ($python_end_time - $python_start_time) / 1000000000" | bc)
cd ..

echo "Rust execution time: $rust_duration seconds"
echo "Python execution time: $python_duration seconds"
