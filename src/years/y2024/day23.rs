use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::{Graph, Undirected};

pub fn part_a(input: &str) -> usize {
    let (graph, nodes) = parse(input);

    let mut threes = HashSet::new();
    // let res = floyd_warshall(&graph, |_| 1).unwrap();
    // dbg!(&res);
    for key in nodes.keys() {
        let mut potential_3s = vec![];
        for other in nodes.keys() {
            if key == other {
                continue;
            }

            if graph.find_edge(nodes[key], nodes[other]).is_some()
                || graph.find_edge(nodes[other], nodes[key]).is_some()
            {
                potential_3s.push(other);
            }

            // if res[&(nodes[key], nodes[other])] == 1 {
            //     potential_3s.push(other);
            // }
        }
        for &p3a in &potential_3s {
            for &p3b in &potential_3s {
                // if res[&(nodes[p3a], nodes[p3b])] == 1 {
                if graph.find_edge(nodes[p3a], nodes[p3b]).is_some()
                    || graph.find_edge(nodes[p3b], nodes[p3a]).is_some()
                {
                    let mut v = vec![key, p3a, p3b];
                    v.sort();
                    threes.insert((v[0], v[1], v[2]));
                }
            }
        }
    }

    dbg!(&threes);
    threes
        .into_iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

pub fn part_b(input: &str) -> String {
    let (graph, _) = parse(input);

    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut x = HashSet::new();

    bron_kerbosch(&graph, &mut r, &mut p, &mut x, &mut cliques);

    cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .into_iter()
        .map(|k| graph[k].clone())
        .sorted()
        .join(",")
}

// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
//
// All verticies in R
// Some verticies in P
// No verticies in X
// Neibour set N(v)
fn bron_kerbosch(
    graph: &ComGraph,
    r: &mut HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    cliques: &mut Vec<HashSet<NodeIndex>>,
) {
    // if P and X are both empty then
    if p.is_empty() && x.is_empty() {
        // report R as a maximal clique
        cliques.push(r.clone());
        return;
    }

    //     for each vertex v in P do
    //         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    //         P := P \ {v}
    //         X := X ⋃ {v}
    for v in p.clone() {
        // R ⋃ {v}
        let mut r_new = r.clone();
        r_new.insert(v);

        // P ⋂ N(v)
        let mut p_new = p.clone();
        p_new.retain(|&node| graph.contains_edge(v, node));

        // X ⋂ N(v)
        let mut x_new = x.clone();
        x_new.retain(|&node| graph.contains_edge(v, node));

        bron_kerbosch(graph, &mut r_new, &mut p_new, &mut x_new, cliques);

        // P := P \ {v}
        // X := X ⋃ {v}
        p.remove(&v);
        x.insert(v);
    }
}

type ComGraph = Graph<String, (), Undirected>;
type ComNodes = HashMap<String, petgraph::prelude::NodeIndex>;
type NodeIndex = petgraph::prelude::NodeIndex;

fn parse(input: &str) -> (ComGraph, ComNodes) {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();

    input.lines().for_each(|l| {
        let (left, right) = l.split_once("-").unwrap();

        let ln = *nodes
            .entry(left.to_owned())
            .or_insert_with(|| graph.add_node(left.to_owned()));

        let rn = *nodes
            .entry(right.to_owned())
            .or_insert_with(|| graph.add_node(right.to_owned()));

        graph.add_edge(ln, rn, ());
    });

    return (graph, nodes);
}
