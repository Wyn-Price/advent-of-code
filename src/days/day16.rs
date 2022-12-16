use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, HashSet},
};

use petgraph::{algo::dijkstra, data::Build, stable_graph::NodeIndex, Graph};

pub fn part_a(input: &str) -> i64 {
    let (mut map, graph) = parse(input);
    // let best = find_best(-1, 1, "AA".to_owned(), 0, HashSet::new(), &mut map);
    // best

    let mut vec = map.values_mut().collect::<Vec<_>>();

    for i in 0..vec.len() {
        let v = &vec[i];
        let node_map = dijkstra(&graph, v.node, None, |_| 1);

        let mut time_to_nodes = vec![];
        for v in &vec {
            if v.rate != 0 {
                let a: &Valve = v;
                time_to_nodes.push((a.key.clone(), *node_map.get(&v.node).unwrap() as i64));
            }
        }
        let mut a = vec.get_mut(i).unwrap();
        a.time_to_nodes = time_to_nodes;
    }

    find_best(1, "AA".to_owned(), 0, 0, HashSet::new(), &map, &graph)
}

fn find_best(
    time: i64,
    game_current: String,
    game_total_rate: i64,
    game_score: i64,
    game_open_valves: HashSet<String>,
    valves: &HashMap<String, Valve>,
    graph: &Graph<String, ()>,
) -> i64 {
    let current = valves.get(&game_current).unwrap();

    current
        .time_to_nodes
        .iter()
        .filter_map(|(valve_key, time_taken)| {
            if game_open_valves.contains(valve_key) {
                None
            } else {
                Some((valves.get(valve_key)?, *time_taken))
            }
        })
        .map(|(valve, mut time_taken)| {
            if time + time_taken >= 30 {
                time_taken = 30 - time - 1;
            }

            let new_game_time = time + time_taken + 1;
            let new_game_score = game_score + (time_taken + 1) * game_total_rate;

            if time == 30 {
                return new_game_score;
            }

            let new_game_current = valve.key.clone();

            let mut new_game_open_valves = game_open_valves.clone();
            new_game_open_valves.insert(valve.key.clone());

            let new_game_total_rate = game_total_rate + valve.rate as i64;

            // println!("{valve:?} {time_taken}");
            // new_game_score

            find_best(
                new_game_time,
                new_game_current,
                new_game_total_rate,
                new_game_score,
                new_game_open_valves,
                valves,
                graph,
            )
        })
        .max()
        .unwrap_or_else(|| game_score + (31 - time) * game_total_rate)
}

pub fn part_b(input: &str) -> i64 {
    parse(input);
    panic!("Part B not implimented yet");
}

fn parse(input: &str) -> (HashMap<String, Valve>, Graph<String, ()>) {
    let mut graph = Graph::new();
    let map: HashMap<String, Valve> = input
        .lines()
        .map(|l| {
            let (key, rate, t, l, v, leading_to) = scan_fmt!(
                l,
                r"Valve {} has flow rate={d}; {} {} to {} {/.+/}",
                String,
                u8,
                String,
                String,
                String,
                String
            )
            .unwrap();

            let node = graph.add_node(key.clone());

            (
                key.clone(),
                Valve {
                    key,
                    node,
                    rate,
                    leads_to: leading_to.split(", ").map(|s| s.to_owned()).collect(),
                    time_to_nodes: vec![],
                },
            )
        })
        .collect();

    map.values().for_each(|v| {
        v.leads_to
            .iter()
            .map(|l| map.get(l).unwrap().node)
            .for_each(|other| {
                graph.add_edge(v.node, other, ());
            })
    });

    (map, graph)
}
#[derive(Debug)]
struct Valve {
    key: String,
    node: NodeIndex,
    rate: u8,
    leads_to: Vec<String>,
    time_to_nodes: Vec<(String, i64)>,
}
