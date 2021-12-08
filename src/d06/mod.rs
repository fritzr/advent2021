use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;

pub struct Day6;

impl Day for Day6 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, _input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let mut num_days = 80;
        if let Some(args) = &opts.args {
            assert_eq!(args.len() > 0, true);
            num_days = args[0].parse()?;
        }
        if opts.verbose {
            println!("Simulating {} days", num_days);
        }
        // TODO
        Ok((PartResult::new(), PartResult::new()))
    }
}

