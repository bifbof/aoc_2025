// use std::collections::HashSet;

// use good_lp::{Solution, SolverModel, variable};

// #[derive(Debug)]
// struct Problem {
//     lights: Vec<bool>,
//     joltages: Vec<u16>,
//     buttons: Vec<Vec<usize>>,
// }

// fn main() {
//     let data = parse();
//     let p1: u64 = data.iter().map(part1).sum();
//     println!("{p1}");
//     let p2: f64 = data.iter().map(part2).sum();
//     println!("{p2}");
// }

// fn parse() -> Vec<Problem> {
//     let data = std::fs::read_to_string("input/10.txt").unwrap();
//     let mut problems = Vec::new();
//     for line in data.lines() {
//         let (lights, rest) = line.split_once(' ').unwrap();
//         let (buttons, joltages) = rest.rsplit_once(' ').unwrap();
//         let lights = lights.as_bytes()[1..lights.len() - 1]
//             .iter()
//             .map(|e| *e == b'#')
//             .collect();
//         let joltages = joltages[1..joltages.len() - 1]
//             .split(',')
//             .map(|n| n.parse().unwrap())
//             .collect();
//         let buttons = buttons
//             .split_whitespace()
//             .map(|button| {
//                 let button = &button[1..button.len() - 1];
//                 button.split(',').map(|num| num.parse().unwrap()).collect()
//             })
//             .collect();

//         problems.push(Problem {
//             lights,
//             joltages,
//             buttons,
//         });
//     }
//     problems
// }

// fn part1(p: &Problem) -> u64 {
//     let Problem {
//         lights: goal,
//         joltages: _,
//         buttons,
//     } = p;
//     // with small enough problem (2^n states) we can still simulate this
//     let mut steps = 0;
//     let mut lights = HashSet::from([vec![false; goal.len()]]);
//     while !lights.contains(goal) {
//         steps += 1;
//         let mut new_lights = HashSet::new();
//         for light in lights {
//             for button in buttons {
//                 let mut new_light = light.clone();
//                 for &idx in button {
//                     new_light[idx] = !new_light[idx];
//                 }
//                 new_lights.insert(new_light);
//             }
//         }
//         lights = new_lights;
//     }
//     steps
// }

// fn part2(p: &Problem) -> f64 {
//     // I really tried to solve that myself, but no in the end it stayed strong on the NP side.
//     let Problem {
//         lights: _,
//         joltages,
//         buttons,
//     } = p;

//     let mut var_collector = good_lp::ProblemVariables::new();
//     let vars: Vec<_> = (0..buttons.len())
//         .map(|_| var_collector.add(variable().integer().min(0)))
//         .collect();

//     let total_sum = vars.iter().sum::<good_lp::Expression>();
//     let mut problem = good_lp::highs(var_collector.minimise(total_sum));
//     let mut constraints: Vec<good_lp::Expression> = vec![0.into(); joltages.len()];
//     for i in 0..buttons.len() {
//         for &x in &buttons[i] {
//             constraints[x] += vars[i];
//         }
//     }
//     for (e, &j) in constraints.into_iter().zip(joltages) {
//         problem.add_constraint(e.eq(j as f64));
//     }
//     let solution = problem.solve().unwrap();
//     let mut total = 0.0;
//     for var in &vars {
//         total += solution.value(*var);
//     }
//     total
// }

// commented out to avoid building `good_lp` for all the other bin that don't need them
fn main() {

}
