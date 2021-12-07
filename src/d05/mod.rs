use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::time::{Instant, Duration};
use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::{Ordering, Reverse};
use std::mem::swap;

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
        let mut point1 = Point::from(points.next().ok_or("expected start point")?.into())?;
        let mut point2 = Point::from(points.next().ok_or("expected start point")?.into())?;
        if point1 > point2 {
            swap(&mut point1, &mut point2);
        }
        Ok(Line { start: point1, end: point2 })
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
impl Ord for LineSweepEvent { fn cmp(&self, other: &Self) -> Ordering {
        self.point().cmp(&other.point())
    }
}

struct LineSweep {
    // min-heap, visit points in ascending order
    events: BinaryHeap<Reverse<LineSweepEvent>>,
    // lines are ordered by where the intersect the sweep line
    active: BTreeMap<Point, Line>,
    // intersections
    intersections: Vec<Point>,
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

    fn check_intersection(&mut self, line1: Line, line2: Line) -> Option<Line, Point> {
        line1.intersection(line2).and_then(|point| {
            self.events.push(Reverse(LineSweepEvent::Intersection(line1, line2, point)));
            self.intersections.push(point);
        });
    }

    fn start_event(&mut self, line: Line) {
        match self.active.insert(line.start, line) {
            None => (),
            Some(_) => panic!("line squashed by identical line"),
        };
        // Check the nearby element in the status for an intersection.
        self.active.next().and_then(|adjacent| {
            self.check_intersection(line, adjacent).and_then(|point| {
                // swap line1 and line2 in active since they've "crossed"
                self.active.remove_entry(line.start).expect("line1");
                let new_line = Line { start: point, end: line.end };
                assert_gt!(point, new_line, adjacent);
                self.active.insert(point, new_line);
            }).or_else(|| {
                // TODO move line to its end point?
            });
        });
    }

    fn end_event(&mut self, line: Line) {
        self.active.remove(line.end).expect("line not present for removal");
    }

    fn intersection_event(&mut self, line1: Line, line2: Line, point: Point) {
        // TODO
    }

    pub fn intersections(&mut self) -> Vec<Point> {
        while let Some(event) = self.events.pop() {
            match event {
                LineSweepEvent::Start(line) => start_event(*line),
                LineSweepEvent::End(line) => end_event(*line),
                LineSweepEvent::Intersection(line1, line2, point)
                    => intersection_event(*line1, *line2, *point),
            }
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

