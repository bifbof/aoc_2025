use std::{cmp::Reverse, collections::HashMap, fmt::Debug, str::FromStr};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct NodeId(u16);

impl Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rest = self.0;
        let c = u8::try_from(rest % 26).unwrap() + b'a';
        let rest = rest / 26;
        let b = u8::try_from(rest % 26).unwrap() + b'a';
        let rest = rest / 26;
        let a = u8::try_from(rest).unwrap() + b'a';
        let bytes = [a, b, c];
        let s = str::from_utf8(&bytes).unwrap();
        f.write_str(s)
    }
}

impl FromStr for NodeId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        if let [a, b, c] = s
            && a.is_ascii_lowercase()
            && b.is_ascii_lowercase()
            && c.is_ascii_lowercase()
        {
            let a: u16 = (a - b'a').into();
            let b: u16 = (b - b'a').into();
            let c: u16 = (c - b'a').into();
            Ok(Self(((a * 26) + b) * 26 + c))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<NodeId, HashMap<NodeId, u64>>,
}

impl Graph {
    fn new() -> Self {
        let edges = HashMap::new();
        Self { edges }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        // super bad idea in production code ;)
        // but for us it makes computing the flow easier
        if to == "you" {
            return;
        }
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        // build the residual graph directly
        self.edges.entry(from).or_default().insert(to, 1);
        self.edges.entry(to).or_default().insert(from, 0);
    }

    fn compute_flow(&mut self) -> u64 {
        let from: NodeId = "you".parse().unwrap();
        let to: NodeId = "out".parse().unwrap();
        loop {
            // find a path with remaining capacity
            let mut stack = vec![from];
            let mut visited = HashMap::from([(from, from)]);
            while let Some(node) = stack.pop() {
                for (neigh, _) in self.edges[&node].iter().filter(|(_, c)| **c > 0) {
                    if !visited.contains_key(neigh) {
                        visited.insert(*neigh, node);
                        stack.push(*neigh);
                    }
                }
            }
            if !visited.contains_key(&to) {
                break;
            }
            let mut path = vec![to];
            let mut node = to;
            while node != from {
                node = visited[&node];
                path.push(node);
            }
            println!("path {path:?}");
            for (u, v) in path.iter().tuple_windows() {
                println!("{u:?}, {v:?}");
                *self.edges.get_mut(u).unwrap().get_mut(v).unwrap() += 1;
                *self.edges.get_mut(v).unwrap().get_mut(u).unwrap() -= 1;
            }
        }
        self.edges[&from].values().map(|v| 1 - v).sum()
    }

    fn dfs_exit_times(&self, node: NodeId, time: &mut u64, outtime: &mut HashMap<NodeId, u64>) {
        println!("{:?}", node);
        if outtime.contains_key(&node) {
            return;
        }
        println!("{:?}", self.edges[&node]);
        for (&neigh, _) in self.edges[&node].iter().filter(|(_, v)| **v > 0) {
            self.dfs_exit_times(neigh, time, outtime);
        }
        outtime.insert(node, *time);
        *time += 1;
    }

    fn part1(&self) -> u64 {
        let you: NodeId = "you".parse().unwrap();
        let out: NodeId = "out".parse().unwrap();
        let mut map = HashMap::new();
        self.dfs_exit_times(you, &mut 0, &mut map);
        let mut times: Vec<_> = map.into_iter().collect();
        times.sort_unstable_by_key(|&(_, time)| Reverse(time));
        let mut possibilities = HashMap::from([(you, 1)]);
        for (node, _) in times {
            for (&neigh, _) in self.edges[&node].iter().filter(|(_, v)| **v > 0) {
                *possibilities.entry(neigh).or_insert(0) += possibilities[&node];
            }
        }
        possibilities[&out]
    }
}

fn main() {
    let mut graph = parse();
    println!("{}", graph.part1());
}

fn parse() -> Graph {
    let data = std::fs::read_to_string("input/11.txt").unwrap();
    let mut graph = Graph::new();
    for line in data.lines() {
        let (from, tos) = line.split_once(": ").unwrap();
        for to in tos.split_ascii_whitespace() {
            graph.add_edge(from, to);
        }
    }
    graph
}
