use aoc_2025::Grid;

fn main() {
    let (grid, start) = parser();
    solve(&grid, start);
}

fn parser() -> (Grid<u8>, usize) {
    let data = std::fs::read_to_string("input/07.txt").unwrap();
    let mut lines = data.lines().map(str::as_bytes);
    let start = lines.next().unwrap().iter().position(|ch| *ch == b'S');
    let grid = lines.map(<[u8]>::to_vec).collect();
    let grid = Grid::new(grid);
    (grid, start.unwrap())
}

fn solve(grid: &Grid<u8>, start: usize) {
    let mut beams = vec![0_u64; grid.cols()];
    beams[start] = 1;
    let mut hit_splitters: u64 = 0;
    for row in grid.iter() {
        let mut new_beams = vec![0; grid.cols()];
        for (pos, (obstacle, beam)) in row.iter().zip(beams).enumerate() {
            match *obstacle {
                b'^' => {
                    hit_splitters += 1;
                    new_beams[pos - 1] += beam;
                    new_beams[pos + 1] += beam;
                }
                _ => {
                    new_beams[pos] += beam;
                }
            }
        }
        beams = new_beams;
    }
    println!("{hit_splitters}");
    println!("{}", beams.iter().sum::<u64>());
}
