use std::num::ParseIntError;
use std::num::IntErrorKind;
pub use structopt::StructOpt;

fn parse_day_range(s: &str) -> Result<(u8, u8), ParseIntError> {
    if s.trim().len() == 0 {
        return Ok((1, 25));
    }
    let mut split = s.split("..");
    let lb = match split.next().unwrap().parse::<u8>() {
        Ok(num) => num,
        Err(error) => match error.kind() {
            IntErrorKind::Empty => 1u8,
            _ => return Err(error),
        }
    };
    let ub = match split.next() {
        Some(ub) => match ub.parse::<u8>() {
            Ok(num) => num,
            Err(error) => match error.kind() {
                IntErrorKind::Empty => 25u8,
                _ => return Err(error),
            }
        },
        None     => lb, // no '..' was present, use single range
    };
    Ok((lb, ub))
}

#[derive(Debug, StructOpt)]
#[structopt(name="advent2021", about="Solutions for Advent of Code 2021 in Rust.")]
pub struct Cli {
    /// Display runtime of day(s).
    #[structopt(short, long)]
    pub time: bool,
    /// Force grab input from web again.
    #[structopt(short, long)]
    pub web: bool,
    /// Use alternate input file
    #[structopt(short, long)]
    pub input: Option<String>,
    /// Verbose output.
    #[structopt(short, long)]
    pub verbose: bool,
    /// Day(s) to run (1-25).
    ///
    /// Format is like slice notation: [start][..][end].
    ///
    /// If a single number is given, only runs the given day. Otherwise
    /// if dots are present, either enpoint is clamped to 1..25 when omitted.
    #[structopt(default_value="1..25", parse(try_from_str=parse_day_range))]
    pub day: (u8, u8),
}
