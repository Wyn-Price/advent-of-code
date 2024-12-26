use core::panic;
use std::collections::HashMap;

use petgraph::{Graph, Undirected};
use rand::{rngs::StdRng, Rng, SeedableRng};

type Board = Graph<String, (), Undirected>;

pub fn part_a(input: &str) -> i64 {
    let mut graph = parse(input);

    graph = remove_most_used(graph);
    graph = remove_most_used(graph);
    graph = remove_most_used(graph);

    let a = graph.node_indices().nth(1).unwrap();
    for bi in 1..graph.node_count() {
        let b = graph.node_indices().nth(bi).unwrap();
        // See if they're disconnected
        let res = petgraph::algo::astar(&graph, a, |n| n == b, |_| 0, |_| 0);
        if res.is_none() {
            // Idk if dijkstra is the nicest
            let ca = petgraph::algo::dijkstra(&graph, a, None, |_| 0).len();
            let cb = petgraph::algo::dijkstra(&graph, b, None, |_| 0).len();
            return ca as i64 * cb as i64;
        }
    }

    panic!("All nodes still connected?");
}

fn remove_most_used(mut graph: Board) -> Board {
    let mut seen = HashMap::new();

    // Generate some random paths and look at which edges were most used
    let mut r = StdRng::seed_from_u64(42);

    (0..500)
        .map(|_| {
            let ai = r.gen_range(0..graph.node_count());
            let bi = r.gen_range(0..graph.node_count());

            let a = graph.node_indices().nth(ai).unwrap();
            let b = graph.node_indices().nth(bi).unwrap();
            return (a, b);
        })
        .filter(|(a, b)| a != b)
        .for_each(|(a, b)| {
            let (_, path) = petgraph::algo::astar(&graph, a, |n| n == b, |_| 0, |_| 0).unwrap();
            for window in path.windows(2) {
                let edge = graph.find_edge(window[0], window[1]).unwrap();
                seen.entry(edge).and_modify(|v| *v += 1).or_insert(1);
            }
        });

    let top = seen.keys().max_by_key(|&k| seen.get(k).unwrap()).unwrap();
    graph.remove_edge(*top);
    return graph;
}

fn parse(input: &str) -> Board {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (key, value) = line.split_once(": ").unwrap();
        let kn = *nodes
            .entry(key)
            .or_insert_with(|| graph.add_node(key.to_owned()));
        for val in value.split_ascii_whitespace() {
            let vn = *nodes
                .entry(val)
                .or_insert_with(|| graph.add_node(val.to_owned()));

            graph.add_edge(kn, vn, ());
        }
    }
    graph
}
