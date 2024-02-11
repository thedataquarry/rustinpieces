format:
	cargo fmt --all --quiet

check:
	cargo check --all-targets --quiet

lint:
	cargo clippy --all-targets --quiet

develop:
	maturin develop

release:
	maturin develop -r

test: develop
	pytest

all: format check lint test
