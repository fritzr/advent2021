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

All solutions use stable features as of rust 1.57.0 (f1edd0429 2021-11-29).
Cargo will figure out the packages I used (see `Cargo.toml`) but most days
use only `std`.

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
