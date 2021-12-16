use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;

pub struct Day13;

const INPUT_POINTS_GUESS: usize = 1024;
const INPUT_FOLDS_GUESS: usize = 16;

#[derive(Debug)]
struct Point(u16, u16);

impl Point {
    fn from(line: String) -> Result<Point, Box<dyn Error>> {
        let mut parts = line.splitn(2, ",");
        Ok(Point(
            parts.next().ok_or("missing X coord")?.parse()?,
            parts.next().ok_or("missing Y coord")?.parse()?,
        ))
    }
}

#[derive(Debug)]
enum Fold {
    Up(u16),   // fold up over y=...
    Left(u16), // fold left over x=...
}

impl Fold {
    fn from(line: String) -> Result<Fold, Box<dyn Error>> {
        let mut parts = line.splitn(2, "=");
        match parts.next().ok_or("missing 'fold along'")?
            .chars().rev().next().ok_or("empty axis")?
        {
            'y' => Ok(Fold::Up(parts.next().ok_or("missing fold value")?.parse()?)),
            'x' => Ok(Fold::Left(parts.next().ok_or("missing fold value")?.parse()?)),
            axis => Err(format!("invalid fold axis '{}", axis).into()),
        }
    }
}

fn read_points_folds(input: &mut dyn BufRead)
    -> Result<(Vec<Point>, Vec<Fold>), Box<dyn Error>>
{
    let mut lines = input.lines();
    let mut points = Vec::with_capacity(INPUT_POINTS_GUESS);
    while let Some(line) = lines.next() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        points.push(Point::from(line)?);
    }
    let mut folds = Vec::with_capacity(INPUT_FOLDS_GUESS);
    while let Some(line) = lines.next() {
        let line = line?;
        folds.push(Fold::from(line)?);
    }
    Ok((points, folds))
}

impl Day for Day13 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let (points, folds) = read_points_folds(input)?;
        if opts.verbose {
            println!("points:\n{:?}\n\nfolds:\n{:?}\n", points, folds);
        }
        Ok((PartResult::new(), PartResult::new()))
    }
}

