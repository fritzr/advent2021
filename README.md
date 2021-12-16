# advent2021
Puzzle solutions for Advent of Code 2021 in Rust.

## Running

Running the binary:

```
  cargo run -- <args>
```

Or:

```
  cargo build
  ./target/debug/advent2021 <args>
```

## Dependencies

Most solutions use stable features as of rust 1.57.0 (f1edd0429 2021-11-29).
Day 14 uses the experimental Cursor feature of LinkedList
`#![feature(linked_list_cursors)]`, which requires a nightly build of the
compiler. Therefore, you need to first run `rustup install nightly` and
configure the nightly compiler toolchain, e.g. `cargo +nightly`.

## Usage

```
advent2021 0.1.0
Solutions for Advent of Code 2021 in Rust.

USAGE:
    advent2021 [FLAGS] [OPTIONS] [day]

FLAGS:
    -h, --help       Prints help information
    -t, --time       Display runtime of day(s)
    -V, --version    Prints version information
    -v, --verbose    Verbose output
    -w, --web        Force grab input from web again

OPTIONS:
    -i, --input <input>    Use alternate input file

ARGS:
    <day>    Day(s) to run (1-25) [default: 1..25]
```
