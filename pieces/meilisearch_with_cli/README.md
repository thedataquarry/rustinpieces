# Interact with Meilisearch though a CLI

Update settings, index data, and search using [Meilisearch](https://www.meilisearch.com/) with a CLI.

## Goal

[Meilisearch](https://github.com/meilisearch/meilisearch) is a fast, open source, typo-tolerant search
engine with a focus on being easy to setup and use. In Meilisearch, indexes are created to store
documents and their associated settings. These indexes are similar to tables in relational databases
or collections in document databases.

In this example we will create and update the settings for an index, send documents to the server for
indexing, and perform searches all through a CLI.

For data we will use the [wine reviews dataset from Kaggle](https://www.kaggle.com/datasets/zynicide/wine-reviews).
The dataset is in the following format:

```
{
  id: number | null,
  title: string,
  description: string,
  taster_name: string | null,
  designation: string | null,
  variety: string | null,
  country: string | null,
  winery: string | null,
  points: string | null,
  taster_twitter_handle: string | null,
  price: number | null,
  region_1: string | null,
  region_2: string | null,
  province: string | null,
}
```

We will update the searchable attributes to include `title`, `description`, `taster_name`, `designation`,
`variety`, `province`, `country`, and `winery`. Reducing the number of searchable fields will make indexing
faster because we don't need to waste time indexing fields that aren't going to be searched.

Additionally we will make `title` and `country` sortable files. The results in Meilisearch are ordered
based on ranking rules with the default rules being: `["words", "typo", "proximity", "attribute", "sort", "exactness"]`.
The result of this is records are only sorted if `words`, `typo`, `proximity`, and `attribute` all
match for multiple records. Because we want sort to take precedence we will update the ranking rules
to `["sort", "words", "typo", "proximity", "attribute", "exactness"]` in addition to adding `title`
and `country` as sortable fields.

The use of a CLI is common for managing indexes and preforming indexing of documents, while searching
is typically done though a web API or front end. Since APIs and front ends are out of scope for this
example we will also implement a search through the CLI.

## Python Setup

Install the dependencies in a virtual environment via `requirements.txt`.

```bash
# First time setup
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# For subsequent runs, simply activate the environment
source venv/bin/activate
```

In this project installing `requirements.txt` will also install the project itself as an editable
install containing an entry point called `meilisearch-cli`. This allows us to run the program through
the entry point, `meilisearch-cli create-index` for example, without the need to use the python command,
`python main.py create-index`.

This project uses [typer](https://github.com/tiangolo/typer) to create the CLI,
[rich](https://github.com/Textualize/rich) to pretty print text and create a progress spinner,
[srsly](https://github.com/explosion/srsly) to handle the jsonl.gz files,
[meilisearch-python-sdk](https://github.com/sanders41/meilisearch-python-sdk) to manage interactions
with Meilisearch, and [pytest-meilisearch](https://github.com/sanders41/pytest-meilisearch) to help
with testing.

### Run tests

First start the Meilisearch Docker container from the `pieces/meilisearch_with_cli` directory if it
is not already running:

```bash
docker compose up
```

Then run the tests:

```bash
$ pytest -v
==================================================================================== test session starts ====================================================================================
platform linux -- Python 3.12.1, pytest-7.4.4, pluggy-1.3.0 -- /home/paul/development/rust/rustinpieces/pieces/meilisearch_with_cli/python/.venv/bin/python
cachedir: .pytest_cache
rootdir: /home/paul/development/rust/rustinpieces/pieces/meilisearch_with_cli/python
configfile: pyproject.toml
plugins: meilisearch-0.3.3, anyio-4.2.0
collected 3 items

test_main.py::test_create_index PASSED                                                                                                                                                [ 33%]
test_main.py::test_index_data PASSED                                                                                                                                                  [ 66%]
test_main.py::test_search PASSED                                                                                                                                                      [100%]

===================================================================================== 3 passed in 0.68s =====================================================================================
```

### Run CLI commands

First start the Meilisearch Docker container from the `pieces/meilisearch_with_cli` directory if it
is not already running:

```bash
docker compose up
```

Then create the index and update its settings:

```bash
meilisarch-cli create-index
```

Once the index is created, add the documents to the index:

```bash
meilisearch-cli index-data
```

Now you can run searches:

```bash
meilisearch-cli search red
```

You can use `--help` to see information about the commands:

```bash
$ meilisearch-cli --help
Usage: meilisearch-cli [OPTIONS] COMMAND [ARGS]...

╭─ Options ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ --version             -v                                       Show the instealled version                                                                                                │
│ --install-completion          [bash|zsh|fish|powershell|pwsh]  Install completion for the specified shell. [default: None]                                                                │
│ --show-completion             [bash|zsh|fish|powershell|pwsh]  Show completion for the specified shell, to copy it or customize the installation. [default: None]                         │
│ --help                                                         Show this message and exit.                                                                                                │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
╭─ Commands ────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ create-index                                                                                                                                                                              │
│ index-data                                                                                                                                                                                │
│ search                                                                                                                                                                                    │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯

$ meilisearch-cli create-index --help
Usage: meilisearch-cli create-index [OPTIONS]

╭─ Options ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ --index-name  -i      TEXT  The name to use for the index [default: wine]                                                                                                                 │
│ --help                      Show this message and exit.                                                                                                                                   │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯

$ meilisearch-cli index-data --help
Usage: meilisearch-cli index-data [OPTIONS]

╭─ Options ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ --data-path   -d      FILE  Path to the data file [default: ../data/winemag-data-130k-v2.jsonl.gz]                                                                                        │
│ --index-name  -i      TEXT  The name to use for the index [default: wine]                                                                                                                 │
│ --wait        -w            Wait for the data to finish indexing                                                                                                                          │
│ --help                      Show this message and exit.                                                                                                                                   │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯

$ meilisearch-cli search --help
Usage: meilisearch-cli search [OPTIONS] QUERY

╭─ Arguments ───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ *    query      TEXT  The search to preform [default: None] [required]                                                                                                                    │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
╭─ Options ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│ --limit       -l      INTEGER  Limit the number of search results [default: 20]                                                                                                           │
│ --sort        -s      TEXT     Sort order for the results [default: None]                                                                                                                 │
│ --index-name  -i      TEXT     The name to use for the index [default: wine]                                                                                                              │
│ --help                         Show this message and exit.                                                                                                                                │
╰───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

### Takeaways

Typically in a CLI like this asyncio will not provide any benefits, and may actually be slower
because of the need to create an event loop. An exception in this case is that `meilisearch-python-sdk`
has an optimization that sends documents for indexing concurrently when adding them in batches,
as we are here, if asyncio is used. Because of this we take advantage of `meilisearch-python-sdk`
providing both and `AsyncClient` and a `Client` to use asyncio when indexing the documents, while
performing the index creation and searches synchronously.

In the tests you will notice that the index name is created using a uuid. `pytest-meilisearch` is
set to clear all indexes between test runs so there should be no name clashes. However, because our
Meilisearch Docker image is being shared here between Python and Rust for both testing and trying
out the examples it is possible that other indexes are present. The use of a uuid for the index
ensures our test results are not skewed by other data being present.

## Rust setup

This project uses [clap](https://github.com/clap-rs/clap) to create the CLI,
[colored](https://github.com/colored-rs/colored) to print text with color,
[indicatif](https://github.com/console-rs/indicatif) to create a progress spinner,
[flate2](https://github.com/rust-lang/flate2-rs) to handle the jsonl.gz files, [serde](https://github.com/serde-rs/serde)
to handle serilization and deserilization, [serde-json](https://github.com/serde-rs/json) to handle
JSON, [async-std](https://github.com/async-rs/async-std) for asyncio, and [meilisearch-rust](https://github.com/meilisearch/meilisearch-rust)
to manage interactions with Meilisearch.

```bash
cargo add anyhow
cargo add async-std --features attributes
cargo add clap --features derive
cargo add colored
cargo add falate2
cargo add indicatif
cargo add lazy_static
cargo add meilisearch-sdk
cargo add serde --features derive
cargo add serde-json
cargo add --dev tempfile
cargo add --dev uuid --features v4
```

By default when building a package in rust the binary will be the name of the project, `meilisearch_with_cli`
in this example. In order to match what we have done in Python, we have added a `bin` section to
the `Cargo.toml` file with a name of `meilisearch-cli`. By doing this Rust will name the binary
`meilisearch-cli` when compiling so when running the program with the binary you use, for example,
`meilisearch-cli create-index` rather than `meilisearch_with_cli create-index`. Note however, for
testing and developing we will still use `cargo run`. To use the CLI with `cargo run` you provide
an extra `--` and this tells cargo that it is the end of the cargo commands and the start of the
commands to pass to the program. For example to create the index you will run `cargo run -- create-index`.

### Run linting and tests

First start the Meilisearch Docker container from the `pieces/meilisearch_with_cli` directory if it
is not already running:

```bash
docker compose up
```

The provided `Makefile` runs the formatter, linter, and tests all in sequence.

```bash
make all

# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
cargo test --quiet
```

### Run linter and formatter only

Cargo provides out-of-the-box for formatting (`cargo fmt --all`), compile checks (`cargo check --all-targets`),
and linting (`cargo clippy --all-targets`). The following command runs both. It's highly recommended
to run both prior to pushing Rust code to a repository.

```bash
make format
make check
make lint
# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
```

### Run tests only

First start the Meilisearch Docker container from the `pieces/meilisearch_with_cli` directory if it
is not already running:

```bash
docker compose up
```

The Rust in-built test client allows tests to be defined within the same file as the code being tested.
Because Rust is a compiled language, the compiler will know to ignore the tests when building the
final binary for runtime.

Tests are run using `make test` or `cargo test`.

```bash
make test
cargo test

Finished test [unoptimized + debuginfo] target(s) in 0.09s
Running unittests src/main.rs (/home/paul/development/rust/rustinpieces/target/debug/deps/meilisearch_cli-e08e59e4efbca66a)

running 3 tests
test tests::test_search ... ok
test tests::test_create_index ... ok
test tests::test_index_data ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s
```

### Run CLI commands

First start the Meilisearch Docker container from the `pieces/meilisearch_with_cli` directory if it
is not already running:

```bash
docker compose up
```

Then create the index and update its settings:

```bash
cargo run -- create-index
```

Once the index is created add the documents to the index:

```bash
cargo run -- index-documents
```

Now you can run searches:

```bash
cargo run -- search red
```

You can use `--help` to see information about the commands:

```bash
$ cargo run -- --help
Usage: meilisearch-cli <COMMAND>

Commands:
  create-index  Create the wine index and update settings
  index-data    Index the wine data
  search        Preform a search
  help          Print this message or the help of the given subcommand(s)

$ cargo run -- create-index --help
Create the wine index and update settings

Usage: meilisearch-cli create-index [OPTIONS]

Options:
  -i, --index-name <INDEX_NAME>  The name to use for the index. Default: wine
  -h, --help                     Print help

$ cargo run -- index-data --help
Index the wine data

Usage: meilisearch-cli index-data [OPTIONS]

Options:
  -d, --data-path <DATA_PATH>    Index the wine data
  -i, --index-name <INDEX_NAME>  The name to use for the index. Default: wine
  -w, --wait                     Wait for the data to finish indexing
  -h, --help                     Print help

$ cargo run -- search --help
Preform a search

Usage: meilisearch-cli search [OPTIONS] <QUERY>

Arguments:
  <QUERY>  The search to preform

Options:
  -l, --limit <LIMIT>            Limit the number of search results
  -s, --sort <SORT>              Sort order for the results
  -i, --index-name <INDEX_NAME>  The name to use for the index. Default: wine
  -h, --help                     Print help
```

### Takeaways

In Rust asyncio is run using crates, of which you have options to chose from. `meilisearch-rust` was
designed to let the user pick which async crate to use rather than dictating one. In this example we
use async-std, but we could have just as easily used tokio or some other crate. It is possilbe to make
the program synchronous in `meilisearch-rust` using the [futures](https://github.com/rust-lang/futures-rs)
crate `block_on`, however it is not as simple to mix asyncio and synchronous calls as we have done in
the Python example. Because of this we choose to make everything async in the Rust examples.

As with the Python example, we create indexes using uuids for the names in the tests. Unlike in Python
where the tests are run syncronously, the Rust test runner runs tests in parallel. This makes the
use of uuids here more important in Rust. Without using unique names for the indexes we would end up
with race conditions in the tests. Sometimes they would pass and sometimes they would fail without an
obvious reason for why. This is because one test could be using the index from another test by chance.

## Performance

One big performance advantage Rust has over Python in CLIs, as a compiled language, is startup time.
With Python, there is overhead associated with starting the Python interpreter which causes a noticeable
delay in startup time, while Rust is almost instant. An informal test using the `time` command illustrates
this well. For `create-index`, the Rust program runs in an average of 184ms while the Python program
average is 796ms on a System76 Lemur Pro with a 13th Gen Intel Core i7-1355U processor and 40GB of
RAM.

To further illustrate this, for `index-data` the average run time for Rust is 3.41s of which 3.29s is
spent doing the actual indexing. The average run time for Python is 3.03s of which 1.19s is spent on
the indexing. Because of the async batch indexing optimizations in `meilisearch-python-async` the Python
example is significantly faster than the Rust example at indexing, however almost all of this advantage
is lost to the startup time of Python.

The `search_benchmark.sh` script runs the same 10 searches in Rust and Python, each time running a
new search through the CLI. The total runtime for the 10 searches in Rust was 0.143s while in Python
it was 5.945s. This is 42 times faster attributed almost entirely to startup time! The point of this
example is not to show that running rapid commands through the CLI like this is faster. Instead, it
illustrates that each run of the CLI is significantly faster which adds up over time. If you are taking
the time to write a CLI it is likely that it will be run many times. In this situation the speed advantage
adds up over time.
