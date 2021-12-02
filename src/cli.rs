use std::num::ParseIntError;
pub use structopt::StructOpt;

fn parse_day_range(s: &str) -> Result<(u8, u8), ParseIntError> {
    if s.trim().len() == 0 {
        return Ok((1, 25));
    }
    let mut split = s.split("..");
    let lb = match split.next() {
        Some(lb) => lb.parse()?,
        None     => 0u8,
    };
    let ub = match split.next() {
        Some(ub) => ub.parse()?,
        None     => 25u8,
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
    /// Either enpoint is clamped to 1..25 when omitted.
    #[structopt(default_value="", parse(try_from_str=parse_day_range))]
    pub day: (u8, u8),
}
