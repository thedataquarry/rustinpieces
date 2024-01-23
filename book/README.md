# Building the Book

This page contains the source code for the book _Rust in Pieces_, written in
Markdown.

## Setup

To build and run the book locally, you'll need to install
[mdBook](https://github.com/rust-lang/mdBook).

Assuming you have Rust installed, you can install `mdbook` with Cargo:

```bash
cargo install mdbook
```

This builds mdBook from source on your machine. Alternatively, if you're
using macOS, you can install it with Homebrew:

```bash
brew install mdbook
```

## Build and serve

To run the book locally, run the following command from this level of the
repository (after running `cd book`):

```bash
mdbook serve --open
```

This builds and serves the book on localhost. The `--open` flag will open the
book in your default browser.

## Run tests

It's strongly recommended to run tests prior to pushing any updates.

```bash
mdbook test
```

This runs mdBook's test suite, which checks for broken links and incorrectly
formatted Rust code blocks.

> [!NOTE]
> mdBook will test all code blocks in the book that are marked as
> `rust` or `rs`. If you leave a code block unmarked (i.e., just enclose
> code in triple backticks), **the test suite will fail**, so make sure to
> mark all code blocks, including bash/console with the appropriate language.
