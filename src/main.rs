use std::error::Error;

mod cli;
mod util;
mod d01;
//mod d02;
//mod d03;
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

pub trait Day {
    fn run(&self, cli: &cli::Cli) -> Result<(String, String), Box<dyn Error>>;
}

const DAYS: [&dyn Day; 1] = [
    &d01::Day1{},
];

const MAX_DAY: usize = DAYS.len();

fn run_day(opts: &cli::Cli, day_index: usize) -> Result<(), Box<dyn Error>> {
    print!("Day {}: ", day_index + 1);
    if day_index >= MAX_DAY {
        println!("unimplemented");
    }
    else {
        let (part1, part2) = DAYS[day_index].run(&opts)?;
        println!("\n  Part 1: {}\n  Part 2: {}\n", part1, part2);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    use structopt::StructOpt;
    let opts = cli::Cli::from_args();
    if opts.day.0 == opts.day.1 {
        if opts.verbose {
            println!("Running day {}", opts.day.0);
        }
        return run_day(&opts, (opts.day.0 - 1).into());
    }
    else if opts.input.is_some() {
        println!("ERROR: cannot specify -i with multiple days");
        return Ok(());
    }
    let day_start: usize = (opts.day.0 - 1).into();
    let day_end: usize = MAX_DAY.min((opts.day.1 + 1).into());
    if opts.verbose {
        println!("Running days {}..{}", day_start, day_end);
    }
    for day_index in day_start..day_end {
        run_day(&opts, day_index)?;
    }
    Ok(())
}
