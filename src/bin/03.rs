fn main() {
    let banks = parse();
    part1(&banks);
    part2(&banks);
}

fn parse() -> Vec<Vec<u64>> {
    let filename = "input/03.txt";
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut banks = Vec::new();
    for line in data.lines() {
        let bank = line
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .map(<_>::from)
            .collect();
        banks.push(bank);
    }
    banks
}

fn max_jolts(values: &[u64], size: usize) -> u64 {
    let mut start = 0;
    let mut value = 0;
    for remaining in (0..size).rev() {
        let next = (start..values.len() - remaining)
            .max_by_key(|idx| (values[*idx], usize::MAX - idx))
            .unwrap();
        value = value * 10 + values[next];
        start = next + 1;
    }
    value
}

fn part1(banks: &[Vec<u64>]) {
    let mut sum = 0;
    for bank in banks {
        sum += max_jolts(&bank, 2);
    }
    println!("{sum}");
}

fn part2(banks: &[Vec<u64>]) {
    let mut sum = 0;
    for bank in banks {
        sum += max_jolts(&bank, 12);
    }
    println!("{sum}");
}
