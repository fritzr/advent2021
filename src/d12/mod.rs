use std::io::BufRead;
use crate::{cli, Day, PartResult, util, util::vec2d::Vec2d};
use std::error::Error;

pub struct Day12;

impl Day for Day12 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        Ok((PartResult::new(), PartResult::new()))
    }
}

