use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::{Instant, Duration};
use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::{Ordering, Reverse};

pub struct Day5;

const INPUT_LEN_GUESS: usize = 500;
type Coord = u16;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
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

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.col),
            cmp => cmp
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
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

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        // Opposite dimensions from event ordering of points;
        // order by col first, then by row
        match self.start.col.cmp(&other.start.col) {
            Ordering::Equal => self.start.row.cmp(&other.start.row),
            cmp => cmp
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
enum LineSweepEvent {
    Start(Line),
    End(Line),
    Intersection(Line, Line, Point),
}

impl LineSweepEvent {
    fn point(&self) -> Point {
        match self {
            LineSweepEvent::Start(l) => l.start,
            LineSweepEvent::End(l) => l.end,
            LineSweepEvent::Intersection(_, _, p) => *p,
        }
    }
}

// Visit points ordered by row first, then column
impl Ord for LineSweepEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.point().cmp(&other.point())
    }
}

struct LineSweep {
    // min-heap, visit points in ascending order
    events: BinaryHeap<Reverse<LineSweepEvent>>,
    // lines are ordered by where the intersect the sweep line
    active: BTreeMap<Point, Line>,
}

impl LineSweep {
    pub fn from<'a, I>(lines: I) -> LineSweep
        where I: IntoIterator<Item=&'a Line>
    {
        let mut sweep = LineSweep { events: BinaryHeap::new(), active: BTreeMap::new() };
        for line in lines {
            sweep.events.push(Reverse(LineSweepEvent::Start(*line)));
            sweep.events.push(Reverse(LineSweepEvent::End(*line)));
        }
        sweep
    }

    pub fn intersections(&self) -> Vec<Point> {
        /* TODO
        while let Some(event) = self.events.pop() {
            match event {
            }
        }
        */
        Vec::<Point>::new()
    }
}

fn read_lines(input: &mut dyn BufRead) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines = Vec::<Line>::with_capacity(INPUT_LEN_GUESS);
    for line in input.lines() {
        lines.push(Line::from(line?)?);
    }
    Ok(lines)
}

impl Day for Day5 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, _opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let time = Instant::now();
        let lines = read_lines(input)?;
        let cardinal_lines = lines.iter().filter(|l| l.is_horiz() || l.is_vert());
        let part1 = LineSweep::from(cardinal_lines).intersections().len();
        let time = time.elapsed();
        Ok((PartResult { answer: part1.to_string(), time },
            PartResult { answer: "unimplemented".into(), time: Duration::new(0, 0) }))
    }
}

