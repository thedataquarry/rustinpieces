format:
	cargo fmt --all --quiet

check:
	cargo check --all-targets --quiet

lint:
	cargo clippy --all-targets --quiet

develop:
	uv run maturin develop

release:
	uv run maturin develop -r

test: develop
	uv run pytest

all: format check lint test
