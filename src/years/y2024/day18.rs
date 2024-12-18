use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::{algo::dijkstra, Graph};

pub fn part_a(input: &str) -> i64 {
    let all_bytes = parse(input);
    let bytes = all_bytes[0..1024].into_iter().collect::<HashSet<_>>();
    let size = if bytes.len() == 25 { 6 } else { 70 };
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();
    for x in 0..=size {
        for y in 0..=size {
            if bytes.contains(&(x, y)) {
                continue;
            }
            let node = *nodes
                .entry((x, y))
                .or_insert_with(|| graph.add_node((x, y)));
            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (x + dx, y + dy);
                if bytes.contains(&np) {
                    continue;
                }
                let other_node = *nodes.entry(np).or_insert_with(|| graph.add_node(np));
                graph.add_edge(node, other_node, ());
            }
        }
    }

    let dijkstra = dijkstra(&graph, nodes[&(0, 0)], Some(nodes[&(size, size)]), |_| 1);
    return dijkstra[&nodes[&(size, size)]] as i64;
}

pub fn part_b(input: &str) -> String {
    let all_bytes = parse(input);
    let size = if all_bytes.len() == 25 { 6 } else { 70 };
    let amount = if all_bytes.len() == 25 { 12 } else { 1024 };
    for i in amount..all_bytes.len() {
        let bytes = all_bytes[0..=i].into_iter().collect::<HashSet<_>>();
        let mut graph = Graph::new_undirected();
        let mut nodes = HashMap::new();
        for x in 0..=size {
            for y in 0..=size {
                if bytes.contains(&(x, y)) {
                    continue;
                }
                let node = *nodes
                    .entry((x, y))
                    .or_insert_with(|| graph.add_node((x, y)));
                for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let np = (x + dx, y + dy);
                    if bytes.contains(&np) {
                        continue;
                    }
                    let other_node = *nodes.entry(np).or_insert_with(|| graph.add_node(np));
                    graph.add_edge(node, other_node, ());
                }
            }
        }

        let dijkstra = dijkstra(&graph, nodes[&(0, 0)], Some(nodes[&(size, size)]), |_| 1);
        let byte = all_bytes[i];
        println!("{} {:?} {:?}", i, byte, dijkstra.get(&nodes[&(size, size)]));
        if !dijkstra.contains_key(&nodes[&(size, size)]) {
            return format!("{},{}", byte.0, byte.1);
        }
    }

    panic!("None?")
}

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}
