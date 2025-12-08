use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u64, u64, u64);

impl Point {
    const fn distance(&self, other: &Self) -> u64 {
        let d0 = self.0.abs_diff(other.0).pow(2);
        let d1 = self.1.abs_diff(other.1).pow(2);
        let d2 = self.2.abs_diff(other.2).pow(2);
        d0 + d1 + d2
    }
}

#[derive(Debug)]
struct UnionFind {
    find: Vec<usize>,
    union: HashMap<usize, Vec<usize>>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let find = (0..size).collect();
        let union = (0..size).map(|i| (i, vec![i])).collect();
        Self { find, union }
    }

    fn find(&self, index: usize) -> usize {
        self.find[index]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return;
        }
        let (a, b) = if self.union[&a].len() < self.union[&b].len() {
            (b, a)
        } else {
            (a, b)
        };
        let bs = self.union.remove(&b).unwrap();
        for b in &bs {
            self.find[*b] = a;
        }
        self.union.get_mut(&a).unwrap().extend(bs);
    }

    fn finished(&self) -> bool {
        self.find
            .first()
            .is_none_or(|idx| self.union[idx].len() == self.find.len())
    }

    fn groups(&self) -> impl Iterator<Item = &Vec<usize>> {
        self.union.values()
    }
}

fn main() {
    let points = parser();
    solve(&points);
}

fn parser() -> Vec<Point> {
    let data = std::fs::read_to_string("input/08.txt").unwrap();
    let mut points = Vec::new();
    for line in data.lines() {
        let coords: [u64; 3] = line
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect_array()
            .unwrap();
        points.push(Point(coords[0], coords[1], coords[2]));
    }
    points
}

fn solve(points: &[Point]) {
    let n = points.len();

    let mut distances = (0..n)
        .tuple_combinations()
        .map(|(i, j)| (points[i].distance(&points[j]), i, j))
        .collect_vec();
    distances.sort_unstable();
    let mut uf = UnionFind::new(points.len());
    for (idx, (_, p1, p2)) in distances.into_iter().enumerate() {
        uf.union(p1, p2);
        if idx + 1 == points.len() {
            let circuits: usize = uf.groups().map(<Vec<_>>::len).k_largest(3).product();
            println!("{circuits}");
        }
        if uf.finished() {
            println!("{}", points[p1].0 * points[p2].0);
            break;
        }
    }
}
