use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::{Instant, Duration};

pub struct Day5;

const INPUT_LEN_GUESS: usize = 500;
type Coord = u16;

#[derive(Debug, Clone)]
struct Point {
    col: Coord,
    row: Coord,
}

impl Point {
    // pub fn new() -> Point { Point { col: 0, row: 0 } }
    pub fn from(string: String) -> Result<Point, Box<dyn Error>> {
        let mut coords = string.split(",");
        Ok(Point {
            col: coords.next().ok_or("expected first point coordinate")?.parse()?,
            row: coords.next().ok_or("expected second point coordinate")?.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // pub fn new() -> Line { Line { start: Point::new(), end: Point::new() } }
    pub fn from(string: String) -> Result<Line, Box<dyn Error>> {
        let mut points = string.split(" -> ");
        Ok(Line {
            start: Point::from(points.next().ok_or("expected start point")?.into())?,
            end: Point::from(points.next().ok_or("expected end point")?.into())?,
        })
    }
    pub fn is_horiz(&self) -> bool { self.start.row == self.end.row }
    pub fn is_vert(&self) -> bool { self.start.col == self.end.col }
}

fn read_lines(input: &mut dyn BufRead) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines = Vec::<Line>::with_capacity(INPUT_LEN_GUESS);
    for line in input.lines() {
        lines.push(Line::from(line?)?);
    }
    Ok(lines)
}

fn intersections<'a, I>(lines: I) -> Vec<Point>
    where I: IntoIterator<Item=&'a Line>
{
    // TODO
    lines.into_iter().map(|line| line.start.clone()).collect()
}

impl Day for Day5 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let lines = read_lines(input)?;
        let part1 = intersections(lines.iter().filter(|l| l.is_horiz() || l.is_vert())).len();
        let time = time.elapsed();
        Ok((PartResult { answer: part1.to_string(), time },
            PartResult { answer: "unimplemented".into(), time: Duration::new(0, 0) }))
    }
}

