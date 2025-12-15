// I dislike joke problems :( I spent too much time trying to solve a NP problem
fn main() {
    let data = std::fs::read_to_string("input/12.txt").unwrap();
    let (data, problems) = data.rsplit_once("\n\n").unwrap();
    let shapes: Vec<_> = data
        .split("\n\n")
        .map(|lines| lines.bytes().filter(|&b| b == b'#').count())
        .collect();
    let mut this = 0;
    for prob in problems.lines() {
        let (grid, nums) = prob.split_once(": ").unwrap();
        let (rows, cols) = grid.split_once('x').unwrap();
        let area = rows.parse::<usize>().unwrap() * cols.parse::<usize>().unwrap();
        let needed: usize = nums
            .split(' ')
            .zip(&shapes)
            .map(|(n, f)| n.parse::<usize>().unwrap() * f)
            .sum();
        if needed < area {
            this += 1;
        }
    }
    println!("{this}");
}
