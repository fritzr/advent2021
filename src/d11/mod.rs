use std::io::BufRead;
use crate::{cli, Day, PartResult, util};
use std::error::Error;

pub struct Day11;

impl Day for Day11 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let octopi = util::read_grid(input);
        println!("{:?}", octopi);
        Ok((PartResult::new(),PartResult::new()))
    }
}

