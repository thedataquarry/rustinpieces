# REST API with PostgreSQL

A REST API that interacts with PostgreSQL.

## Goal

In this piece, we will use `fastapi` + `asyncpg` packages in Python and the `axum` + `sqlx` crates
in Rust to build a REST API.

## Python Setup

Install the dependencies in a virtual environment via `requirements.txt`.

```sh
# First time setup
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# for subsequent runs, simply activate the environment
source venv/bin/activate
```

# Start the database

From the `pieces/rest_api_postgres` directory start the database with docker compose

```sh
docker compose up
```

# Start the development server

The server can be started either with `Make` or manually.

```sh
# With Make
make dev

# Manually
uvicorn app.main:app --reload
```

FastAPI automatically generates interactive OpenAPI documentation. We can use this to test/interact
with our API. To view the documentation go to `http://127.0.0.1/docs`, then from here expeand the
section you want to try and clieck the `Try it out` button.

## Rust setup

In this example we use [cargo-watch](https://github.com/watchexec/cargo-watch) to automatically
restart the server any time code is changed. This prevents us from having to stop and start the
server any time a code change is made. If you do not already have `cargo-watch` installed you can
install it by running `cargo install cargo-watch`.

Install dependencies via Cargo. In this piece, we use the `anyhow`, `axum`, `chrono`, `dotenvy`,
`serde`, `sqlx`, and `tokio` crates. Addtinally we use `http-body-util`, `mime`, `serde_json`,
`tower`, and `uuid` for testing.

```sh
cargo add anyhow
cargo add axum
cargo add chrono --features chrono
cargo add dotenvy
cargo add serde --features derive
cargo add sqlx --features postgres --features runtime-tokio --features chrono
cargo add tokio --features full
cargo add --dev http-body-util
cargo add --dev mime
cargo add --dev serde-json
cargo add --dev tower
cargo add --dev uuid --features v4
```

# Start the database

From the `pieces/rest_api_postgres` directory start the database with docker compose

```sh
docker compose up
```

## Start the development server

The server can be started either with `Make` or manually.

```sh
# With Make
make dev

# Manually
cargo watch -w src -c -x run
```

Make commands are provided to interact with the API

```sh
# Add a book
make add-book BOOK='{"title": "Some Title", "authorFirstName": "FirstName", "authorLastName": "LastName", "bookStatus": "Read", "dateAdded": "2024-01-30T15:47:01Z", "dateRead": null, "rating": 1}'

# Delete a book
make delete-book BOOK_ID=1

# Get a book
make get-book BOOK_ID=1

# Get all books
make get-books

# Update a book
make update-book BOOK='{"id": 2, "title": "New Title", "authorFirstName": "FirstName", "authorLastName": "LastName", "bookStatus": "Read", "dateAdded": "2024-01-30T15:47:01Z", "dateRead": null, "rating": 1}'
```

# Takeaways

With FastAPI we are using `uvicorn` the `--reload` flag to auto-restart the server when changes are
made. In Rust we get the equivalent functionality by using `cargo-watch`. While in this example we are
using `cargo-watch` to restart the webserver, `cargo-watch` is not specific to web servers and can
be used for any Rust program that you want to re-run when changes are saved.

One nice feature of FastAPI is OpenAPI documenation is automatically generated for routes. This
makes it very easy to test and document the API. Because Axum does not provide this same functionality
a program such as [postman](https://www.postman.com/) with test routes manually created is needed to
serve this pourpose. Additionally documentation would still need to be created.

In the tests you may have noticed that in Python we clear the database tables between each test, but
we don't do this in Rust. The reason for this is by default pytest runs one test at a time while
the default in Rust is to run tests in parallel. Because of this if we were to clear the tables
between tests in Rust we end up with race conditions.

Let's use an example where we are checking that we can retrieve records from the database. First the
test will add records to the database to ensure there is something to retrieve, then retrieve the
records and check the result. Because the tests are running in parallel in Rust it is possible that
a different test finishes and clears the database before the tests retrieving the data finishes. If
this happens the test will fail because the setup data was deleted and there are no records to
retrieve.
