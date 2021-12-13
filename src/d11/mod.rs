use std::io::BufRead;
use crate::{cli, Day, PartResult, util::vec2d::Vec2d};
use std::error::Error;

pub struct Day11;

fn read_grid(input: &mut dyn BufRead) -> Result<Vec2d<u8>, Box<dyn Error>> {
    let mut lines = input.lines().peekable();
    let width = lines.peek().ok_or("empty input")?.as_ref().map_err(|e| e.to_string())?.len();
    Ok(lines.flat_map(|s| s.expect("EOF").into_bytes())
       .map(|b| b - b'0')
       .collect::<Vec2d<u8>>()
       .reshaped_from(|_, len| (len / width, width))?)
}

impl Day for Day11 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let octopi = read_grid(input);
        println!("{:?}", octopi);
        Ok((PartResult::new(),PartResult::new()))
    }
}

