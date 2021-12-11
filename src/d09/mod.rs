use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;

pub struct Day9;

impl Day for Day9 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, _input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        Ok((PartResult::new(), PartResult::new()))
    }
}

