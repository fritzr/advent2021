use std::error::Error;

mod cli;
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

fn main() -> Result<(), Box<dyn Error>> {
    use structopt::StructOpt;
    let opts = cli::Cli::from_args();
    let day_end = MAX_DAY.min((opts.day.1 + 1).into());
    if opts.verbose {
        println!("Running day(s) {}..{}", opts.day.0, day_end);
    }
    let day_start = opts.day.0 - 1;
    for day_index in day_start.into()..day_end {
        println!("Day {}:", day_index + 1);
        let (part1, part2) = DAYS[day_index].run(&opts)?;
        println!("  Part 1: {}\n  Part 2: {}\n", part1, part2);
    }
    Ok(())
}
