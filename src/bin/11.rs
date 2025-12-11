use std::collections::{HashMap, HashSet};

// for some unnecessary complications reason lets make everything &str
#[derive(Debug)]
struct Graph<'graph> {
    edges: HashMap<&'graph str, Vec<&'graph str>>,
}

impl<'graph> Graph<'graph> {
    fn new() -> Self {
        let edges = HashMap::new();
        Self { edges }
    }

    fn add_edge(&mut self, from: &'graph str, to: &'graph str) {
        // build the residual graph directly
        self.edges.entry(from).or_default().push(to);
    }

    fn dfs_exit_times(&self, node: &'graph str) -> Vec<&'graph str> {
        let mut stack = vec![(node, false)];
        let mut visited = HashSet::from([node]);
        let mut exit_time = Vec::new();
        while let Some((node, finished)) = stack.pop() {
            if finished {
                exit_time.push(node);
                continue;
            }
            stack.push((node, true));
            let Some(neighs) = self.edges.get(node) else {
                continue;
            };
            for neigh in neighs {
                if !visited.contains(neigh) {
                    stack.push((neigh, false));
                    visited.insert(neigh);
                }
            }
        }
        exit_time.reverse();
        exit_time
    }

    fn solve(&self, from: &str, to: &str) -> (u64, u64) {
        let times = self.dfs_exit_times(from);
        // {visited neither, visited fft, visited dac, visited both}
        let mut possibilities = HashMap::from([(&from, (1, 0, 0, 0))]);
        for node in times {
            let Some(neighs) = self.edges.get(node) else {
                continue;
            };
            for neigh in neighs {
                let pnode = possibilities[&node];
                let pneigh = possibilities.entry(neigh).or_insert((0, 0, 0, 0));
                match node {
                    "fft" => {
                        pneigh.1 += pnode.0;
                        pneigh.3 += pnode.2;
                    }
                    "dac" => {
                        pneigh.2 += pnode.0;
                        pneigh.3 += pnode.1;
                    }
                    _ => {
                        pneigh.0 += pnode.0;
                        pneigh.1 += pnode.1;
                        pneigh.2 += pnode.2;
                        pneigh.3 += pnode.3;
                    }
                }
            }
        }
        let out = possibilities[&to];
        let part1 = out.0 + out.1 + out.2 + out.3;
        let part2 = out.3;
        (part1, part2)
    }
}

fn main() {
    let data = std::fs::read_to_string("input/11.txt").unwrap();
    let graph = {
        let mut graph = Graph::new();
        for line in data.lines() {
            let (from, tos) = line.split_once(": ").unwrap();
            for to in tos.split_ascii_whitespace() {
                graph.add_edge(from, to);
            }
        }
        graph
    };
    println!("{}", graph.solve("you", "out").0);
    println!("{}", graph.solve("svr", "out").1);
}
