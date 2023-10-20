# Sh-mole

A simple utility for debugging everything shell-related (you like ~~kissing boys~~ debug-printing don't you?)

## Installation

You'll need a recent enough (see MSRV) Rust toolchain installed.

### Building from git

```sh
cargo install --git https://github.com/thecaralice/shmole.git
```

### Building from crates.io

```sh
cargo install shmole
```

## Usage

Simply call `mole` with any arguments you want. It will debug-print everything to stderr.

### "Everything"

- Current working directory
- Path to the executable
- Command-line arguments
- Environment variables

## Supported systems

- Unix-like
- Windows (might have problems with SGR sequences used for rendering bold stuff)
- maybe something else if it has an implementation for the corresponding functions in Rust stdlib
