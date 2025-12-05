fn main() {
    let (ranges, mut numbers) = parse();
    let ranges = merge_ranges(ranges);
    part1(&ranges, &mut numbers);
    part2(&ranges);
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

fn part1(ranges: &[(u64, u64)], numbers: &mut [u64]) {
    let mut ranges = ranges.iter().peekable();
    numbers.sort_unstable();
    let mut good = 0;
    for number in numbers.iter() {
        while let Some((_, to)) = ranges.peek()
            && to < number
        {
            ranges.next();
        }
        let Some((from, to)) = ranges.peek() else {
            break;
        };
        if (from..=to).contains(&number) {
            good += 1;
        }
    }
    println!("{good}");
}

fn part2(ranges: &[(u64, u64)]) {
    let count: u64 = ranges.iter().map(|(from, to)| to - from + 1).sum();
    println!("{count}");
}
