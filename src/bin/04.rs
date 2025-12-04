use aoc_2025::Grid;
use itertools::Itertools;

fn main() {
    let mut grid = parse();
    part1(&grid);
    part2(&mut grid);
}

fn parse() -> Grid<u8> {
    let filename = "input/04.txt";
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    Grid::new(
        data.lines()
            .map(|line| line.as_bytes().iter().copied().collect())
            .collect(),
    )
}

// search and remove could be done in one go
fn removable_hay(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    (0..grid.rows())
        .cartesian_product(0..grid.cols())
        .filter(|&(r, c)| grid[r][c] == b'@')
        .filter(|idx| {
            let count = grid.neighbors(*idx).filter(|&neigh| neigh == b'@').count();
            count < 4
        })
        .collect()
}

fn part1(grid: &Grid<u8>) {
    let removable = removable_hay(grid);
    println!("{}", removable.len());
}

fn part2(grid: &mut Grid<u8>) {
    let mut counter = 0;
    loop {
        let removable = removable_hay(&grid);
        if removable.is_empty() {
            break;
        }
        counter += removable.len();
        for (row, col) in removable {
            grid[row][col] = b'.';
        }
    }
    println!("{counter}");
}
