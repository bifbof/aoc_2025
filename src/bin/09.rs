use itertools::Itertools as _;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u32, u32);

struct Line(Point, Point);

impl Line {
    fn new(p0: Point, p1: Point) -> Self {
        // a bit unnecessary but makes crosses_within simpler
        if p0 < p1 { Self(p0, p1) } else { Self(p1, p0) }
    }

    fn crosses_within(&self, other: &Self) -> bool {
        let Self(Point(x00, y00), Point(x01, y01)) = *self;
        let Self(Point(x10, y10), Point(x11, y11)) = *other;
        let d0 = (x00 + 1..x01).contains(&x10) && (y10 + 1..y11).contains(&y00);
        let d1 = (x10 + 1..x11).contains(&x00) && (y00 + 1..y01).contains(&y10);
        d0 || d1
    }

    fn contains(&self, point: Point) -> bool {
        let Self(Point(x0, y0), Point(x1, y1)) = *self;
        (x0..=x1).contains(&point.0) && (y0..=y1).contains(&point.1)
    }
}

struct Polygon {
    lines: Vec<Line>,
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        let lines = points
            .iter()
            .circular_tuple_windows()
            .map(|(p0, p1)| Line::new(*p0, *p1))
            .collect();
        Self { lines, points }
    }

    fn contains(&self, point: Point) -> bool {
        // check if point is on any line
        for line in &self.lines {
            if line.contains(point) {
                return true;
            }
        }
        // within ==> all quadrants uneven, outside ==> all quadrants even
        // The proof idea is the following: we can split up the form into rectangles
        // the rectangle we are in creates the unevenness, all other rectangles only add even amount to quadrants
        let quadrant = self
            .points
            .iter()
            .filter(|p| p.0 > point.0 && p.1 > point.1)
            .count();
        quadrant % 2 == 1
    }
}

fn main() {
    let points = parse();
    solve(&points);
}

fn parse() -> Vec<Point> {
    let data = std::fs::read_to_string("input/09.txt").unwrap();
    data.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| Point(a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn solve(points: &[Point]) {
    let polygon = Polygon::new(points.to_vec());
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for (&p0, &p1) in points.iter().tuple_combinations() {
        let area = u64::from(p0.0.abs_diff(p1.0) + 1) * u64::from(p0.1.abs_diff(p1.1) + 1);
        if area < part2 {
            continue;
        }
        part1 = part1.max(area);

        let p01 = Point(p0.0, p1.1);
        let p10 = Point(p1.0, p0.1);
        if !polygon.contains(p01) || !polygon.contains(p10) {
            continue;
        }
        let rect = Polygon::new(vec![p0, p01, p1, p10]);
        // as all edges have length > 1 we cannot have empty holes
        // thus if we cross a line (without corners) we get always get a hole
        let crosses = rect
            .lines
            .iter()
            .cartesian_product(&polygon.lines)
            .any(|(e0, e1)| e0.crosses_within(e1));
        if crosses {
            continue;
        }
        part2 = part2.max(area);
    }
    println!("{part1}");
    println!("{part2}");
}
