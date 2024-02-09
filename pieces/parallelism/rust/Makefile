format:
	cargo fmt --all --quiet

check:
	cargo check --all-targets --quiet

lint:
	cargo clippy --all-targets --quiet

test:
	cargo test --quiet

run:
	cargo run --quiet

all: format check lint test run
