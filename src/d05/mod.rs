use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::Ordering;

pub struct Day5;

type Coord = u16;
const INPUT_NCOLS: usize = 1000;
const INPUT_NROWS: usize = 1000;
const INPUT_GRID: usize = INPUT_NROWS * INPUT_NCOLS;
const INPUT_LEN_GUESS: usize = 500;

#[derive(Debug, Clone, Copy, PartialEq)] // PartialOrd, Eq
struct Point {
    col: Coord,
    row: Coord,
}

#[derive(Debug, Clone, Copy)]
struct PointDelta(Ordering, Ordering);

impl Point {
    // pub fn new() -> Point { Point { col: 0, row: 0 } }
    pub fn from(string: String) -> Result<Point, Box<dyn Error>> {
        let mut coords = string.split(",");
        let point = Point {
            col: coords.next().ok_or("expected first point coordinate")?.parse()?,
            row: coords.next().ok_or("expected second point coordinate")?.parse()?,
        };
        if usize::from(point.col) >= INPUT_NCOLS {
            Err(format!("col {} exceeds ncols {}", point.col, INPUT_NCOLS).into())
        } else if usize::from(point.row) >= INPUT_NROWS {
            Err(format!("row {} exceeds nrows {}", point.row, INPUT_NROWS).into())
        } else {
            Ok(point)
        }
    }
    // Convert to a delta point.
    pub fn delta(&self, other: Point) -> PointDelta {
        PointDelta(self.col.cmp(&other.col), self.row.cmp(&other.row))
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { row: self.row + other.row, col: self.col + other.col }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.row += other.row;
        self.col += other.col;
    }
}

fn apply_delta(coord: Coord, dir: Ordering) -> Coord {
    match dir {
        Ordering::Less => coord.saturating_sub(1),
        Ordering::Greater => coord + 1,
        Ordering::Equal => coord,
    }
}

impl Add<PointDelta> for Point {
    type Output = Self;
    fn add(self, delta: PointDelta) -> Point {
        Point {
            col: apply_delta(self.col, delta.0),
            row: apply_delta(self.row, delta.1),
        }
    }
}

impl AddAssign<PointDelta> for Point {
    fn add_assign(&mut self, delta: PointDelta) {
        self.col = apply_delta(self.col, delta.0);
        self.row = apply_delta(self.row, delta.1);
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { row: self.row - other.row, col: self.col - other.col }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.row -= other.row;
        self.col -= other.col;
    }
}

// Iterator over Points.
struct Points {
    next: Point,
    end: Point,
    delta: PointDelta,
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.next == self.end {
            None
        } else {
            let current = Some(self.next);
            self.next += self.delta;
            current
        }
    }
}

#[derive(Debug, Clone, Copy)] // , PartialEq, PartialOrd, Eq
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
            end: Point::from(points.next().ok_or("expected start point")?.into())?,
        })
    }
    pub fn is_horiz(&self) -> bool { self.start.row == self.end.row }
    pub fn is_vert(&self) -> bool { self.start.col == self.end.col }

    pub fn points(&self) -> Points {
        let delta = self.end.delta(self.start);
        Points {
            next: self.start,
            end: self.end + delta, // one past the end
            delta,
        }
    }
}

fn read_lines(input: &mut dyn BufRead) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines = Vec::<Line>::with_capacity(INPUT_LEN_GUESS);
    for line in input.lines() {
        lines.push(Line::from(line?)?);
    }
    Ok(lines)
}


fn overlaps<I>(lines: I, verbose: bool) -> usize
    where I: IntoIterator<Item=Line>
{
    let mut count = 0;
    let mut graph: [u8; INPUT_GRID] = [0; INPUT_GRID];
    for line in lines {
        if verbose {
            println!("  line {:?}", line);
        }
        for point in line.points().inspect(|p| if verbose { println!("    points {:?}", p); }) {
            let index = usize::from(point.row) * INPUT_NCOLS + usize::from(point.col);
            if graph[index] == 1 {
                count += 1;
                if verbose {
                    println!("  overlap {:?}", point);
                }
            }
            graph[index] = graph[index].saturating_add(1);
        }
    }
    count
}

impl Day for Day5 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let lines = read_lines(input)?;
        Ok((
            PartResult::from(|| {
                overlaps(lines.iter()
                        .filter(|l| l.is_horiz() || l.is_vert())
                        .map(|l| *l),
                    opts.verbose)
            }),
            PartResult::from(|| overlaps(lines.into_iter(), opts.verbose))
        ))
    }
}

