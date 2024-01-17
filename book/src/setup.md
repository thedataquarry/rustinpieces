# Setup & installation

This section provides an opinionated guide to setting up your development environment for working with Rust and Python.
If you're an experienced developer in either language, feel free to skip this section.

## Python

For macOS/Linux users, it's recommended to manage Python versions using [pyenv](https://github.com/pyenv/pyenv). `pyenv` lets you easily switch between multiple versions of Python. It's simple, unobtrusive, and follows the UNIX tradition of single-purpose tools that do one thing well.

Follow the instructions from the [installation steps](https://github.com/pyenv/pyenv?tab=readme-ov-file#installation) section of the README to install `pyenv` on your system.

Windows users can use [pyenv-win](https://github.com/pyenv-win/pyenv-win), a fork of `pyenv` that allows you to install and manage Windows-native Python versions.

### Python version

This book uses Python 3.11.x. You can install the latest minor version using `pyenv`:

```bash
pyenv install 3.11.7
```

### Virtual environments

It's recommended to use [virtual environments](https://docs.python.org/3/tutorial/venv.html) to manage your Python dependencies. This allows you to create isolated environments for each project, and avoid dependency conflicts between projects.

The `venv` module is included in the Python standard library, so you don't need to install anything extra to use it.

To create a virtual environment, run the following command:

```bash
# Setup a new environment for the first time
python -m venv venv
# Activate the environment
source venv/bin/activate
```

You can deactivate the environment by running `deactivate` in your shell.

## Rust

For macOS/Linux users, [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos) is the recommended to manage Rust versions. Using this tool, you can easily switch between multiple versions of Rust, and it also ships with the `cargo` package manager.

See the [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows) for instructions on how to install `rustup` on Windows.

### Rust version

This book uses Rust 1.75.x. You can install the latest minor version using `rustup`:

```bash
rustup install 1.75.0
```

You can start a new Rust project in your local directory by running `cargo new <project-name>`, and you're ready to go!
