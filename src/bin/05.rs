fn main() {
    let (ranges, mut numbers) = parse();
    let ranges = merge_ranges(ranges);
    numbers.sort_unstable();
    let answer1 = part1(&ranges, numbers);
    let answer2 = part2(&ranges);
    println!("{answer1}");
    println!("{answer2}");
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable();
    let mut merged = Vec::new();
    for (from, to) in ranges {
        match merged.last_mut() {
            Some((_, prev_to)) if from <= *prev_to => *prev_to = to.max(*prev_to),
            _ => merged.push((from, to)),
        }
    }
    merged
}

fn parse() -> (Vec<(u64, u64)>, Vec<u64>) {
    let filename = "input/05.txt";
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    let (ranges, numbers) = data.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        })
        .collect();
    let numbers = numbers.lines().map(|line| line.parse().unwrap()).collect();

    (ranges, numbers)
}

fn part1(ranges: &[(u64, u64)], numbers: Vec<u64>) -> u64 {
    let mut ranges = ranges.iter().peekable();
    let mut good = 0;
    for number in numbers {
        let (from, to) = loop {
            match ranges.peek() {
                None => return good,
                Some((_, to)) if *to < number => {
                    ranges.next();
                }
                Some((from, to)) => break (from, to),
            }
        };
        if *from <= number && number <= *to {
            good += 1;
        }
    }
    good
}

fn part2(ranges: &[(u64, u64)]) -> u64 {
    ranges.iter().map(|(from, to)| to - from + 1).sum()
}
