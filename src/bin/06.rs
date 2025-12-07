enum Op {
    Add,
    Mul,
}

impl Op {
    const fn neutral(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input/06.txt").expect("Unable to read file");
    part1(&data);
    part2(&data);
}

fn part1(data: &str) {
    let mut lines = data.lines().map(|line| line.split_whitespace());
    let ops = lines.next_back().unwrap();
    let mut nss: Vec<_> = lines.collect();
    let mut total: u64 = 0;
    for op in ops {
        let mut result = if op == "+" { 0 } else { 1 };
        for ns in &mut nss {
            let number: u64 = ns.next().unwrap().parse().unwrap();
            result = if op == "+" {
                result + number
            } else {
                result * number
            };
        }
        total += result;
    }
    println!("{total}");
}

fn part2(data: &str) {
    let mut lines = data.lines().map(|line| line.chars());
    let ops = lines.next_back().unwrap();
    let mut nss: Vec<_> = lines.collect();

    let mut total = 0;
    let mut curr_op = Op::Add;
    let mut result = 0;
    for potential_op in ops {
        match potential_op {
            '+' | '*' => {
                total += result;
                curr_op = match potential_op {
                    '+' => Op::Add,
                    '*' => Op::Mul,
                    _ => unreachable!(),
                };
                result = curr_op.neutral();
            }
            ' ' => {}
            _ => unreachable!(),
        }
        let mut number = String::new();
        for ns in &mut nss {
            if let Some(digit @ '1'..='9') = ns.next() {
                number.push(digit);
            }
        }
        let number: u64 = number.parse().unwrap_or_else(|_| curr_op.neutral());
        result = match curr_op {
            Op::Add => result + number,
            Op::Mul => result * number,
        }
    }
    total += result;
    println!("{total}");
}
