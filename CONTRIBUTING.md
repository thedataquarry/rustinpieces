Thank you for your interest in contributing to the _Rust In Pieces_ project!

There are a few ways of helping: critiquing the documentation and/or code examples, fixing
incorrect information, and fixing bugs.

It is encouraged for you to run the tests, formatters, and linters before submitting PRs. They will
all need to pass before the PR can be merged so you can save youself some time by running them
locally before submitting. The README of each piece contains instructions for running these checks.

Additionally we use [pre-commit](https://pre-commit.com) to run linting checks. To install
pre-commit create a virtual environment, activate it, and then install the requirements (Note - if
you have pre-commit installed at your system level you can skip this step):

```sh
# First time setup
python -m venv venv
source venv/bin/activate
pip install -r requirements-dev.txt

# For subsequent runs, simply activate the environment
source venv/bin/activate
```

Next install the pre-commit hooks:

```sh
pre-commit install
```

Now any time you commit your code pre-commit will run its checks. If you want to run the checks
before commiting you can do so manually:

```sh
pre-commit run --all-files
```

If committing Rust code, make sure to run Cargo's clippy and fmt on the `piece` so that linting is
respected. You can do this by eiter using the `Makefile` provided in each `piece`, or by running
the Cargo commands individually.

```sh
make all

# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
cargo test --quiet
cargo run --quiet
```

## Critiquing the documentation and/or code

This is the easiest way to contribute. Basically, as you read the documentation or experiment with
the code samples, if you find something confusing, incorrect, or missing, then you can file an
[issue](https://github.com/thedataquarry/rustinpieces/issues) explaining your concerns.

## Fixing incorrect information

If you find typos or incorrect links we welcome PRs to fix them. For these types of fixes there is
no need to open an issue first. For larger changes (or potentially adding a new piece), you are
encouraged to first open an issue to discuss the change before taking the time to make changes.

## Fixing bugs

If you find a bug in the code examples as you work through them, PRs for fixes are welcome. When
fixing a bug in either a Python or Rust file please check the other language's corresponding file
to see if the same bug exists there. If you know how to fix both then you can do both in one PR. If
you are unsure how to make the fix in one language let us know in your PR description so we can make
the additional fix.
