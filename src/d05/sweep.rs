use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::{Ordering, Reverse};
use std::mem::swap;
use std::ops::{RangeFrom, RangeTo};
use std::convert::From;

impl Point {
    pub fn x() -> Coord { col }
    pub fn y() -> Coord { row }
    // Invert the x and y coordinates.
    pub fn inverse(&self) -> Point {
        Point { col: self.row, row: self.col }
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


enum Intersection {
    Intersect(Point),
    Overlap(Line),
}

impl Line {
    pub fn xmin(&self) -> Point {
        if self.start.x() < self.end.x() { self.start } else { self.end }
    }
    pub fn xmax(&self) -> Point {
        if self.start.x() < self.end.x() { self.end } else { self.start }
    }
    pub fn ymin(&self) -> Point { self.start }
    pub fn ymax(&self) -> Point { self.end }
    // Invert the x and y coordinates of the line.
    // Notable turns horizontal lines vertical and vice-versa.
    //   0123                 0123
    //       p                    q
    // 0    /               0    /
    // 1   /  => inverse => 1   /
    // 2  /                 2  /
    // 3 /                  3 /
    //  q                    p
    //   0123                 0123
    // 0                    0   |p
    // 1 p  q => inverse => 1   |
    // 2 ----               2   |
    // 3                    3   |q
    pub fn inverse(&self) -> Line {
        Line { start: self.start.inverse(), end: self.end.inverse() }
    }

    // Ok(0) for horizontal,
    // 1 or -1 for diagonal,
    // None for vertical.
    pub fn slope(&self) -> Option<Coord> {
        if self.is_vert() {
            None // vertical
        } else {
            if self.end.y() == self.start.y() {
                return Ok(0); // horizontal
            }
            // diagonal
            let sgny = self.end.y() - self.start.y();
            let sgnx = self.end.x() - self.start.x();
            if (sgny > 0) == (sgnx > 0) {
                Ok(1)
            } else {
                Ok(-1)
            }
        }
    }

    pub fn intercept(&self) -> Option<Coord> {
        if self.is_vert() {
            None
        } else if self.is_horiz() {
            Ok(self.start.y())
        } else {
            Ok(self.start.y() - self.start.x() * self.slope())
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Intersection> {
        let (p, q) = (self.start, self.end);
        let (r, s) = (other.start, other.end);
        // Recognize all cases of line intersection, after identifying that each line is
        // either horizontal, vertical, or diagonal with slope either 1 or -1.

        // 1. Horizontal lines.
        if self.is_horiz() && other.is_horiz() {
            let y = p.y();
            // Lines must have the same Y to overlap.
            // According to the sort criteria, Px < Qx and Rx < Sx.
            //  P--------R======S----Q
            if y != r.y() || r.x() > q.x() || s.x() < p.x() {
                return None;
            }
            let xmin = max(p.x(), r.x());
            let xmax = min(s.x(), q.x());
            if xmin == xmax {
                return Ok(Intersection::Intersect(Point { col: xmin, row: y }));
            }
            return Ok(Intersection::Overlap(Line {
                start: Point { col: xmin, row: y },
                end: Point { col: xmax, row: y }
            }));
        }

        // 2. Vertical lines.
        if self.is_vert() && other.is_vert() {
            // Handle exactly as horizontal lines with flipped x and y coordinates.
            return match self.inverse().intersection(other.inverse()) {
                Ok(Intersection::Intersect(p)) => Ok(p.inverse()),
                Ok(Intersection::Overlap(l)) => Ok(l.inverse()),
                None => None,
            };
        }

        // 3a. Self horizontal, other vertical.
        if self.is_horiz() && other.is_vert() {
            // According to the sort criteria, Ry < Sy and Px < Qx.
            //      R              R
            //      |              |
            // P----+--Q   P-----Q |
            //      |              |
            //      S              S
            if p.x() <= r.x() && r.x() <= q.x() && r.y() <= p.y() && s.y() >= p.y() {
                return Ok(Intersection::Intersect(Point { col: r.col, row: p.row }));
            }
            return None;
        }

        // 3b. Self vertical, other horizontal -- handle using 3a by swapping args.
        if self.is_vert() && other.is_horiz() {
            return other.intersection(self);
        }

        // 4a. Self horizontal, other diagonal.
        if self.is_horiz() {
            if r.y() <= p.y() && s.y() >= p.y() {
                // Solving for PQ = RS yields x = (Py - y0) / m, where
                //   m = (Sy - Ry) / (Sx - Rx)    (slope)
                //   y0 = Sy - Sx * m             (intercept);
                // This reduces to
                //   x = (Py - Sy) / m + Sx
                // And we have an intersection when min{Px,Qx} <= x <= max{Px,Qx}.
                // Note that the slope is always 1 or -1,
                // so multiplying by m and dividing by m are equivalent.
                let m = other.slope().expect("[4a,m] other must be diagonal");
                let x = m * (p.y() - s.y()) + s.x();
                if self.xmin() <= x && x <= self.xmax() {
                    let y = m * x + other.intercept().expect("[4a,y0] other must be diagonal");
                    return Ok(Intersection::Intersect(Point { col: x, row: y }));
                }
            }
            return None;
        }

        // 4b. Other horizontal, self diagonal -- handle using 4a by swapping args.
        if other.is_horiz() {
            return other.intersection(self);
        }

        // 5a. Self vertical, other diagonal -- handle as 4 with flipped x and y coordinates.
        if self.is_vert() {
            return match self.inverse().intersection(other.inverse()) {
                Ok(Intersection::Intersect(p)) => Ok(p.inverse()),
                Ok(Intersection::Overlap(l)) => Ok(l.inverse()),
                None => None,
            };
        }

        // 5b. Other vertical, self diagonal -- handle using 5a by swapping args.
        if other.is_vert() {
            return other.intersection(self);
        }

        // 6. Both diagonal.

        // 6a. Parallel diagonals.
        if self.slope() == other.slope() {
            // P          By the sorting criteria, Py < Qy and Ry < Sy.
            //  \         Parallel diagonals overlap when the lines have the same y-intercept.
            //   \
            //   XX R
            //    \\
            //     \\
            //      XX S
            //        \
            //         Q
            if self.intercept() == other.intercept()
                && !(other.xmax() < self.xmin() || other.xmin() > self.xmax()) {
                // start = min{p, r}
                let start = if r.y() < p.y() {
                    Point { col: r.x(), r.y() }
                } else {
                    Point { col: p.x(), p.y() }
                };
                // end = max{q, s}
                let end = if s.y() > q.y() {
                    Point { col: s.x(), s.y() }
                } else {
                    Point { col: q.x(), q.y() }
                };
                return if start == end {
                    Ok(Intersection::Intersect(start))
                } else {
                    Ok(Intersection::Overlap(Line { start, end }))
                };
            }
            return None;
        }

        // 6b. Orthogonal diagonals: self.slope() === -1 * other.slope().
        let m = self.slope().expect("[6b,m] self must be diagonal");
        // Solving for PQ = RS where m = self.slope() and m' = other.slope() and m == -m' gives:
        //   x = [ 1/m * (Py - Ry) - Px - Rx ] / 2
        // Remember m is always 1 or -1 so multiplying and dividing by m are equivalent.
        // Furthermore, m' (other.slope()) is always -m.
        let x = (m * (p.y() - r.y()) - p.x() - r.x()) / 2;
        if self.xmin() <= x && x <= self.xmax() {
            let y = m * x + self.intercept().expect("[6b,y0] self must be diagonal");
            return Ok(Intersection::Intersect(Point { col: x, row: y }));
        }

        None
    }
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
    Start(&Line),
    End(&Line),
    Intersection(&mut Line, &mut Line, Point),
}

impl LineSweepEvent {
    fn point(&self) -> Point {
        match self {
            LineSweepEvent::Start(l) => l.start,
            LineSweepEvent::End(l) => l.end,
            LineSweepEvent::Intersection(_, _, p) => *p,
        }
    }
    // If points are equal, sort by opposite end
    fn backup(&self) -> Point {
        match self {
            LineSweepEvent::Start(l) => l.end,
            LineSweepEvent::End(l) => l.start,
            // Classically we sort around intersection points by the line's angle.
            LineSweepEvent::Intersection(_, l2, _) => l2.start,
        }
    }
}

// Visit points ordered by row first, then column
impl Ord for LineSweepEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.point().cmp(&other.point()) {
            Ordering::Equal => self.backup().cmp(&other.backup()),
            cmp => cmp,
        }
    }
}

struct ActiveLine {
    line: Line,
}

struct LineSweep {
    // storage for lines
    lines: Vec<Line>,
    // min-heap, visit points in ascending order
    events: BinaryHeap<Reverse<LineSweepEvent>>,
    // ref to lines ordered by where they intersect the sweep line
    // TODO provide total order for multiple lines at one intersection point
    //      (classically done sorting by angle... we could do that too)
    active: BTreeMap<&mut Line>,
    // intersections
    intersections: Vec<Point>,
}

impl LineSweep {
    pub fn from<'a, I>(lines: I) -> LineSweep
        where I: IntoIterator<Item=&'a Line>
    {
        let mut sweep = LineSweep { events: BinaryHeap::new(), active: BTreeMap::new() };
        self.lines = lines.into_iter().collect();
        for line in self.lines {
            sweep.events.push(Reverse(LineSweepEvent::Start(line)));
            sweep.events.push(Reverse(LineSweepEvent::End(line)));
        }
        sweep
    }

    fn check_intersection(&mut self, line1: &mut Line, line2: &mut Line) {
        line1.intersection(line2).and_then(|intersection| {
            match intersection {
                Intersection::Intersect(point) => {
                    self.intersections.push(point);
                    self.events.push(Reverse(LineSweepEvent::Intersection(line1, line2, point)));
                },
                // For overlaps, we want to record the intersection points but not visit them
                // later as events, since they won't add any new information. The overlap will
                // already contain at least one Start or End event for at least one of the lines.
                Intersection::Overlap(line) => {
                    self.intersections.append(line.points().collect());
                },
            }
        });
    }

    fn check_adjacent(&mut self, line: &Line) {
        // Check the adjacent lines in the status for intersections.
        self.active.range(RangeFrom { start: &line })
            .next().expect("range doesn't contain start")
            .next().and_then(|radjacent| self.check_intersection(line, radjacent));
        self.active.range(RangeTo { end: &line }).rev()
            .next().and_then(|ladjacent| self.check_intersection(ladjacent, line));
    }

    fn update_active(&mut self, line: &mut Line, point: Point) {
        // swap line1 and line2 in active since they've "crossed" at the intersection
        let _removed = self.active.remove(line);
        assert_eq!(_removed, true);
        line.start = point;
        self.active.insert(line);
    }

    fn start_event(&mut self, line: Line) {
        // TODO -- anything else here?
        self.check_adjacent(line);
    }

    fn end_event(&mut self, line: Line) {
        // TODO this will not find the line correctly in the status, since status is currently
        // sorted by the prior point at which it intersected the sweep line
        self.active.remove(line.end).expect("line not present for removal");
    }

    fn intersection_event(&mut self, line1: &mut Line, line2: &mut Line, point: Point) {
        // TODO -- anything else here?
        self.update_active(line1, point);
        self.update_active(line1, point);
        self.check_adjacent(new_line);
    }

    pub fn intersections(&mut self) -> Vec<Point> {
        while let Some(event) = self.events.pop() {
            match event {
                LineSweepEvent::Start(line) => start_event(*line),
                LineSweepEvent::End(line) => end_event(*line),
                LineSweepEvent::Intersection(line1, line2, point)
                    => intersection_event(line1, line2, *point),
            }
        }
    }
}

