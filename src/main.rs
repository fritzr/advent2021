use std::error::Error;
use std::io::BufRead;
use std::time::{Instant, Duration};
use std::string::ToString;

mod cli;
mod util;
mod d01;
mod d02;
mod d03;
//mod d04;
//mod d05;
//mod d06;
//mod d07;
//mod d08;
//mod d09;
//mod d10;
//mod d11;
//mod d12;
//mod d13;
//mod d14;
//mod d15;
//mod d16;
//mod d17;
//mod d18;
//mod d19;
//mod d20;
//mod d21;
//mod d22;
//mod d23;
//mod d24;
//mod d25;

pub struct PartResult {
    answer: String,
    time: Duration,
}

impl PartResult {
    pub fn from<F, T>(mut part: F) -> PartResult
        where T: ToString, F: FnMut() -> T
    {
        let time = Instant::now();
        let answer = part().to_string();
        let time = time.elapsed();
        PartResult { answer, time }
    }
}

pub trait Day {
    fn mod_path(&self) -> &str;
    fn run(&self, input: &mut dyn BufRead, cli: &cli::Cli) -> Result<(PartResult, PartResult), Box<dyn Error>>;
}

const DAYS: [&dyn Day; 3] = [
    &d01::Day1{},
    &d02::Day2{},
    &d03::Day3{},
];

const MAX_DAY: usize = DAYS.len();

fn run_day(opts: &cli::Cli, day_index: usize) -> Result<(), Box<dyn Error>> {
    print!("Day {}: ", day_index + 1);
    if day_index >= MAX_DAY {
        println!("unimplemented");
    }
    else {
        let day = &DAYS[day_index];
        let input_clock = Instant::now();
        let mut input = util::read_input(opts, day.mod_path())?;
        let input_clock = input_clock.elapsed();
        let (part1, part2) = day.run(input.as_mut(), &opts)?;
        println!();
        println!("  Part 1: {}\n  Part 2: {}", part1.answer, part2.answer);
        if opts.time {
            println!("  Time\t{:?}\t{:?}\t{:?}\t{:?}",
                input_clock, part1.time, part2.time,
                input_clock + part1.time + part2.time);
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    use structopt::StructOpt;
    let opts = cli::Cli::from_args();
    if opts.time {
        println!("Times:\tinput\t\tpart 1\tpart 2\ttotal");
    }
    if opts.day.0 == opts.day.1 {
        return run_day(&opts, (opts.day.0 - 1).into());
    }
    else if opts.input.is_some() {
        println!("ERROR: cannot specify -i with multiple days");
        return Ok(());
    }
    let day_start: usize = (opts.day.0 - 1).into();
    let day_end: usize = MAX_DAY.min((opts.day.1 + 1).into());
    let clock = Instant::now();
    for day_index in day_start..day_end {
        run_day(&opts, day_index)?;
    }
    if opts.time {
        println!("Total runtime: {:?}", clock.elapsed());
    }
    Ok(())
}
