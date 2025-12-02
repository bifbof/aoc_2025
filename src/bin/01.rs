fn main() {
    let data = parse();
    part(data);
}

fn parse() -> Vec<i64> {
    let filename = "input/01.txt";
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut rotations = Vec::new();
    for line in data.lines() {
        let (dir, num) = line.split_at(1);
        let num: i64 = num.parse().unwrap();
        let rot = match dir {
            "R" => num,
            "L" => -num,
            _ => unreachable!(),
        };
        rotations.push(rot);
    }
    rotations
}

fn part(data: Vec<i64>) {
    let mut dial: i64 = 50;
    let mut counter = 0;
    let mut total = 0;
    for rotation in data {
        total += (rotation / 100).unsigned_abs();
        let rotation = rotation % 100;
        let new = dial + rotation;
        if dial != 0 && !(1..100).contains(&new) {
            total += 1;
        }
        dial = new.rem_euclid(100);
        if dial == 0 {
            counter += 1;
        }
    }

    println!("{counter}");
    println!("{total}");
}
