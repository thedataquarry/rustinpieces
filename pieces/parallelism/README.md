# Parallelism

Compute the number of male and female pronouns in news articles via parallel processing.

## Goal

In this piece, we use parallel processing to compute the number of male and female pronouns in
news articles. We'll use the `rayon` crate in Rust and the `concurrent.futures` module in Python to
parallelize the computation, comparing their performance and ease of use.

The aim of computing pronouns is to get a (very) crude measure of how many men and women are
covered by writers in various news articles from a dataset, in this case, the
_[All the news](https://www.kaggle.com/datasets/snapcrack/all-the-news)_ dataset from Kaggle.

## Approach

The pronouns we'll be looking for are: `he`, `him`, `his`, `she`, `her`, `hers`.

Consider a simple example of a news article's content text:

```
She says she belongs here and that he makes her feel at home.
```

The pronouns in this text are: `she`, `she`, `he`, `her`, giving a male pronoun count
of 1 and a female pronoun count of 3.

To arrive at these numbers, the following steps are performed:

* Clean the text by removing punctuation and converting to lowercase
* Split the text into words
* Use regex to identify patterns that match how the pronouns are typically used
* Count the number of male and female pronouns in each article
* Process multiple articles at once, in batches, by utilizing multiple CPU cores (if available)

Because each article's counts are independent of one another, this can be viewed as an
*embarrassingly parallel* problem, which can be easily parallelized.

## Inputs

The input is a truncated version of the "All the News" dataset, containing 10 records
each in three files: `./data/articles1.csv`, `./data/articles2.csv` and `./data/articles3.csv`.
The full dataset is not provided here as it's ~680 MB in size and contains 143,000
records. You can download the full dataset from [here](https://www.kaggle.com/datasets/snapcrack/all-the-news)
(note that you need to sign into Kaggle to download it).

The following fields are present in the input CSVs:

* `_`: Unknown identifier (useless field)
* `id`: Unique identifier for the article
* `title`: Title of the article
* `publication`: Name of the publication
* `author`: Name of the author
* `date`: Date of publication
* `year`: Year of publication
* `month`: Month of publication
* `url`: URL of the article
* `content`: Text of the article

> [!NOTE]
> The raw data header contains a missing field, so a CSV parser will fail to infer the headers
> correctly. To fix this, modify the header to include a dummy `_` field at the start of the header
> as can be seen in the example data provided in this piece.

## Output

The outputs are a set of CSV files with the `_processed` suffix, containing the following fields:

* `id`: Unique identifier for the article
* `publication`: Name of the publication
* `author`: Name of the author
* `date`: Date of publication
* `num_male_pronouns`: Number of male pronouns in the article's text
* `num_female_pronouns`: Number of female pronouns in the article's text

We also display the run time comparisons of the parallel processing code in Rust and Python, for this
sample dataset in this repo, as well as the full dataset downloaded from Kaggle.

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

### Run scripts

A single script `main.py` is used to run the parallel processing code, that divides up each CSV's
records into batches and processes them via `concurrent.futures.ProcessPoolExecutor`.

The script is run as follows:

```bash
python main.py
```

### Run tests

Tests can be run as follows:

```bash
$ pytest -v
======================================= test session starts ========================================
platform darwin -- Python 3.11.7, pytest-8.0.0, pluggy-1.4.0 -- /Users/prrao/.pyenv/versions/3.11.7/bin/python3.11
cachedir: .pytest_cache
rootdir: /Users/prrao/code/rustinpieces/pieces/parallelism/python
plugins: Faker-22.0.0
collected 5 items

test_main.py::test_record PASSED                                                             [ 20%]
test_main.py::test_count_gendered_pronouns PASSED                                            [ 40%]
test_main.py::test_clean_text PASSED                                                         [ 60%]
test_main.py::test_calculate_counts PASSED                                                   [ 80%]
test_main.py::test_process_batches PASSED                                                    [100%]

======================================== 5 passed in 0.15s =========================================
```

### Results

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Python version used was `3.11.7`.


| numRecords | Python     | Rust      | Speedup factor |
| ---------- | ---------- | --------- | -------------- |
| 10         | 0.255 sec  | 0.192 sec | 1.3x           |
| 143000     | 13.493 sec | 3.416 sec | 4.0x           |

For the full dataset, the Rust code using `rayon` is about 4x faster than the Python code. This
performance difference would only increase as the number of records increases.

## Rust Setup

Install dependencies via Cargo. In this piece, we use the `rayon` crate to parallelize the
computation, and the `csv` and `regex` crates to parse the CSV and clean the text. As always, `serde`
is used for serialization and deserialization of the CSV records.

```bash
cargo add csv
cargo add rayon
cargo add regex
cargo add serde --features derive
```

### Run scripts

The provided `Makefile` runs the formatter, linter, tests for `main.rs` file all in sequence.

```bash
make all

# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
cargo test --quiet
cargo run --quiet
```

Ensure to run the Rust code in release mode for best performance, as follows.

```bash
cargo run --release --quiet
# or
cargo run -r --quiet
```

### Run tests

The Rust in-built test client allows tests to be defined within the same file as the code being
tested. Because Rust is a compiled language, the compiler will know to ignore the tests when
building the final binary for runtime.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet
```

```bash
running 4 tests
test tests::test_load_csv ... ok
test tests::test_clean_text ... ok
test tests::test_count_gendered_pronouns ... ok
test tests::test_process_record ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Results

The results for the data loading and for the query throughput are shown below.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Rust version used was `1.75.0`.

| numRecords | Python     | Rust      | Speedup factor |
| ---------- | ---------- | --------- | -------------- |
| 10         | 0.255 sec  | 0.192 sec | 1.3x           |
| 143000     | 13.493 sec | 3.416 sec | 4.0x           |

For the full dataset, the Rust code using `rayon` is about 4x faster than the Python code. This
performance difference would only increase as the number of records increases. It should be noted
here that Rayon allows for easy parallelization of processing the batches of records, as well as
capturing the regex pattern and the pronoun counts, and it uses work-stealing to distribute the
work across multiple CPU cores, ensuring that the work is as evenly distributed as possible while
keeping the most CPU cores active.

## Takeaways

The Rust code using `rayon` is about 4x faster than the Python code using `concurrent.futures` for this
dataset of 143K records. Although Rust was expected to be faster, it's breathtaking to see the
performance difference in action, considering how easy it was to parallelize the computation in Rust.

The parallelization approaches in Python and Rust could not be more different. In Python, the
`concurrent.futures` module uses a `ProcessPoolExecutor`, where the user has to specify the maximum
number of CPU cores to use, and the module takes care of the rest. In Rust, Rayon uses work-stealing
to distribute the work across multiple threads, and by default, it uses as many threads as there are
CPU cores available. It's highly recommended to read the Rayon
[FAQ](https://github.com/rayon-rs/rayon/blob/master/FAQ.md) page to learn more about how it works
under the hood.

Because of Rayon's design, the most CPU cores (and associated threads) are kept active, meaning that
the developer can focus on the program's logic and let Rayon handle the parallelization. This is in
contrast to Python, where the developer is responsible for batching the records beforehand. All in
all, the Rust code is faster, quite concise and easy to read, and the `rayon` crate makes
parallelizing the computation a breeze.
