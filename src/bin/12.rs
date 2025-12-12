use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_2025::Grid;
use itertools::{Itertools, iproduct};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Pixel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Pixel {
    fn new(value: usize) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            _ => panic!(),
        }
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Pixel::Zero => "A",
            Pixel::One => "B",
            Pixel::Two => "C",
            Pixel::Three => "D",
            Pixel::Four => "E",
            Pixel::Five => "F",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Shape([[bool; 3]; 3], Pixel);

impl Shape {
    fn new<'a>(rows: impl Iterator<Item = &'a str>, pixel: Pixel) -> Self {
        let data = rows
            .map(|row| {
                row.as_bytes()
                    .iter()
                    .map(|byte| *byte == b'#')
                    .collect_array()
                    .unwrap()
            })
            .collect_array()
            .unwrap();
        Self(data, pixel)
    }

    fn rotate90(&self) -> Self {
        let mut copy = self.0;
        for (r, c) in iproduct!(0..3, 0..3) {
            copy[r][c] = self.0[2 - c][r];
        }
        Self(copy, self.1)
    }

    const fn mirror(&self) -> Self {
        let mut copy = self.0;
        copy[0].swap(0, 2);
        copy[1].swap(0, 2);
        copy[2].swap(0, 2);
        Self(copy, self.1)
    }

    const fn pixel_type(&self) -> Pixel {
        self.1
    }

    fn insert(&self, grid: &mut Grid<bool>, row: usize, col: usize) -> bool {
        for (r, c) in iproduct!(0..3, 0..3) {
            if grid[row + r][col + c] && self.0[r][c] {
                return false;
            }
        }
        for (r, c) in iproduct!(0..3, 0..3) {
            if self.0[r][c] {
                grid[row + r][col + c] = true;
            }
        }
        true
    }

    fn remove(&self, grid: &mut Grid<bool>, row: usize, col: usize) {
        for (r, c) in iproduct!(0..3, 0..3) {
            if self.0[r][c] {
                grid[row + r][col + c] = false;
            }
        }
    }
}

#[derive(Debug)]
struct Problem {
    grid: Grid<bool>,
    goal: [usize; 6],
}

fn main() {
    let (shapes, problems) = parse();
    for Problem { mut grid, mut goal } in problems {
        let mut cache = HashMap::new();
        let ans = solve(&shapes, &mut grid, &mut cache, &mut goal);
        println!("{ans}");
    }
}

fn parse() -> (HashSet<Shape>, Vec<Problem>) {
    let data = std::fs::read_to_string("input/12.txt").unwrap();
    let (shapes, rest) = data.rsplit_once("\n\n").unwrap();
    let shapes = shapes.split("\n\n").map(|line| line.split('\n').skip(1));
    let mut collection = HashSet::new();
    for (id, shape) in shapes.enumerate() {
        let shape = Shape::new(shape, Pixel::new(id));
        collection.insert(shape);
    }
    for entry in collection.clone() {
        let mirror = entry.mirror();
        collection.insert(mirror);
        collection.insert(mirror.rotate90());
        collection.insert(mirror.rotate90().rotate90());
        collection.insert(mirror.rotate90().rotate90().rotate90());
        collection.insert(entry.rotate90());
        collection.insert(entry.rotate90().rotate90());
        collection.insert(entry.rotate90().rotate90().rotate90());
    }

    let mut problems = Vec::new();

    for problem in rest.lines() {
        let (grid, counts) = problem.split_once(": ").unwrap();
        let (nrows, ncols) = grid.split_once('x').unwrap();
        let nrows = nrows.parse().unwrap();
        let ncols = ncols.parse().unwrap();
        let count = counts
            .split(' ')
            .map(|c| c.parse::<usize>().unwrap())
            .collect_array()
            .unwrap();
        let grid = Grid::new(vec![vec![false; ncols]; nrows]);
        problems.push(Problem { grid, goal: count });
    }
    (collection, problems)
}

fn solve(
    shapes: &HashSet<Shape>,
    grid: &mut Grid<bool>,
    cache: &mut HashMap<Grid<bool>, bool>,
    goal: &mut [usize; 6],
) -> bool {
    // println!("{:?}", grid);
    // println!("{:?}", cache.len());
    if let Some(ans) = cache.get(grid) {
        return *ans;
    }
    if goal.iter().all(|val| *val == 0) {
        cache.insert(grid.clone(), true);
        return true;
    }
    // iterate through all possible rows and all possible shapes
    for shape in shapes {
        let pixel_idx = shape.pixel_type() as usize;
        if goal[pixel_idx] == 0 {
            continue;
        }
        for (r, c) in (0..grid.rows() - 2).cartesian_product(0..grid.cols() - 2) {
            if !((r == 0 || grid[r-1][c]) && (c == 0 || grid[r][c-1])) {
                continue;
            }
            if shape.insert(grid, r, c) {
                goal[pixel_idx] -= 1;
                let solvable = solve(shapes, grid, cache, goal);
                shape.remove(grid, r, c);
                goal[pixel_idx] += 1;
                if solvable {
                    cache.insert(grid.clone(), true);
                    return true;
                }
            }
        }
    }
    cache.insert(grid.clone(), false);
    false
}
