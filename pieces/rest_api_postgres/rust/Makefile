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

dev:
	cargo watch -w src -c -x run


add-book:
	@curl -X POST http://127.0.0.1:3000/book -H 'Content-Type: application/json' --data-binary '$(BOOK)'

delete-book:
	@curl -X DELETE http://127.0.0.1:3000/book/$(BOOK_ID)

get-books:
	@curl http://127.0.0.1:3000/book

get-book:
	@curl http://127.0.0.1:3000/book/$(BOOK_ID)

update-book:
	@curl -X PUT http://127.0.0.1:3000/book -H 'Content-Type: application/json' --data-binary '$(BOOK)'

all: format check lint test run
