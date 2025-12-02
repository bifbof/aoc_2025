use itertools::Itertools;

fn main() {
    let data = parse();
    part1(&data);
    part2(&data);
}

fn parse() -> Vec<(u64, u64)> {
    let filename = "input/02.txt";
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut ranges = Vec::new();
    for range in data.split(',') {
        let (from, to) = range.split_once('-').unwrap();
        ranges.push((from.parse().unwrap(), to.parse().unwrap()));
    }
    ranges
}

fn part1(ranges: &[(u64, u64)]) {
    let sum: u64 = ranges
        .iter()
        .flat_map(|&(from, to)| from..=to)
        .filter(|id| {
            let id = id.to_string();
            let mid = id.len() / 2;
            id[..mid] == id[mid..]
        })
        .sum();
    println!("{sum}");
}

fn repeated_digits(value: u64) -> bool {
    let value = value.to_string();
    let value = value.as_bytes();
    for chunk_size in 1..=(value.len() / 2) {
        if value.len() % chunk_size != 0 {
            continue;
        }
        if value.chunks(chunk_size).all_equal() {
            return true;
        }
    }
    false
}

fn part2(ranges: &[(u64, u64)]) {
    let sum: u64 = ranges
        .iter()
        .flat_map(|&(from, to)| from..=to)
        .filter(|id| repeated_digits(*id))
        .sum();
    println!("{sum}");
}
