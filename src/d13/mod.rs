use std::io::BufRead;
use crate::{cli, Day, PartResult, util::vec2d::Vec2d};
use std::error::Error;

pub struct Day13;

const INPUT_POINTS_GUESS: usize = 1024;
const INPUT_FOLDS_GUESS: usize = 16;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

    fn fold(&self, point: Point) -> Point {
        match *self {
            Fold::Up(y) => Point(point.0, if point.1 > y { y - (point.1 - y) } else { point.1 }),
            Fold::Left(x) => Point(if point.0 > x { x - (point.0 - x) } else { point.0 }, point.1),
        }
    }
}

fn fold(points: &Vec<Point>, folds: &Vec<Fold>, verbose: bool) -> (Vec2d<char>, usize)
{
    // The last fold in each direction indicates the final shape.
    // (Points never overlap the fold lines.)
    let mut shape: (usize, usize) = (0, 0);
    for fold in folds.iter().rev() {
        match *fold {
            Fold::Left(x) => if shape.0 == 0 { shape = (usize::from(x), shape.1) },
            Fold::Up(y) => if shape.1 == 0 { shape = (shape.0, usize::from(y)) },
        }
        if shape.0 != 0 && shape.1 != 0 {
            break;
        }
    }
    // If we didn't have a fold in a particular direction, bound it by the maximal point.
    if shape.0 == 0 || shape.1 == 0 {
        for point in points {
            if shape.0 < (usize::from(point.0) + 1) {
                shape = (usize::from(point.0) + 1, shape.1);
            }
            if shape.1 < (usize::from(point.1) + 1) {
                shape = (shape.0, usize::from(point.1) + 1);
            }
        }
    }
    // Perform all folds on each point.
    // XXX Can we combine all folds into a single affine transformation?
    if verbose {
        println!("final shape {:?}", shape);
    }
    let mut result = Vec2d::from('.', shape);
    let mut nonzero = 0;
    for point in points.iter().map(|point| {
        folds.iter().fold(*point, |point, fold| fold.fold(point))
    }) {
        let index = (usize::from(point.0), usize::from(point.1));
        if verbose {
            println!("marking folded point {:?}", point);
        }
        if result[index] == '.' {
            nonzero += 1;
        }
        result[index] = '*';
    }
    (result, nonzero)
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
        Ok((PartResult::from(|| {
                if opts.verbose {
                    println!("1 fold:");
                }
                let one_fold = folds.iter().take(1).cloned().collect();
                let (graph, visible) = fold(&points, &one_fold, opts.verbose);
                if opts.verbose {
                    println!("\n{}\n", graph);
                }
                visible
            }),
            PartResult::from(|| {
                if opts.verbose {
                    println!("All {} folds:", folds.len());
                }
                let (graph, _) = fold(&points, &folds, opts.verbose);
                format!("\n{}", graph)
            })))
    }
}

